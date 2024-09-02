use std::os::raw::c_void;
use std::{ffi::OsStr, sync::Arc};

use crate::error::Result;

use crate::ffi::{_ctl_result_t, ctl_endurance_gaming_t};
use crate::{
    error::Error,
    ffi::{
        ctl_3d_feature_getset_t, ctl_3d_feature_t, ctl_adapter_bdf_t, ctl_device_adapter_handle_t,
        ctl_device_adapter_properties_t, ctl_device_type_t, ctl_gaming_flip_mode_flag_t,
        ctl_property_value_type_t, ControlLib,
    },
};

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

    pub fn feature_endurance_gaming(&self) -> Result<Box<ctl_endurance_gaming_t>> {
        let mut current_app = std::env::current_exe().map_or("".to_string(), |path| {
            path.file_name()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .to_string()
        });

        let mut settings: Box<ctl_endurance_gaming_t> = Box::new(unsafe { std::mem::zeroed() });
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

        Error::from_result(unsafe {
            self.control_lib
                .ctlGetSet3DFeature(self.device_adapter_handle, &mut feature)
        })?;

        Ok(settings)
    }

    pub fn feature_frame_limit(&self) -> Result<i32> {
        let mut current_app = std::env::current_exe().map_or("".to_string(), |path| {
            path.file_name()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .to_string()
        });

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

        Error::from_result(unsafe {
            self.control_lib
                .ctlGetSet3DFeature(self.device_adapter_handle, &mut feature)
        })?;

        Ok(unsafe { feature.Value.IntType.Value })
    }

    pub fn feature_flip_mode(&self) -> Result<ctl_gaming_flip_mode_flag_t> {
        let mut current_app = std::env::current_exe().map_or("".to_string(), |path| {
            path.file_name()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .to_string()
        });

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

        Error::from_result(unsafe {
            self.control_lib
                .ctlGetSet3DFeature(self.device_adapter_handle, &mut feature)
        })?;

        let flip_mode = match unsafe { feature.Value.EnumType.EnableType } {
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
