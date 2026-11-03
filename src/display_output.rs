use std::sync::Arc;

use crate::{
    error::{Error, Result},
    ffi::{
        ctl_display_config_flag_t, ctl_display_config_flags_t, ctl_display_output_handle_t,
        ctl_display_properties_t, ctl_intel_arc_sync_monitor_params_t,
        ctl_intel_arc_sync_profile_params_t, ctl_intel_arc_sync_profile_t,
        ctl_std_display_feature_flag_t, ctl_std_display_feature_flags_t, ControlLib,
    },
};

#[doc(alias = "ctl_display_output_handle_t")]
pub struct DisplayOutput {
    pub(crate) display_output_handle: ctl_display_output_handle_t,
    pub(crate) control_lib: Arc<ControlLib>,
}

impl DisplayOutput {
    pub fn display_output_handle(&self) -> ctl_display_output_handle_t {
        self.display_output_handle
    }

    #[doc(alias = "ctlGetDisplayProperties")]
    pub fn properties(&self) -> Result<DisplayProperties> {
        let mut properties = ctl_display_properties_t {
            Size: std::mem::size_of::<ctl_display_properties_t>() as u32,
            Version: 0,
            ..Default::default()
        };

        Error::from_result(unsafe {
            self.control_lib
                .ctlGetDisplayProperties(self.display_output_handle, &mut properties)
        })?;

        Ok(DisplayProperties(properties))
    }

    #[doc(alias = "ctlGetIntelArcSyncInfoForMonitor")]
    pub fn intel_arc_sync_monitor_params(&self) -> Result<IntelArcSyncMonitorParams> {
        let mut params = ctl_intel_arc_sync_monitor_params_t {
            Size: std::mem::size_of::<ctl_intel_arc_sync_monitor_params_t>() as u32,
            Version: 0,
            ..Default::default()
        };

        Error::from_result(unsafe {
            self.control_lib
                .ctlGetIntelArcSyncInfoForMonitor(self.display_output_handle, &mut params)
        })?;

        Ok(IntelArcSyncMonitorParams {
            is_supported: params.IsIntelArcSyncSupported,
            minimum_refresh_rate_hz: params.MinimumRefreshRateInHz,
            maximum_refresh_rate_hz: params.MaximumRefreshRateInHz,
            max_frame_time_increase_us: params.MaxFrameTimeIncreaseInUs,
            max_frame_time_decrease_us: params.MaxFrameTimeDecreaseInUs,
        })
    }

    #[doc(alias = "ctlGetIntelArcSyncProfile")]
    pub fn intel_arc_sync_profile(&self) -> Result<IntelArcSyncProfileParams> {
        let mut params = ctl_intel_arc_sync_profile_params_t {
            Size: std::mem::size_of::<ctl_intel_arc_sync_profile_params_t>() as u32,
            Version: 0,
            ..Default::default()
        };

        Error::from_result(unsafe {
            self.control_lib
                .ctlGetIntelArcSyncProfile(self.display_output_handle, &mut params)
        })?;

        Ok(IntelArcSyncProfileParams {
            profile: params.IntelArcSyncProfile,
            max_refresh_rate_hz: params.MaxRefreshRateInHz,
            min_refresh_rate_hz: params.MinRefreshRateInHz,
            max_frame_time_increase_us: params.MaxFrameTimeIncreaseInUs,
            max_frame_time_decrease_us: params.MaxFrameTimeDecreaseInUs,
        })
    }
}

#[doc(alias = "ctl_display_properties_t")]
pub struct DisplayProperties(pub ctl_display_properties_t);

impl DisplayProperties {
    /// The Windows `DISPLAYCONFIG` target ID of this output. Combined with the adapter LUID
    /// from [`crate::device_adapter::DeviceAdapter::device_id()`] this identifies the display
    /// in `QueryDisplayConfig()`.
    #[cfg(windows)]
    pub fn windows_display_encoder_id(&self) -> u32 {
        unsafe { self.0.Os_display_encoder_handle.WindowsDisplayEncoderID }
    }

    pub fn display_config_flags(&self) -> ctl_display_config_flags_t {
        self.0.DisplayConfigFlags
    }

    pub fn is_active(&self) -> bool {
        self.has_config_flag(ctl_display_config_flag_t::CTL_DISPLAY_CONFIG_FLAG_DISPLAY_ACTIVE)
    }

    pub fn is_attached(&self) -> bool {
        self.has_config_flag(ctl_display_config_flag_t::CTL_DISPLAY_CONFIG_FLAG_DISPLAY_ATTACHED)
    }

    fn has_config_flag(&self, flag: ctl_display_config_flag_t) -> bool {
        self.0.DisplayConfigFlags & flag.0 as u32 != 0
    }

    pub fn feature_supported_flags(&self) -> ctl_std_display_feature_flags_t {
        self.0.FeatureSupportedFlags
    }

    pub fn feature_enabled_flags(&self) -> ctl_std_display_feature_flags_t {
        self.0.FeatureEnabledFlags
    }

    #[doc(alias = "CTL_STD_DISPLAY_FEATURE_FLAG_ADAPTIVESYNC_VRR")]
    pub fn is_adaptive_sync_supported(&self) -> bool {
        self.0.FeatureSupportedFlags
            & ctl_std_display_feature_flag_t::CTL_STD_DISPLAY_FEATURE_FLAG_ADAPTIVESYNC_VRR.0 as u32
            != 0
    }

    #[doc(alias = "CTL_STD_DISPLAY_FEATURE_FLAG_ADAPTIVESYNC_VRR")]
    pub fn is_adaptive_sync_enabled(&self) -> bool {
        self.0.FeatureEnabledFlags
            & ctl_std_display_feature_flag_t::CTL_STD_DISPLAY_FEATURE_FLAG_ADAPTIVESYNC_VRR.0 as u32
            != 0
    }

    /// Refresh rate of the currently applied timing, in Hz.
    pub fn refresh_rate(&self) -> f32 {
        self.0.Display_Timing_Info.RefreshRate
    }
}

#[doc(alias = "ctl_intel_arc_sync_monitor_params_t")]
#[derive(Debug)]
pub struct IntelArcSyncMonitorParams {
    pub is_supported: bool,
    pub minimum_refresh_rate_hz: f32,
    pub maximum_refresh_rate_hz: f32,
    pub max_frame_time_increase_us: u32,
    pub max_frame_time_decrease_us: u32,
}

#[doc(alias = "ctl_intel_arc_sync_profile_params_t")]
#[derive(Debug)]
pub struct IntelArcSyncProfileParams {
    pub profile: ctl_intel_arc_sync_profile_t,
    pub max_refresh_rate_hz: f32,
    pub min_refresh_rate_hz: f32,
    pub max_frame_time_increase_us: u32,
    pub max_frame_time_decrease_us: u32,
}
