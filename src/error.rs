use std::{fmt, mem::MaybeUninit};

use crate::ffi::ctl_result_t;

#[doc(alias = "ctl_result_t")]
pub struct Error(pub ctl_result_t);

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            ctl_result_t::CTL_RESULT_SUCCESS => {
                "success."
            }
            ctl_result_t::CTL_RESULT_SUCCESS_STILL_OPEN_BY_ANOTHER_CALLER => {
                "success but still open by another caller."
            }
            ctl_result_t::CTL_RESULT_ERROR_SUCCESS_END => {
                "Success group error code end value, not to be used."
            }
            ctl_result_t::CTL_RESULT_ERROR_GENERIC_START => {
                "Generic error code starting value, not to be used."
            }
            ctl_result_t::CTL_RESULT_ERROR_NOT_INITIALIZED => {
                "Result not initialized."
            }
            ctl_result_t::CTL_RESULT_ERROR_ALREADY_INITIALIZED => {
                "Already initialized."
            }
            ctl_result_t::CTL_RESULT_ERROR_DEVICE_LOST => {
                "Device hung, reset, was removed, or driver update occurred."
            }
            ctl_result_t::CTL_RESULT_ERROR_OUT_OF_HOST_MEMORY => {
                "Insufficient host memory to satisfy call."
            }
            ctl_result_t::CTL_RESULT_ERROR_OUT_OF_DEVICE_MEMORY => {
                "Insufficient device memory to satisfy call."
            }
            ctl_result_t::CTL_RESULT_ERROR_INSUFFICIENT_PERMISSIONS => {
                "Access denied due to permission level."
            }
            ctl_result_t::CTL_RESULT_ERROR_NOT_AVAILABLE => {
                "Resource was removed."
            }
            ctl_result_t::CTL_RESULT_ERROR_UNINITIALIZED => {
                "Library not initialized."
            }
            ctl_result_t::CTL_RESULT_ERROR_UNSUPPORTED_VERSION => {
                "Generic error code for unsupported versions."
            }
            ctl_result_t::CTL_RESULT_ERROR_UNSUPPORTED_FEATURE => {
                "Generic error code for unsupported features."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_ARGUMENT => {
                "Generic error code for invalid arguments."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_API_HANDLE => {
                "API handle in invalid."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_NULL_HANDLE => {
                "Handle argument is not valid."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_NULL_POINTER => {
                "Pointer argument may not be nullptr."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_SIZE => {
                "Size argument is invalid (e.g., must not be zero)"
            }
            ctl_result_t::CTL_RESULT_ERROR_UNSUPPORTED_SIZE => {
                "Size argument is not supported by the device (e.g., too large)"
            }
            ctl_result_t::CTL_RESULT_ERROR_UNSUPPORTED_IMAGE_FORMAT => {
                "Image format is not supported by the device."
            }
            ctl_result_t::CTL_RESULT_ERROR_DATA_READ => {
                "Data read error."
            }
            ctl_result_t::CTL_RESULT_ERROR_DATA_WRITE => {
                "Data write error."
            }
            ctl_result_t::CTL_RESULT_ERROR_DATA_NOT_FOUND => {
                "Data not found error."
            }
            ctl_result_t::CTL_RESULT_ERROR_NOT_IMPLEMENTED => {
                "Function not implemented."
            }
            ctl_result_t::CTL_RESULT_ERROR_OS_CALL => {
                "Operating system call failure."
            }
            ctl_result_t::CTL_RESULT_ERROR_KMD_CALL => {
                "Kernel mode driver call failure."
            }
            ctl_result_t::CTL_RESULT_ERROR_UNLOAD => {
                "Library unload failure."
            }
            ctl_result_t::CTL_RESULT_ERROR_ZE_LOADER => {
                "Level0 loader not found."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_OPERATION_TYPE => {
                "Invalid operation type."
            }
            ctl_result_t::CTL_RESULT_ERROR_NULL_OS_INTERFACE => {
                "Null OS interface."
            }
            ctl_result_t::CTL_RESULT_ERROR_NULL_OS_ADAPATER_HANDLE => {
                "Null OS adapter handle."
            }
            ctl_result_t::CTL_RESULT_ERROR_NULL_OS_DISPLAY_OUTPUT_HANDLE => {
                "Null display output handle."
            }
            ctl_result_t::CTL_RESULT_ERROR_WAIT_TIMEOUT => {
                "Timeout in Wait function."
            }
            ctl_result_t::CTL_RESULT_ERROR_PERSISTANCE_NOT_SUPPORTED => {
                "Persistance not supported."
            }
            ctl_result_t::CTL_RESULT_ERROR_PLATFORM_NOT_SUPPORTED => {
                "Platform not supported."
            }
            ctl_result_t::CTL_RESULT_ERROR_UNKNOWN_APPLICATION_UID => {
                "Unknown Appplicaion UID in Initialization call."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_ENUMERATION => {
                "The enum is not valid."
            }
            ctl_result_t::CTL_RESULT_ERROR_FILE_DELETE => {
                "Error in file delete operation."
            }
            ctl_result_t::CTL_RESULT_ERROR_RESET_DEVICE_REQUIRED => {
                "The device requires a reset."
            }
            ctl_result_t::CTL_RESULT_ERROR_FULL_REBOOT_REQUIRED => {
                "The device requires a full reboot."
            }
            ctl_result_t::CTL_RESULT_ERROR_LOAD => {
                "Library load failure."
            }
            ctl_result_t::CTL_RESULT_ERROR_UNKNOWN => {
                "Unknown or internal error."
            }
            ctl_result_t::CTL_RESULT_ERROR_RETRY_OPERATION => {
                "Operation failed, retry previous operation again."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_START => {
                "Core error code starting value, not to be used."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_NOT_SUPPORTED => {
                "The Overclock is not supported."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_VOLTAGE_OUTSIDE_RANGE => {
                "The Voltage exceeds the acceptable min/max."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_FREQUENCY_OUTSIDE_RANGE => {
                "The Frequency exceeds the acceptable min/max."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_POWER_OUTSIDE_RANGE => {
                "The Power exceeds the acceptable min/max."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_TEMPERATURE_OUTSIDE_RANGE => {
                "The Temperature exceeds the acceptable min/max."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_IN_VOLTAGE_LOCKED_MODE => {
                "The Overclock is in voltage locked mode."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_RESET_REQUIRED => {
                "It indicates that the requested change will not be applied until the device is reset."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_WAIVER_NOT_SET => {
                "The $OverclockWaiverSet function has not been called."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_OVERCLOCK_DEPRECATED_API => {
                "The error indicates to switch to newer API version if applicable."
            }
            ctl_result_t::CTL_RESULT_ERROR_CORE_END => {
                "Core error code end value, not to be used."
            }
            ctl_result_t::CTL_RESULT_ERROR_3D_START => {
                "3D error code starting value, not to be used"
            }
            ctl_result_t::CTL_RESULT_ERROR_3D_END => {
                "3D error code end value, not to be used."
            }
            ctl_result_t::CTL_RESULT_ERROR_MEDIA_START => {
                "Media error code starting value, not to be used."
            }
            ctl_result_t::CTL_RESULT_ERROR_MEDIA_END => {
                "Media error code end value, not to be used."
            }
            ctl_result_t::CTL_RESULT_ERROR_DISPLAY_START => {
                "Display error code starting value, not to be used."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_AUX_ACCESS_FLAG => {
                "Invalid flag for Aux access."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_SHARPNESS_FILTER_FLAG => {
                "Invalid flag for Sharpness."
            }
            ctl_result_t::CTL_RESULT_ERROR_DISPLAY_NOT_ATTACHED => {
                "Error for Display not attached."
            }
            ctl_result_t::CTL_RESULT_ERROR_DISPLAY_NOT_ACTIVE => {
                "Error for display attached but not active."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_POWERFEATURE_OPTIMIZATION_FLAG => {
                "Error for invalid power optimization flag."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_POWERSOURCE_TYPE_FOR_DPST => {
                "DPST is supported only in DC Mode."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_PIXTX_GET_CONFIG_QUERY_TYPE => {
                "Invalid query type for pixel transformation get configuration."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_PIXTX_SET_CONFIG_OPERATION_TYPE => {
                "Invalid operation type for pixel transformation set configuration."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_SET_CONFIG_NUMBER_OF_SAMPLES => {
                "Invalid number of samples for pixel transformation set configuration."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_PIXTX_BLOCK_ID => {
                "Invalid block id for pixel transformation."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_PIXTX_BLOCK_TYPE => {
                "Invalid block type for pixel transformation."
            }
            ctl_result_t::CTL_RESULT_ERROR_INVALID_PIXTX_BLOCK_NUMBER => {
                "Invalid block number for pixel transformation."
            }
            ctl_result_t::CTL_RESULT_ERROR_INSUFFICIENT_PIXTX_BLOCK_CONFIG_MEMORY => {
                "Insufficient memery allocated for BlockConfigs."
            }
            ctl_result_t::CTL_RESULT_ERROR_3DLUT_INVALID_PIPE => {
                "Invalid pipe for 3dlut."
            }
            ctl_result_t::CTL_RESULT_ERROR_3DLUT_INVALID_DATA => {
                "Invalid 3dlut data."
            }
            ctl_result_t::CTL_RESULT_ERROR_3DLUT_NOT_SUPPORTED_IN_HDR => {
                "3dlut not supported in HDR."
            }
            ctl_result_t::CTL_RESULT_ERROR_3DLUT_INVALID_OPERATION => {
                "Invalid 3dlut operation."
            }
            ctl_result_t::CTL_RESULT_ERROR_3DLUT_UNSUCCESSFUL => {
                "3dlut call unsuccessful."
            }
            ctl_result_t::CTL_RESULT_ERROR_AUX_DEFER => {
                "AUX defer failure."
            }
            ctl_result_t::CTL_RESULT_ERROR_AUX_TIMEOUT => {
                "AUX timeout failure."
            }
            ctl_result_t::CTL_RESULT_ERROR_AUX_INCOMPLETE_WRITE => {
                "AUX incomplete write failure."
            }
            ctl_result_t::CTL_RESULT_ERROR_I2C_AUX_STATUS_UNKNOWN => {
                "I2C/AUX unkonown failure."
            }
            ctl_result_t::CTL_RESULT_ERROR_I2C_AUX_UNSUCCESSFUL => {
                "I2C/AUX unsuccessful."
            }
            ctl_result_t::CTL_RESULT_ERROR_LACE_INVALID_DATA_ARGUMENT_PASSED => {
                "Lace Incorrrect AggressivePercent data or LuxVsAggressive Map data passed by user."
            }
            ctl_result_t::CTL_RESULT_ERROR_EXTERNAL_DISPLAY_ATTACHED => {
                "External Display is Attached hence fail the Display Switch."
            }
            ctl_result_t::CTL_RESULT_ERROR_CUSTOM_MODE_STANDARD_CUSTOM_MODE_EXISTS => {
                "Standard custom mode exists."
            }
            ctl_result_t::CTL_RESULT_ERROR_CUSTOM_MODE_NON_CUSTOM_MATCHING_MODE_EXISTS => {
                "Non custom matching mode exists."
            }
            ctl_result_t::CTL_RESULT_ERROR_CUSTOM_MODE_INSUFFICIENT_MEMORY => {
                "Custom mode insufficent memory."
            }
            ctl_result_t::CTL_RESULT_ERROR_ADAPTER_ALREADY_LINKED => {
                "Adapter is already linked."
            }
            ctl_result_t::CTL_RESULT_ERROR_ADAPTER_NOT_IDENTICAL => {
                "Adapter is not identical for linking."
            }
            ctl_result_t::CTL_RESULT_ERROR_ADAPTER_NOT_SUPPORTED_ON_LDA_SECONDARY => {
                "Adapter is LDA Secondary, so not supporting requested operation."
            }
            ctl_result_t::CTL_RESULT_ERROR_SET_FBC_FEATURE_NOT_SUPPORTED => {
                "Set FBC Feature not supported."
            }
            ctl_result_t::CTL_RESULT_ERROR_DISPLAY_END => {
                "Display error code end value, not to be used."
            }
            ctl_result_t::CTL_RESULT_MAX => {
                "Invalid error code: this enum entry is used to indicate the size of the enum."
            },
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IGCL_RESULT({self})")
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn from_result(result: ctl_result_t) -> Result<(), Self> {
        match result {
            ctl_result_t::CTL_RESULT_SUCCESS
            | ctl_result_t::CTL_RESULT_SUCCESS_STILL_OPEN_BY_ANOTHER_CALLER => Ok(()),
            x => Err(Self(x)),
        }
    }

    pub fn from_result_with_assume_init_on_success<T>(
        result: ctl_result_t,
        ret: MaybeUninit<T>,
    ) -> Result<T, Self> {
        Self::from_result(result).map(|()| unsafe { ret.assume_init() })
    }
}
