use std::sync::Arc;

use crate::{
    error::Error,
    ffi::{ctl_mem_handle_t, ctl_mem_state_t, ControlLib},
};
use anyhow::Result;

#[doc(alias = "ctl_mem_state_t")]
pub struct MemoryState {
    pub total: u64,
    pub free: u64,
}

impl MemoryState {
    pub fn used(&self) -> u64 {
        self.total - self.free
    }
}

#[doc(alias = "ctl_mem_handle_t")]
pub struct MemoryModule {
    pub(crate) control_lib: Arc<ControlLib>,
    pub(crate) memory_module_handle: ctl_mem_handle_t,
}

impl MemoryModule {
    #[doc(alias = "ctlMemoryGetState")]
    pub fn memory_state(&self) -> Result<MemoryState> {
        let mut state = ctl_mem_state_t {
            Size: std::mem::size_of::<ctl_mem_state_t>() as u32,
            Version: 0,
            ..Default::default()
        };
        Error::from_result(unsafe {
            self.control_lib
                .ctlMemoryGetState(self.memory_module_handle, &mut state)
        })?;

        Ok(MemoryState {
            total: state.size,
            free: state.free,
        })
    }
}
