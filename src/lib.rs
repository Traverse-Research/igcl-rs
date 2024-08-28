#![doc = include_str!("../README.md")]

use std::mem::MaybeUninit;

#[allow(nonstandard_style)]
pub mod ffi;

use error::{Error, Result};
use ffi::{ctlEnumerateDevices, ctlInit, ctl_api_handle_t, ctl_device_adapter_handle_t};

pub mod error;

pub struct Igcl {
    api_handle: ctl_api_handle_t,
}

impl Igcl {
    /// Try to initialize a new instance of IGCL.
    pub fn new() -> Result<Self> {
        // Pointer to init args struct.
        let mut init_args = MaybeUninit::uninit();
        // Pointer to a pointer to an API handle.
        let mut api_handle = MaybeUninit::uninit();

        let api_handle = Error::from_result_with_assume_init_on_success(
            unsafe { ctlInit(init_args.as_mut_ptr(), api_handle.as_mut_ptr()) },
            api_handle,
        )?;

        Ok(Self { api_handle })
    }

    /// Enumerate GPUs available to IGCL.
    pub fn enumerate_adapters(&self) -> Result<&[ctl_device_adapter_handle_t]> {
        let mut adapter_handle = MaybeUninit::zeroed();
        let mut num_adapters = MaybeUninit::zeroed();

        Error::from_result(unsafe {
            ctlEnumerateDevices(
                self.api_handle,
                num_adapters.as_mut_ptr(),
                adapter_handle.as_mut_ptr(),
            )
        })?;

        let num_adapters = unsafe { num_adapters.assume_init() };
        let mut adapters = vec![MaybeUninit::zeroed(); num_adapters as usize];

        Error::from_result(unsafe {
            ctlEnumerateDevices(
                self.api_handle,
                num_adapters as *mut _,
                adapters[0].as_mut_ptr() as *mut _,
            )
        })?;

        todo!()
    }
}
