/// Mod Error Codes
use super::bindings;
use super::bindings::*;

/// Rust version of all the error codes in the Lepton SDK
/// `CommOk` is lumped in with regular `Ok` since Rust doesn't let
/// two enum variants be represented by the same integer.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum LeptonResult {
    Ok                     = Result_LEP_OK,
    Error                  = Result_LEP_ERROR,
    NotReady               = Result_LEP_NOT_READY,
    RangeError             = Result_LEP_RANGE_ERROR,
    ChecksumError          = Result_LEP_CHECKSUM_ERROR,
    BadArgPointerError     = Result_LEP_BAD_ARG_POINTER_ERROR,
    DataSizeError          = Result_LEP_DATA_SIZE_ERROR,
    UndefinedFunctionError = Result_LEP_UNDEFINED_FUNCTION_ERROR,
    FunctionNotSupported   = Result_LEP_FUNCTION_NOT_SUPPORTED,
    OTPWriteError          = Result_LEP_OTP_WRITE_ERROR,
    OTPReadError           = Result_LEP_OTP_READ_ERROR,
    OTPNotProgrammedError  = Result_LEP_OTP_NOT_PROGRAMMED_ERROR,
    I2CBusNotReady         = Result_LEP_ERROR_I2C_BUS_NOT_READY,
    I2CBufferOverflow      = Result_LEP_ERROR_I2C_BUFFER_OVERFLOW,
    I2CArbitrationLost     = Result_LEP_ERROR_I2C_ARBITRATION_LOST,
    I2CBusError            = Result_LEP_ERROR_I2C_BUS_ERROR,
    I2CNackRecieved        = Result_LEP_ERROR_I2C_NACK_RECEIVED,
    I2CFail                = Result_LEP_ERROR_I2C_FAIL,
    DivZeroError           = Result_LEP_DIV_ZERO_ERROR,
    CommPortNotOpen        = Result_LEP_COMM_PORT_NOT_OPEN,
    CommInvalidPortError   = Result_LEP_COMM_INVALID_PORT_ERROR,
    CommRangeError         = Result_LEP_COMM_RANGE_ERROR,
    ErrorCreatingComm      = Result_LEP_ERROR_CREATING_COMM,
    ErrorStartingComm      = Result_LEP_ERROR_STARTING_COMM,
    ErrorClosingComm       = Result_LEP_ERROR_CLOSING_COMM,
    CommChecksumError      = Result_LEP_COMM_CHECKSUM_ERROR,
    CommNoDevice           = Result_LEP_COMM_NO_DEV,
    TimeoutError           = Result_LEP_TIMEOUT_ERROR,
    ErrorWritingComm       = Result_LEP_COMM_ERROR_WRITING_COMM,
    ErrorReadingComm       = Result_LEP_COMM_ERROR_READING_COMM,
    CommCountError         = Result_LEP_COMM_COUNT_ERROR,
    OperationCanceled      = Result_LEP_OPERATION_CANCELED,
    UndefinedError         = Result_LEP_UNDEFINED_ERROR_CODE,
}

#[allow(dead_code)]
impl LeptonResult {
    fn is_ok(self) -> bool {
        self == LeptonResult::Ok
    }
}

impl std::convert::From<bindings::LEP_RESULT> for LeptonResult {
    // Looks like the best way to convert from a C enum (integer) to a Rust enum
    // is still a lookup table
    #[allow(non_upper_case_globals)]
    fn from(result: bindings::LEP_RESULT) -> Self {
        match result {
            Result_LEP_OK                         => LeptonResult::Ok,
            // Result_LEP_COMM_OK                 => LeptonError::Ok,
            Result_LEP_ERROR                      => LeptonResult::Error,
            Result_LEP_NOT_READY                  => LeptonResult::NotReady,
            Result_LEP_RANGE_ERROR                => LeptonResult::RangeError,
            Result_LEP_CHECKSUM_ERROR             => LeptonResult::ChecksumError,
            Result_LEP_BAD_ARG_POINTER_ERROR      => LeptonResult::BadArgPointerError,
            Result_LEP_DATA_SIZE_ERROR            => LeptonResult::DataSizeError,
            Result_LEP_UNDEFINED_FUNCTION_ERROR   => LeptonResult::UndefinedFunctionError,
            Result_LEP_FUNCTION_NOT_SUPPORTED     => LeptonResult::FunctionNotSupported,
            Result_LEP_OTP_WRITE_ERROR            => LeptonResult::OTPWriteError,
            Result_LEP_OTP_READ_ERROR             => LeptonResult::OTPReadError,
            Result_LEP_OTP_NOT_PROGRAMMED_ERROR   => LeptonResult::OTPNotProgrammedError,
            Result_LEP_ERROR_I2C_BUS_NOT_READY    => LeptonResult::I2CBusNotReady,
            Result_LEP_ERROR_I2C_BUFFER_OVERFLOW  => LeptonResult::I2CBufferOverflow,
            Result_LEP_ERROR_I2C_ARBITRATION_LOST => LeptonResult::I2CArbitrationLost,
            Result_LEP_ERROR_I2C_BUS_ERROR        => LeptonResult::I2CBusError,
            Result_LEP_ERROR_I2C_NACK_RECEIVED    => LeptonResult::I2CNackRecieved,
            Result_LEP_ERROR_I2C_FAIL             => LeptonResult::I2CFail,
            Result_LEP_DIV_ZERO_ERROR             => LeptonResult::DivZeroError,
            Result_LEP_COMM_PORT_NOT_OPEN         => LeptonResult::CommPortNotOpen,
            Result_LEP_COMM_INVALID_PORT_ERROR    => LeptonResult::CommInvalidPortError,
            Result_LEP_COMM_RANGE_ERROR           => LeptonResult::CommRangeError,
            Result_LEP_ERROR_CREATING_COMM        => LeptonResult::ErrorCreatingComm,
            Result_LEP_ERROR_STARTING_COMM        => LeptonResult::ErrorStartingComm,
            Result_LEP_ERROR_CLOSING_COMM         => LeptonResult::ErrorClosingComm,
            Result_LEP_COMM_CHECKSUM_ERROR        => LeptonResult::CommChecksumError,
            Result_LEP_COMM_NO_DEV                => LeptonResult::CommNoDevice,
            Result_LEP_TIMEOUT_ERROR              => LeptonResult::TimeoutError,
            Result_LEP_COMM_ERROR_WRITING_COMM    => LeptonResult::ErrorWritingComm,
            Result_LEP_COMM_ERROR_READING_COMM    => LeptonResult::ErrorReadingComm,
            Result_LEP_COMM_COUNT_ERROR           => LeptonResult::CommCountError,
            Result_LEP_OPERATION_CANCELED         => LeptonResult::OperationCanceled,
            Result_LEP_UNDEFINED_ERROR_CODE       => LeptonResult::UndefinedError,
            _ => panic!("Unexpected Lepton Result!")
        }
    }
}

