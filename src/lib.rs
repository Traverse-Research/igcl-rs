#![doc = include_str!("../README.md")]

use std::mem::MaybeUninit;

use anyhow::Result;
use error::Error;
use ffi::{
    ctlEnumerateDevices, ctlGetDeviceProperties, ctl_adapter_bdf_t, ctl_device_adapter_handle_t,
    ctl_device_adapter_properties_t,
};
use helper::IgclHelper;
use std::mem::MaybeUninit;

#[allow(nonstandard_style)]
pub mod ffi;

pub mod error;
pub mod helper;

pub struct DeviceAdapter {
    device_adapter_handle: ctl_device_adapter_handle_t,
    /// Note: the pointer to the device ID is invalid at this point and should not be used.
    /// Use [`Self::device_id`] instead.
    adapter_properties: ctl_device_adapter_properties_t,
    device_id: Vec<u8>,
}

impl DeviceAdapter {
    pub fn device_adapter_handle(&self) -> ctl_device_adapter_handle_t {
        self.device_adapter_handle
    }

    /// Retrieve the unique device identifier, determined by the operating system.
    /// On windows, this will be the LUID.
    pub fn device_id(&self) -> &[u8] {
        &self.device_id
    }

    /// Retrieve the PCI Bus ID, the PCI Device ID, and the PCI Device Function in that order.
    pub fn bus_device_function(&self) -> (u8, u8, u8) {
        let ctl_adapter_bdf_t {
            bus,
            device,
            function,
        } = self.adapter_properties.adapter_bdf;

        (bus, device, function)
    }

    pub fn name(&self) -> &[i8] {
        &self.adapter_properties.name
    }

    pub fn pci_vendor_id(&self) -> u32 {
        self.adapter_properties.pci_vendor_id
    }

    pub fn pci_device_id(&self) -> u32 {
        self.adapter_properties.pci_device_id
    }

    pub fn pci_subsys_vendor_id(&self) -> u16 {
        self.adapter_properties.pci_subsys_vendor_id
    }

    pub fn pci_subsys_id(&self) -> u16 {
        self.adapter_properties.pci_subsys_id
    }

    pub fn device_type(&self) -> i32 {
        self.adapter_properties.device_type
    }
}

pub struct Igcl {
    igcl_helper: IgclHelper,
}

impl Igcl {
    /// Try to initialize a new instance of IGCL.
    pub fn new() -> Result<Self> {
        let igcl_helper = IgclHelper::new()?;
        Ok(Self { igcl_helper })
    }

    /// Enumerate GPUs available to IGCL.
    pub fn devices(&self) -> Result<Vec<DeviceAdapter>> {
        let mut adapter_handle = MaybeUninit::zeroed();
        let mut num_adapters = MaybeUninit::zeroed();

        let api_handle = self.igcl_helper.api_handle();

        Error::from_result(unsafe {
            ctlEnumerateDevices(
                api_handle,
                num_adapters.as_mut_ptr(),
                adapter_handle.as_mut_ptr(),
            )
        })?;

        let num_adapters = unsafe { num_adapters.assume_init() } as usize;
        let mut adapters = vec![MaybeUninit::zeroed(); num_adapters];

        Error::from_result(unsafe {
            ctlEnumerateDevices(
                api_handle,
                num_adapters as *mut _,
                adapters[0].as_mut_ptr() as *mut _,
            )
        })?;

        let mut devices = vec![];

        for idx in 0..num_adapters {
            let device_adapter_handle = unsafe { adapters[idx].assume_init() };
            let mut adapter_properties: ctl_device_adapter_properties_t =
                unsafe { MaybeUninit::zeroed().assume_init() };

            adapter_properties.Size = std::mem::size_of::<ctl_device_adapter_properties_t>() as u32;

            // First query the size of the id.
            Error::from_result(unsafe {
                ctlGetDeviceProperties(device_adapter_handle, (&mut adapter_properties) as *mut _)
            })?;

            let mut device_id = vec![0u8; adapter_properties.device_id_size as usize];
            adapter_properties.pDeviceID = device_id.as_mut_ptr() as *mut _;

            // Then query the actual ID.
            Error::from_result(unsafe {
                ctlGetDeviceProperties(device_adapter_handle, (&mut adapter_properties) as *mut _)
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
