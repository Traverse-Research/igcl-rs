use std::{ffi::CStr, mem::MaybeUninit};

use crate::{
    error::Error,
    ffi::{ctl_api_handle_t, ctl_init_args_t, ctl_result_t},
};

use anyhow::{ensure, Context, Result};

pub type IgclInitializeFn = ::std::option::Option<
    unsafe extern "C" fn(
        pInitDesc: *mut ctl_init_args_t,
        phAPIHandle: *mut ctl_api_handle_t,
    ) -> ctl_result_t,
>;

pub type IgclCloseFn =
    ::std::option::Option<unsafe extern "C" fn(hAPIHandle: ctl_api_handle_t) -> ctl_result_t>;

// Note: only 64 bit windows.
static IGCL_DLL_NAME: &'static str = "ControlLib.dll";
static IGCL_INIT_FUNCTION_NAME: &'static str = "ctlInit";
static IGCL_CLOSE_FUNCTION_NAME: &'static str = "ctlClose";

/// Basic functions for library setup.
struct IgclFunctions {
    _lib: libloading::Library,
    init_fn: IgclInitializeFn,
    close_fn: IgclCloseFn,
}

impl IgclFunctions {
    unsafe fn load() -> anyhow::Result<Self> {
        let dll_name = CStr::from_bytes_with_nul(IGCL_DLL_NAME.as_bytes())
            .unwrap()
            .to_str()
            .unwrap();
        let lib = libloading::Library::new(dll_name)
            .with_context(|| format!("Failed to load `{dll_name}`"))?;

        fn load_symbol<T: Copy>(lib: &libloading::Library, name: &[u8]) -> Result<Option<T>> {
            let name_c = CStr::from_bytes_with_nul(name)?;
            let sym: Option<T> = *unsafe { lib.get(name) }
                .with_context(|| format!("Failed to get function symbol {name_c:?}"))?;
            // Keep the symbol wrapped in an `Option`, as that is what `bindgen` generates
            ensure!(sym.is_some(), "{name_c:?} cannot be NULL");
            Ok(sym)
        }

        let init_fn: IgclInitializeFn = load_symbol(&lib, IGCL_INIT_FUNCTION_NAME.as_bytes())?;
        let close_fn: IgclCloseFn = load_symbol(&lib, IGCL_CLOSE_FUNCTION_NAME.as_bytes())?;

        Ok(Self {
            _lib: lib,
            init_fn,
            close_fn,
        })
    }
}

pub struct IgclHelper {
    api_handle: ctl_api_handle_t,
    functions: IgclFunctions,
}

impl IgclHelper {
    pub fn new() -> Result<Self> {
        let functions = unsafe { IgclFunctions::load()? };

        let api_handle = unsafe {
            // Pointer to init args struct.
            let mut init_args = MaybeUninit::uninit();
            // Pointer to a pointer to an API handle.
            let mut api_handle = MaybeUninit::uninit();

            Error::from_result_with_assume_init_on_success(
                unsafe {
                    (functions.init_fn.unwrap())(init_args.as_mut_ptr(), api_handle.as_mut_ptr())
                },
                api_handle,
            )?
        };

        Ok(Self {
            api_handle,
            functions,
        })
    }

    pub fn api_handle(&self) -> ctl_api_handle_t {
        self.api_handle
    }
}
