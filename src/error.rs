use crate::ffi;
use core::fmt;
use std::{
    fmt::{Display, Formatter},
    mem::MaybeUninit,
};
pub struct Error(ffi::ctl_result_t);

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.0 {
            ffi::_ctl_result_t_CTL_RESULT_SUCCESS => {
                "success."
            }
            ffi::_ctl_result_t_CTL_RESULT_SUCCESS_STILL_OPEN_BY_ANOTHER_CALLER => {
                "success but still open by another caller."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_SUCCESS_END => {
                "Success group error code end value, not to be used."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_GENERIC_START => {
                "Generic error code starting value, not to be used."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_NOT_INITIALIZED => {
                "Result not initialized."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_ALREADY_INITIALIZED => {
                "Already initialized."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_DEVICE_LOST => {
                "Device hung, reset, was removed, or driver update occurred."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_OUT_OF_HOST_MEMORY => {
                "Insufficient host memory to satisfy call."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_OUT_OF_DEVICE_MEMORY => {
                "Insufficient device memory to satisfy call."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INSUFFICIENT_PERMISSIONS => {
                "Access denied due to permission level."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_NOT_AVAILABLE => {
                "Resource was removed."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_UNINITIALIZED => {
                "Library not initialized."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_UNSUPPORTED_VERSION => {
                "Generic error code for unsupported versions."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_UNSUPPORTED_FEATURE => {
                "Generic error code for unsupported features."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_ARGUMENT => {
                "Generic error code for invalid arguments."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_API_HANDLE => {
                "API handle in invalid."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_NULL_HANDLE => {
                "Handle argument is not valid."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_NULL_POINTER => {
                "Pointer argument may not be nullptr."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_SIZE => {
                "Size argument is invalid (e.g., must not be zero)"
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_UNSUPPORTED_SIZE => {
                "Size argument is not supported by the device (e.g., too large)"
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_UNSUPPORTED_IMAGE_FORMAT => {
                "Image format is not supported by the device."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_DATA_READ => {
                "Data read error."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_DATA_WRITE => {
                "Data write error."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_DATA_NOT_FOUND => {
                "Data not found error."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_NOT_IMPLEMENTED => {
                "Function not implemented."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_OS_CALL => {
                "Operating system call failure."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_KMD_CALL => {
                "Kernel mode driver call failure."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_UNLOAD => {
                "Library unload failure."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_ZE_LOADER => {
                "Level0 loader not found."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_OPERATION_TYPE => {
                "Invalid operation type."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_NULL_OS_INTERFACE => {
                "Null OS interface."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_NULL_OS_ADAPATER_HANDLE => {
                "Null OS adapter handle."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_NULL_OS_DISPLAY_OUTPUT_HANDLE => {
                "Null display output handle."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_WAIT_TIMEOUT => {
                "Timeout in Wait function."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_PERSISTANCE_NOT_SUPPORTED => {
                "Persistance not supported."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_PLATFORM_NOT_SUPPORTED => {
                "Platform not supported."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_UNKNOWN_APPLICATION_UID => {
                "Unknown Appplicaion UID in Initialization call."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_ENUMERATION => {
                "The enum is not valid."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_FILE_DELETE => {
                "Error in file delete operation."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_RESET_DEVICE_REQUIRED => {
                "The device requires a reset."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_FULL_REBOOT_REQUIRED => {
                "The device requires a full reboot."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_LOAD => {
                "Library load failure."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_UNKNOWN => {
                "Unknown or internal error."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_RETRY_OPERATION => {
                "Operation failed, retry previous operation again."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_START => {
                "Core error code starting value, not to be used."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_NOT_SUPPORTED => {
                "The Overclock is not supported."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_VOLTAGE_OUTSIDE_RANGE => {
                "The Voltage exceeds the acceptable min/max."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_FREQUENCY_OUTSIDE_RANGE => {
                "The Frequency exceeds the acceptable min/max."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_POWER_OUTSIDE_RANGE => {
                "The Power exceeds the acceptable min/max."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_TEMPERATURE_OUTSIDE_RANGE => {
                "The Temperature exceeds the acceptable min/max."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_IN_VOLTAGE_LOCKED_MODE => {
                "The Overclock is in voltage locked mode."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_RESET_REQUIRED => {
                "It indicates that the requested change will not be applied until the device is reset."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_WAIVER_NOT_SET => {
                "The $OverclockWaiverSet function has not been called."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_OVERCLOCK_DEPRECATED_API => {
                "The error indicates to switch to newer API version if applicable."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CORE_END => {
                "Core error code end value, not to be used."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_3D_START => {
                "3D error code starting value, not to be used"
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_3D_END => {
                "3D error code end value, not to be used."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_MEDIA_START => {
                "Media error code starting value, not to be used."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_MEDIA_END => {
                "Media error code end value, not to be used."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_DISPLAY_START => {
                "Display error code starting value, not to be used."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_AUX_ACCESS_FLAG => {
                "Invalid flag for Aux access."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_SHARPNESS_FILTER_FLAG => {
                "Invalid flag for Sharpness."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_DISPLAY_NOT_ATTACHED => {
                "Error for Display not attached."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_DISPLAY_NOT_ACTIVE => {
                "Error for display attached but not active."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_POWERFEATURE_OPTIMIZATION_FLAG => {
                "Error for invalid power optimization flag."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_POWERSOURCE_TYPE_FOR_DPST => {
                "DPST is supported only in DC Mode."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_PIXTX_GET_CONFIG_QUERY_TYPE => {
                "Invalid query type for pixel transformation get configuration."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_PIXTX_SET_CONFIG_OPERATION_TYPE => {
                "Invalid operation type for pixel transformation set configuration."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_SET_CONFIG_NUMBER_OF_SAMPLES => {
                "Invalid number of samples for pixel transformation set configuration."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_PIXTX_BLOCK_ID => {
                "Invalid block id for pixel transformation."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_PIXTX_BLOCK_TYPE => {
                "Invalid block type for pixel transformation."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INVALID_PIXTX_BLOCK_NUMBER => {
                "Invalid block number for pixel transformation."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_INSUFFICIENT_PIXTX_BLOCK_CONFIG_MEMORY => {
                "Insufficient memery allocated for BlockConfigs."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_3DLUT_INVALID_PIPE => {
                "Invalid pipe for 3dlut."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_3DLUT_INVALID_DATA => {
                "Invalid 3dlut data."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_3DLUT_NOT_SUPPORTED_IN_HDR => {
                "3dlut not supported in HDR."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_3DLUT_INVALID_OPERATION => {
                "Invalid 3dlut operation."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_3DLUT_UNSUCCESSFUL => {
                "3dlut call unsuccessful."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_AUX_DEFER => {
                "AUX defer failure."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_AUX_TIMEOUT => {
                "AUX timeout failure."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_AUX_INCOMPLETE_WRITE => {
                "AUX incomplete write failure."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_I2C_AUX_STATUS_UNKNOWN => {
                "I2C/AUX unkonown failure."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_I2C_AUX_UNSUCCESSFUL => {
                "I2C/AUX unsuccessful."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_LACE_INVALID_DATA_ARGUMENT_PASSED => {
                "Lace Incorrrect AggressivePercent data or LuxVsAggressive Map data passed by user."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_EXTERNAL_DISPLAY_ATTACHED => {
                "External Display is Attached hence fail the Display Switch."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CUSTOM_MODE_STANDARD_CUSTOM_MODE_EXISTS => {
                "Standard custom mode exists."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CUSTOM_MODE_NON_CUSTOM_MATCHING_MODE_EXISTS => {
                "Non custom matching mode exists."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_CUSTOM_MODE_INSUFFICIENT_MEMORY => {
                "Custom mode insufficent memory."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_ADAPTER_ALREADY_LINKED => {
                "Adapter is already linked."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_ADAPTER_NOT_IDENTICAL => {
                "Adapter is not identical for linking."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_ADAPTER_NOT_SUPPORTED_ON_LDA_SECONDARY => {
                "Adapter is LDA Secondary, so not supporting requested operation."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_SET_FBC_FEATURE_NOT_SUPPORTED => {
                "Set FBC Feature not supported."
            }
            ffi::_ctl_result_t_CTL_RESULT_ERROR_DISPLAY_END => {
                "Display error code end value, not to be used."
            },
            _ => "Error unknown."
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IGCL_RESULT({self})")
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn from_result(result: ffi::ctl_result_t) -> Result<(), Self> {
        match result {
            ffi::_ctl_result_t_CTL_RESULT_SUCCESS => Ok(()),
            x => Err(Self(x)),
        }
    }

    pub fn from_result_with_assume_init_on_success<T>(
        result: ffi::ctl_result_t,
        ret: MaybeUninit<T>,
    ) -> Result<T, Self> {
        Self::from_result(result).map(|()| unsafe { ret.assume_init() })
    }
}
