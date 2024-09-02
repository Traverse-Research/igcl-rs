#![doc = include_str!("../README.md")]

use std::mem::MaybeUninit;

use anyhow::Result;
use device_adapter::DeviceAdapter;
use ffi::{ctl_application_id_t, ctl_device_adapter_properties_t, ctl_init_args_t};

use crate::{
    error::Error,
    ffi::{ctl_api_handle_t, ControlLib},
};

#[allow(nonstandard_style)]
pub mod ffi;

pub mod device_adapter;
pub mod error;

pub struct Igcl {
    api_handle: ctl_api_handle_t,
    control_lib: ControlLib,
}

impl Igcl {
    /// Create a new instance of [`Igcl`].
    /// This loads the required dll, and initializes the Igcl library.
    #[doc(alias = "ctlInit")]
    pub fn new() -> Result<Self> {
        let control_lib = unsafe { ControlLib::new("ControlLib")? };

        let api_handle = {
            let mut init_args = ctl_init_args_t {
                Size: std::mem::size_of::<ctl_init_args_t>() as u32,
                Version: 0,
                AppVersion: 0,
                flags: 0,
                SupportedVersion: 0,
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
            let mut api_handle = MaybeUninit::uninit();

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
        let mut num_adapters = MaybeUninit::uninit();

        let mut num_adapters = Error::from_result_with_assume_init_on_success(
            unsafe {
                self.control_lib.ctlEnumerateDevices(
                    self.api_handle,
                    num_adapters.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            },
            num_adapters,
        )?;

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

        for idx in 0..(num_adapters as usize) {
            let device_adapter_handle = adapters[idx];
            let mut adapter_properties: ctl_device_adapter_properties_t =
                unsafe { MaybeUninit::zeroed().assume_init() };

            adapter_properties.Size = std::mem::size_of::<ctl_device_adapter_properties_t>() as u32;
            adapter_properties.pDeviceID = std::ptr::null_mut();
            adapter_properties.device_id_size = 0;

            // First query the size of the id.
            Error::from_result(unsafe {
                self.control_lib
                    .ctlGetDeviceProperties(device_adapter_handle, &mut adapter_properties)
            })?;

            let mut device_id = vec![0u8; adapter_properties.device_id_size as usize];
            adapter_properties.pDeviceID = device_id.as_mut_ptr() as *mut _;

            // Then query the actual ID.
            Error::from_result(unsafe {
                self.control_lib.ctlGetDeviceProperties(
                    device_adapter_handle,
                    &mut adapter_properties
                )
            })?;

            devices.push(DeviceAdapter {
                device_adapter_handle,
                adapter_properties,
                device_id,
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
