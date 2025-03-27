#![doc = include_str!("../README.md")]

use std::{mem::MaybeUninit, sync::Arc};

use anyhow::Result;
use device_adapter::DeviceAdapter;
use ffi::{
    ctl_application_id_t, ctl_device_adapter_properties_t, ctl_init_args_t,
    ctl_init_flag_t::CTL_INIT_FLAG_USE_LEVEL_ZERO, CTL_IMPL_MAJOR_VERSION, CTL_IMPL_MINOR_VERSION,
};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::LUID;

use crate::{
    error::Error,
    ffi::{ctl_api_handle_t, ControlLib},
};

#[allow(clippy::missing_safety_doc)]
#[allow(nonstandard_style)]
pub mod ffi;

pub mod device_adapter;
pub mod error;
pub mod memory;

pub struct Igcl {
    api_handle: ctl_api_handle_t,
    control_lib: Arc<ControlLib>,
}

#[doc(alias = "CTL_MAKE_VERSION")]
fn ctl_make_version(major: u32, minor: u32) -> u32 {
    (major << 16) | (minor & 0x0000ffff)
}

impl Igcl {
    /// Create a new instance of [`Igcl`].
    /// This loads the required dll, and initializes the Igcl library.
    #[doc(alias = "ctlInit")]
    pub fn new() -> Result<Self> {
        let control_lib = Arc::new(unsafe { ControlLib::new("ControlLib")? });

        let api_handle = {
            let mut init_args = ctl_init_args_t {
                Size: std::mem::size_of::<ctl_init_args_t>() as u32,
                Version: 0,
                AppVersion: ctl_make_version(CTL_IMPL_MAJOR_VERSION, CTL_IMPL_MINOR_VERSION),
                flags: 0,
                SupportedVersion: ctl_make_version(CTL_IMPL_MAJOR_VERSION, CTL_IMPL_MINOR_VERSION),
                // According to the igcl documentation (https://intel.github.io/drivers.gpu.control-library/Control/api.html#ctl-init-args-t),
                // this can be all zeroes.
                ApplicationUID: ctl_application_id_t {
                    Data1: 0,
                    Data2: 0,
                    Data3: 0,
                    Data4: [0; 8],
                },
            };

            // Pointer to an API handle.
            let mut api_handle = MaybeUninit::zeroed();

            Error::from_result_with_assume_init_on_success(
                unsafe { control_lib.ctlInit(&mut init_args, api_handle.as_mut_ptr()) },
                api_handle,
            )?
        };

        Ok(Self {
            api_handle,
            control_lib,
        })
    }

    /// Enumerate all available physical devices.
    #[doc(alias = "ctlEnumerateDevices")]
    pub fn enumerate_devices(&self) -> Result<Vec<DeviceAdapter>> {
        // Note(Jan): this MUST be zero, otherwise the api does not write the correct value away.
        // The docs seem to also be wrong, because large values do not get truncated.
        let mut num_adapters = 0u32;

        Error::from_result(unsafe {
            self.control_lib.ctlEnumerateDevices(
                self.api_handle,
                &mut num_adapters,
                std::ptr::null_mut(),
            )
        })?;

        let mut adapters = Vec::with_capacity(num_adapters as usize);

        Error::from_result(unsafe {
            self.control_lib.ctlEnumerateDevices(
                self.api_handle,
                &mut num_adapters,
                adapters.as_mut_ptr(),
            )
        })?;

        unsafe { adapters.set_len(num_adapters as usize) };

        let mut devices = vec![];

        for device_adapter_handle in adapters.into_iter() {
            let mut adapter_properties: ctl_device_adapter_properties_t =
                unsafe { MaybeUninit::zeroed().assume_init() };

            adapter_properties.Size = std::mem::size_of::<ctl_device_adapter_properties_t>() as u32;
            adapter_properties.Version = 0;

            // LUID is only available on windows.
            #[cfg(target_os = "windows")]
            let device_id = {
                let luid_size = std::mem::size_of::<LUID>();
                let mut device_id = vec![0u8; luid_size];
                adapter_properties.device_id_size = luid_size as u32;
                adapter_properties.pDeviceID = device_id.as_mut_ptr() as *mut _;
                device_id
            };

            #[cfg(not(target_os = "windows"))]
            let device_id = vec![];

            Error::from_result(unsafe {
                self.control_lib
                    .ctlGetDeviceProperties(device_adapter_handle, &mut adapter_properties)
            })?;

            devices.push(DeviceAdapter {
                device_adapter_handle,
                adapter_properties,
                device_id,
                control_lib: self.control_lib.clone(),
            })
        }

        Ok(devices)
    }
}

impl Drop for Igcl {
    fn drop(&mut self) {
        if let Err(error) =
            Error::from_result(unsafe { self.control_lib.ctlClose(self.api_handle) })
        {
            eprintln!("Igcl close failed with {error:?}");
        }
    }
}
