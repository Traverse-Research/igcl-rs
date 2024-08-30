use std::mem::MaybeUninit;

use crate::{
    error::Error,
    ffi::{ctl_api_handle_t, ControlLib},
};

use anyhow::Result;

pub struct IgclHelper {
    api_handle: ctl_api_handle_t,
    control_lib: ControlLib,
}

impl IgclHelper {
    pub fn new() -> Result<Self> {
        let control_lib = unsafe { ControlLib::new("ControlLib")? };

        let api_handle = unsafe {
            // Pointer to init args struct.
            let mut init_args = MaybeUninit::uninit();
            // Pointer to a pointer to an API handle.
            let mut api_handle = MaybeUninit::uninit();

            Error::from_result_with_assume_init_on_success(
                unsafe { control_lib.ctlInit(init_args.as_mut_ptr(), api_handle.as_mut_ptr()) },
                api_handle,
            )?
        };

        Ok(Self {
            api_handle,
            control_lib,
        })
    }

    pub fn a(&self) -> Result<()> {
        let ret = MaybeUninit::uninit();

        Error::from_result_with_assume_init_on_success(
            unsafe { self.control_lib.ctlClose(self.api_handle) },
            ret,
        )?
    }
}

impl Drop for IgclHelper {
    fn drop(&mut self) {
        if let Err(error) =
            Error::from_result(unsafe { self.control_lib.ctlClose(self.api_handle) })
        {
            eprintln!("Igcl close failed with {error:?}");
        }
    }
}
