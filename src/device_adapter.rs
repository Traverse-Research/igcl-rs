use std::ffi::CStr;
use std::os::raw::c_void;
use std::{ffi::OsStr, sync::Arc};

use crate::error::Result;

use crate::{
    error::Error,
    ffi::{
        ctl_3d_feature_getset_t, ctl_3d_feature_t, ctl_adapter_bdf_t, ctl_device_adapter_handle_t,
        ctl_device_adapter_properties_t, ctl_device_type_t, ctl_gaming_flip_mode_flag_t,
        ctl_property_value_type_t, ControlLib, _ctl_result_t, ctl_endurance_gaming_t,
        ctl_oc_telemetry_item_t, ctl_power_telemetry_t, ctl_result_t,
    },
};

/// Specifies the scope in which to query for driver settings.
/// Note that IGCL will not fall back to a wider scope when settings haven't been specified for the current one.
/// This is solved by manually falling back to wider scopes in the query functions themselves.
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

    /// Get the scope that encapsulates the current one (if any).
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

    pub fn name(&self) -> &CStr {
        CStr::from_bytes_until_nul(bytemuck::cast_slice(&self.adapter_properties.name)).unwrap()
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

    pub fn power_telemetry(&self) -> Result<Telemetry> {
        let mut telemetry = ctl_power_telemetry_t {
            Size: std::mem::size_of::<ctl_power_telemetry_t>() as u32,
            ..Default::default()
        };

        let result = unsafe {
            self.control_lib
                .ctlPowerTelemetryGet(self.device_adapter_handle, &mut telemetry)
        };

        Error::from_result(result)?;

        Ok(Telemetry {
            time_stamp: telemetry.timeStamp.into(),
            gpu_energy_counter: telemetry.gpuEnergyCounter.into(),
            gpu_voltage: telemetry.gpuVoltage.into(),
            gpu_current_clock_frequency: telemetry.gpuCurrentClockFrequency.into(),
            gpu_current_temperature: telemetry.gpuCurrentTemperature.into(),
            global_activity_counter: telemetry.globalActivityCounter.into(),
            render_compute_activity_counter: telemetry.renderComputeActivityCounter.into(),
            media_activity_counter: telemetry.mediaActivityCounter.into(),
            vram_energy_counter: telemetry.vramEnergyCounter.into(),
            vram_voltage: telemetry.vramVoltage.into(),
            vram_current_clock_frequency: telemetry.vramCurrentClockFrequency.into(),
            vram_current_effective_frequency: telemetry.vramCurrentEffectiveFrequency.into(),
            vram_read_bandwidth_counter: telemetry.vramReadBandwidthCounter.into(),
            vram_write_bandwidth_counter: telemetry.vramWriteBandwidthCounter.into(),
            vram_current_temperature: telemetry.vramCurrentTemperature.into(),
            total_card_energy_counter: telemetry.totalCardEnergyCounter.into(),
            fan_speed: [
                telemetry.fanSpeed[0].into(),
                telemetry.fanSpeed[1].into(),
                telemetry.fanSpeed[2].into(),
                telemetry.fanSpeed[3].into(),
                telemetry.fanSpeed[4].into(),
            ],
        })
    }
}

#[derive(Debug)]
pub enum Value {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
}

#[derive(Debug)]
pub enum Unit {
    FrequencyMhz(Value),
    OperationsGts(Value),
    OperationsMts(Value),
    VoltageVolts(Value),
    PowerWatts(Value),
    TemperatureCelsius(Value),
    EnergyJoules(Value),
    TimeSeconds(Value),
    MemoryBytes(Value),
    AngularSpeedRpm(Value),
    PowerMilliwatts(Value),
    Percent(Value),
    MemSpeedGbps(Value),
    VoltageMillivolts(Value),
}

#[derive(Debug)]
pub struct TelemetryItem(pub Option<Unit>);

impl From<ctl_oc_telemetry_item_t> for TelemetryItem {
    fn from(item: ctl_oc_telemetry_item_t) -> Self {
        TelemetryItem(if item.bSupported {
            let value = match item.type_ {
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_INT8 => {
                    Value::I8(unsafe { item.value.data8 })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_UINT8 => {
                    Value::U8(unsafe { item.value.datau8 })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_INT16 => {
                    Value::I16(unsafe { item.value.data16 })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_UINT16 => {
                    Value::U16(unsafe { item.value.datau16 })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_INT32 => {
                    Value::I32(unsafe { item.value.data32 })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_UINT32 => {
                    Value::U32(unsafe { item.value.datau32 })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_INT64 => {
                    Value::I64(unsafe { item.value.data64 })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_UINT64 => {
                    Value::U64(unsafe { item.value.datau64 })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_FLOAT => {
                    Value::F32(unsafe { item.value.datafloat })
                }
                crate::ffi::ctl_data_type_t::CTL_DATA_TYPE_DOUBLE => {
                    Value::F64(unsafe { item.value.datadouble })
                }
                _ => return TelemetryItem(None),
            };

            Some(match item.units {
                crate::ffi::_ctl_units_t::CTL_UNITS_FREQUENCY_MHZ => Unit::FrequencyMhz(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_OPERATIONS_GTS => Unit::OperationsGts(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_OPERATIONS_MTS => Unit::OperationsMts(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_VOLTAGE_VOLTS => Unit::VoltageVolts(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_POWER_WATTS => Unit::PowerWatts(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_TEMPERATURE_CELSIUS => {
                    Unit::TemperatureCelsius(value)
                }
                crate::ffi::_ctl_units_t::CTL_UNITS_ENERGY_JOULES => Unit::EnergyJoules(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_TIME_SECONDS => Unit::TimeSeconds(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_MEMORY_BYTES => Unit::MemoryBytes(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_ANGULAR_SPEED_RPM => {
                    Unit::AngularSpeedRpm(value)
                }
                crate::ffi::_ctl_units_t::CTL_UNITS_POWER_MILLIWATTS => {
                    Unit::PowerMilliwatts(value)
                }
                crate::ffi::_ctl_units_t::CTL_UNITS_PERCENT => Unit::Percent(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_MEM_SPEED_GBPS => Unit::MemSpeedGbps(value),
                crate::ffi::_ctl_units_t::CTL_UNITS_VOLTAGE_MILLIVOLTS => {
                    Unit::VoltageMillivolts(value)
                }
                _ => return TelemetryItem(None),
            })
        } else {
            None
        })
    }
}

#[derive(Debug)]
pub struct Telemetry {
    pub time_stamp: TelemetryItem,
    pub gpu_energy_counter: TelemetryItem,
    pub gpu_voltage: TelemetryItem,
    pub gpu_current_clock_frequency: TelemetryItem,
    pub gpu_current_temperature: TelemetryItem,
    pub global_activity_counter: TelemetryItem,
    pub render_compute_activity_counter: TelemetryItem,
    pub media_activity_counter: TelemetryItem,
    pub vram_energy_counter: TelemetryItem,
    pub vram_voltage: TelemetryItem,
    pub vram_current_clock_frequency: TelemetryItem,
    pub vram_current_effective_frequency: TelemetryItem,
    pub vram_read_bandwidth_counter: TelemetryItem,
    pub vram_write_bandwidth_counter: TelemetryItem,
    pub vram_current_temperature: TelemetryItem,
    pub total_card_energy_counter: TelemetryItem,
    pub fan_speed: [TelemetryItem; 5],
    // pub gpuPowerLimited: bool,
    // pub gpuTemperatureLimited: bool,
    // pub gpuCurrentLimited: bool,
    // pub gpuVoltageLimited: bool,
    // pub gpuUtilizationLimited: bool,
    // pub vramPowerLimited: bool,
    // pub vramTemperatureLimited: bool,
    // pub vramCurrentLimited: bool,
    // pub vramVoltageLimited: bool,
    // pub vramUtilizationLimited: bool,
    // pub psu: [ctl_psu_info_t; 5usize],
    // pub fanSpeed: [ctl_oc_telemetry_item_t; 5usize],
}
