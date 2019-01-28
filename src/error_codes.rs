/// Mod Error Codes

use super::bindings;
use super::bindings::*;

/// Rust version of all the error codes in the Lepton SDK
/// `CommOk` is lumped in with regular `Ok` since Rust doesn't let
/// two enum variants be represented by the same integer.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum LeptonError {
    Ok = Result_LEP_OK,
    Error = Result_LEP_ERROR,
    NotReady = Result_LEP_NOT_READY,
    RangeError = Result_LEP_RANGE_ERROR,
    ChecksumError = Result_LEP_CHECKSUM_ERROR,
    BadArgPointerError = Result_LEP_BAD_ARG_POINTER_ERROR,
    DataSizeError = Result_LEP_DATA_SIZE_ERROR,
    UndefinedFunctionError =Result_LEP_UNDEFINED_FUNCTION_ERROR,
    FunctionNotSupported = Result_LEP_FUNCTION_NOT_SUPPORTED,
    OTPWriteError = Result_LEP_OTP_WRITE_ERROR,
    OTPReadError = Result_LEP_OTP_READ_ERROR,
    OTPNotProgrammedError = Result_LEP_OTP_NOT_PROGRAMMED_ERROR,
    I2CBusNotReady = Result_LEP_ERROR_I2C_BUS_NOT_READY,
    I2CBufferOverflow = Result_LEP_ERROR_I2C_BUFFER_OVERFLOW,
    I2CArbitrationLost = Result_LEP_ERROR_I2C_ARBITRATION_LOST,
    I2CBusError = Result_LEP_ERROR_I2C_BUS_ERROR,
    I2CNackRecieved = Result_LEP_ERROR_I2C_NACK_RECEIVED,
    I2CFail = Result_LEP_ERROR_I2C_FAIL,
    DivZeroError = Result_LEP_DIV_ZERO_ERROR,
    CommPortNotOpen = Result_LEP_COMM_PORT_NOT_OPEN,
    CommInvalidPortError = Result_LEP_COMM_INVALID_PORT_ERROR,
    CommRangeError = Result_LEP_COMM_RANGE_ERROR,
    ErrorCreatingComm = Result_LEP_ERROR_CREATING_COMM,
    ErrorStartingComm = Result_LEP_ERROR_STARTING_COMM,
    ErrorClosingComm = Result_LEP_ERROR_CLOSING_COMM,
    CommChecksumError = Result_LEP_COMM_CHECKSUM_ERROR,
    CommNoDevice = Result_LEP_COMM_NO_DEV,
    TimeoutError = Result_LEP_TIMEOUT_ERROR,
    ErrorWritingComm = Result_LEP_COMM_ERROR_WRITING_COMM,
    ErrorReadingComm = Result_LEP_COMM_ERROR_READING_COMM,
    CommCountError = Result_LEP_COMM_COUNT_ERROR,
    OperationCanceled = Result_LEP_OPERATION_CANCELED,
    UndefinedError = Result_LEP_UNDEFINED_ERROR_CODE,
}

#[allow(dead_code)]
impl LeptonError {
    fn is_ok(self) -> bool {
        self == LeptonError::Ok
    }
}

impl std::convert::From<bindings::Result> for LeptonError {
    // Looks like the best way to convert from a C enum (integer) to a Rust enum
    // is still a lookup table
    #[allow(non_upper_case_globals)]
    fn from(result: bindings::Result) -> Self {
        match result {
            Result_LEP_OK => LeptonError::Ok,
            // Result_LEP_COMM_OK => LeptonError::Ok,
            Result_LEP_ERROR => LeptonError::Error,
            Result_LEP_NOT_READY => LeptonError::NotReady,
            Result_LEP_RANGE_ERROR => LeptonError::RangeError ,
            Result_LEP_CHECKSUM_ERROR => LeptonError::ChecksumError ,
            Result_LEP_BAD_ARG_POINTER_ERROR => LeptonError::BadArgPointerError ,
            Result_LEP_DATA_SIZE_ERROR => LeptonError::DataSizeError ,
            Result_LEP_UNDEFINED_FUNCTION_ERROR => LeptonError::UndefinedFunctionError ,
            Result_LEP_FUNCTION_NOT_SUPPORTED => LeptonError::FunctionNotSupported ,
            Result_LEP_OTP_WRITE_ERROR => LeptonError::OTPWriteError ,
            Result_LEP_OTP_READ_ERROR => LeptonError::OTPReadError ,
            Result_LEP_OTP_NOT_PROGRAMMED_ERROR => LeptonError::OTPNotProgrammedError ,
            Result_LEP_ERROR_I2C_BUS_NOT_READY => LeptonError::I2CBusNotReady ,
            Result_LEP_ERROR_I2C_BUFFER_OVERFLOW => LeptonError::I2CBufferOverflow ,
            Result_LEP_ERROR_I2C_ARBITRATION_LOST => LeptonError::I2CArbitrationLost ,
            Result_LEP_ERROR_I2C_BUS_ERROR => LeptonError::I2CBusError ,
            Result_LEP_ERROR_I2C_NACK_RECEIVED => LeptonError::I2CNackRecieved ,
            Result_LEP_ERROR_I2C_FAIL => LeptonError::I2CFail ,
            Result_LEP_DIV_ZERO_ERROR => LeptonError::DivZeroError ,
            Result_LEP_COMM_PORT_NOT_OPEN => LeptonError::CommPortNotOpen ,
            Result_LEP_COMM_INVALID_PORT_ERROR => LeptonError::CommInvalidPortError ,
            Result_LEP_COMM_RANGE_ERROR => LeptonError::CommRangeError ,
            Result_LEP_ERROR_CREATING_COMM => LeptonError::ErrorCreatingComm ,
            Result_LEP_ERROR_STARTING_COMM => LeptonError::ErrorStartingComm ,
            Result_LEP_ERROR_CLOSING_COMM => LeptonError::ErrorClosingComm ,
            Result_LEP_COMM_CHECKSUM_ERROR => LeptonError::CommChecksumError ,
            Result_LEP_COMM_NO_DEV => LeptonError::CommNoDevice ,
            Result_LEP_TIMEOUT_ERROR => LeptonError::TimeoutError ,
            Result_LEP_COMM_ERROR_WRITING_COMM => LeptonError::ErrorWritingComm ,
            Result_LEP_COMM_ERROR_READING_COMM => LeptonError::ErrorReadingComm ,
            Result_LEP_COMM_COUNT_ERROR => LeptonError::CommCountError ,
            Result_LEP_OPERATION_CANCELED => LeptonError::OperationCanceled,
            Result_LEP_UNDEFINED_ERROR_CODE => LeptonError::UndefinedError,
            _ => panic!("Unexpected Lepton Result!")

        }
    }
}

