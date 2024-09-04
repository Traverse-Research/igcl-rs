use std::os::raw::c_void;
use std::{ffi::OsStr, sync::Arc};

use crate::error::Result;

use crate::{
    error::Error,
    ffi::{
        ctl_3d_feature_getset_t, ctl_3d_feature_t, ctl_adapter_bdf_t, ctl_device_adapter_handle_t,
        ctl_device_adapter_properties_t, ctl_device_type_t, ctl_gaming_flip_mode_flag_t,
        ctl_property_value_type_t, ControlLib, _ctl_result_t, ctl_endurance_gaming_t, ctl_result_t,
    },
};

/// Specifies the scope in which to query for driver settings.
/// Note that IGCL will not fall back to a wider scope when settings haven't been specified for the current one.
/// This means it may be necessary to query for both the global and process scopes to retrieve a valid driver setting.
pub enum DriverSettingScope<'a> {
    /// Read the global settings.
    Global,
    /// Dynamically detect the current process.
    CurrentProcess,
    /// A specific process with file extension, e.g. `hl2.exe`.
    Process { process_name: &'a str },
}

impl DriverSettingScope<'_> {
    pub fn name(&self) -> String {
        match self {
            DriverSettingScope::Global => String::default(),
            DriverSettingScope::CurrentProcess => {
                std::env::current_exe().map_or("".to_string(), |path| {
                    path.file_name()
                        .unwrap_or(OsStr::new(""))
                        .to_string_lossy()
                        .to_string()
                })
            }
            DriverSettingScope::Process { process_name } => process_name.to_string(),
        }
    }

    /// Get the scope that
    pub fn fall_back_to_higher_scope(&self) -> Option<Self> {
        match self {
            Self::Global => None,
            _ => Some(Self::Global),
        }
    }
}

#[doc(alias = "ctl_device_adapter_handle_t")]
pub struct DeviceAdapter {
    pub(crate) device_adapter_handle: ctl_device_adapter_handle_t,
    /// Note: the pointer to the device ID is invalid at this point and should not be used.
    /// Use [`Self::device_id`] instead.
    pub(crate) adapter_properties: ctl_device_adapter_properties_t,
    pub(crate) device_id: Vec<u8>,
    pub(crate) control_lib: Arc<ControlLib>,
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

    pub fn device_type(&self) -> ctl_device_type_t {
        self.adapter_properties.device_type
    }

    /// Attempt to query the endurance gaming driver setting for the specified scope.
    /// Falls back to a higher scope if the setting could not be found in the current one.
    pub fn feature_endurance_gaming(
        &self,
        scope: DriverSettingScope<'_>,
    ) -> Result<Box<ctl_endurance_gaming_t>> {
        let mut result = ctl_result_t::CTL_RESULT_ERROR_UNKNOWN;
        let mut scope = Some(scope);
        let mut settings: Box<ctl_endurance_gaming_t> = Box::new(unsafe { std::mem::zeroed() });

        while let Some(driver_setting_scope) = scope.take() {
            let mut current_app = driver_setting_scope.name();
            let reference = settings.as_mut();
            let ptr = reference as *mut _ as *mut c_void;

            let mut feature = ctl_3d_feature_getset_t {
                Size: std::mem::size_of::<ctl_3d_feature_getset_t>() as u32,
                Version: 0,
                FeatureType: ctl_3d_feature_t::CTL_3D_FEATURE_ENDURANCE_GAMING,
                ApplicationName: current_app.as_mut_ptr() as *mut _,
                ApplicationNameLength: current_app.as_bytes().len() as i8,
                bSet: false,
                ValueType: ctl_property_value_type_t::CTL_PROPERTY_VALUE_TYPE_CUSTOM,
                Value: unsafe { std::mem::zeroed() },
                CustomValueSize: std::mem::size_of::<ctl_endurance_gaming_t>() as i32,
                pCustomValue: ptr,
            };

            result = unsafe {
                self.control_lib
                    .ctlGetSet3DFeature(self.device_adapter_handle, &mut feature)
            };

            if result != ctl_result_t::CTL_RESULT_SUCCESS {
                scope = driver_setting_scope.fall_back_to_higher_scope();
            }
        }

        Error::from_result(result)?;
        Ok(settings)
    }

    /// Attempt to query the frame rate limit driver setting.
    /// Falls back to a higher scope if the setting could not be found in the current one.
    pub fn feature_frame_limit(&self, scope: DriverSettingScope<'_>) -> Result<i32> {
        let mut result = ctl_result_t::CTL_RESULT_ERROR_UNKNOWN;
        let mut scope = Some(scope);
        let mut frame_rate_limit = 0i32;

        while let Some(driver_setting_scope) = scope.take() {
            let mut current_app = driver_setting_scope.name();

            let mut feature = ctl_3d_feature_getset_t {
                Size: std::mem::size_of::<ctl_3d_feature_getset_t>() as u32,
                Version: 0,
                FeatureType: ctl_3d_feature_t::CTL_3D_FEATURE_FRAME_LIMIT,
                ApplicationName: current_app.as_mut_ptr() as *mut _,
                ApplicationNameLength: current_app.as_bytes().len() as i8,
                bSet: false,
                ValueType: ctl_property_value_type_t::CTL_PROPERTY_VALUE_TYPE_ENUM,
                Value: unsafe { std::mem::zeroed() },
                CustomValueSize: 0,
                pCustomValue: std::ptr::null_mut(),
            };

            result = unsafe {
                self.control_lib
                    .ctlGetSet3DFeature(self.device_adapter_handle, &mut feature)
            };

            if result != ctl_result_t::CTL_RESULT_SUCCESS {
                scope = driver_setting_scope.fall_back_to_higher_scope();
            } else {
                frame_rate_limit = unsafe { feature.Value.IntType.Value };
            }
        }

        Error::from_result(result)?;
        Ok(frame_rate_limit)
    }

    /// Attempt to query the flip mode driver setting.
    /// Falls back to a higher scope if the setting could not be found in the current one.
    pub fn feature_flip_mode(
        &self,
        scope: DriverSettingScope<'_>,
    ) -> Result<ctl_gaming_flip_mode_flag_t> {
        let mut result = ctl_result_t::CTL_RESULT_ERROR_UNKNOWN;
        let mut scope = Some(scope);

        let mut flip_mode = 0;

        while let Some(driver_setting_scope) = scope.take() {
            let mut current_app = driver_setting_scope.name();

            let mut feature = ctl_3d_feature_getset_t {
                Size: std::mem::size_of::<ctl_3d_feature_getset_t>() as u32,
                Version: 0,
                FeatureType: ctl_3d_feature_t::CTL_3D_FEATURE_GAMING_FLIP_MODES,
                ApplicationName: current_app.as_mut_ptr() as *mut _,
                ApplicationNameLength: current_app.as_bytes().len() as i8,
                bSet: false,
                ValueType: ctl_property_value_type_t::CTL_PROPERTY_VALUE_TYPE_ENUM,
                Value: unsafe { std::mem::zeroed() },
                CustomValueSize: 0,
                pCustomValue: std::ptr::null_mut(),
            };

            result = unsafe {
                self.control_lib
                    .ctlGetSet3DFeature(self.device_adapter_handle, &mut feature)
            };

            if result != ctl_result_t::CTL_RESULT_SUCCESS {
                scope = driver_setting_scope.fall_back_to_higher_scope();
            } else {
                flip_mode = unsafe { feature.Value.EnumType.EnableType };
            }
        }

        Error::from_result(result)?;

        let flip_mode = match flip_mode {
            1 => ctl_gaming_flip_mode_flag_t::CTL_GAMING_FLIP_MODE_FLAG_APPLICATION_DEFAULT,
            2 => ctl_gaming_flip_mode_flag_t::CTL_GAMING_FLIP_MODE_FLAG_VSYNC_OFF,
            4 => ctl_gaming_flip_mode_flag_t::CTL_GAMING_FLIP_MODE_FLAG_VSYNC_ON,
            8 => ctl_gaming_flip_mode_flag_t::CTL_GAMING_FLIP_MODE_FLAG_SMOOTH_SYNC,
            16 => ctl_gaming_flip_mode_flag_t::CTL_GAMING_FLIP_MODE_FLAG_SPEED_FRAME,
            32 => ctl_gaming_flip_mode_flag_t::CTL_GAMING_FLIP_MODE_FLAG_CAPPED_FPS,
            _ => return Err(Error(_ctl_result_t::CTL_RESULT_ERROR_UNKNOWN)),
        };

        Ok(flip_mode)
    }
}
