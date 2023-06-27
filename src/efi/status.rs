//! Module that holds the EFI status codes
pub const EFI_SUCCESS: usize = 0;

/// Some error codes require that the high bit is set, so we make a bit mask for them here
pub const ERROR_CODE_MASK: usize = 1 << (usize::BITS - 1);

/// The image failed to load.
pub const EFI_LOAD_ERROR: usize = 1 | ERROR_CODE_MASK;
/// A parameter was incorrect.
pub const EFI_INVALID_PARAMETER: usize = 2 | ERROR_CODE_MASK;
/// The operation is not supported.
pub const EFI_UNSUPPORTED: usize = 3 | ERROR_CODE_MASK;
/// The buffer was not the proper size for the request.
pub const EFI_BAD_BUFFER_SIZE: usize = 4 | ERROR_CODE_MASK;
/// The buffer is not large enough to hold the requested data.
/// The required buffer size is returned in the appropriate parameter when this error occurs.
pub const EFI_BUFFER_TOO_SMALL: usize = 5 | ERROR_CODE_MASK;
/// There is no data pending upon return.
pub const EFI_NOT_READY: usize = 6 | ERROR_CODE_MASK;
/// The physical device reported an error while attempting the operation.
pub const EFI_DEVICE_ERROR: usize = 7 | ERROR_CODE_MASK;
/// The device cannot be written to.
pub const EFI_WRITE_PROTECTED: usize = 8 | ERROR_CODE_MASK;
/// A resource has run out.
pub const EFI_OUT_OF_RESOURCES: usize = 9 | ERROR_CODE_MASK;
/// An inconstancy was detected on the file system causing the operating to fail.
pub const EFI_VOLUME_CORRUPTED: usize = 10 | ERROR_CODE_MASK;
/// There is no more space on the file system.
pub const EFI_VOLUME_FULL: usize = 11 | ERROR_CODE_MASK;
/// The device does not contain any medium to perform the operation.
pub const EFI_NO_MEDIA: usize = 12 | ERROR_CODE_MASK;
/// The medium in the device has changed since the last access.
pub const EFI_MEDIA_CHANGED: usize = 13 | ERROR_CODE_MASK;
/// The item was not found.
pub const EFI_NOT_FOUND: usize = 14 | ERROR_CODE_MASK;
/// Access was denied.
pub const EFI_ACCESS_DENIED: usize = 15 | ERROR_CODE_MASK;
/// The server was not found or did not respond to the request.
pub const EFI_NO_RESPONSE: usize = 16 | ERROR_CODE_MASK;
/// A mapping to a device does not exist.
pub const EFI_NO_MAPPING: usize = 17 | ERROR_CODE_MASK;
/// The timeout time expired.
pub const EFI_TIMEOUT: usize = 18 | ERROR_CODE_MASK;
/// The protocol has not been started.
pub const EFI_NOT_STARTED: usize = 19 | ERROR_CODE_MASK;
/// The protocol has already been started.
pub const EFI_ALREADY_STARTED: usize = 20 | ERROR_CODE_MASK;
/// The operation was aborted.
pub const EFI_ABORTED: usize = 21 | ERROR_CODE_MASK;
/// An ICMP error occurred during the network operation.
pub const EFI_ICMP_ERROR: usize = 22 | ERROR_CODE_MASK;
/// A TFTP error occurred during the network operation.
pub const EFI_TFTP_ERROR: usize = 23 | ERROR_CODE_MASK;
/// A protocol error occurred during the network operation.
pub const EFI_PROTOCOL_ERROR: usize = 24 | ERROR_CODE_MASK;
/// The function encountered an internal version that was incompatible with a version requested by the caller.
pub const EFI_INCOMPATIBLE_VERSION: usize = 25 | ERROR_CODE_MASK;
/// The function was not performed due to a security violation.
pub const EFI_SECURITY_VIOLATION: usize = 26 | ERROR_CODE_MASK;
/// A CRC error was detected.
pub const EFI_CRC_ERROR: usize = 27 | ERROR_CODE_MASK;
/// Beginning or end of media was reached
pub const EFI_END_OF_MEDIA: usize = 28 | ERROR_CODE_MASK;
/// The end of the file was reached.
pub const EFI_END_OF_FILE: usize = 31 | ERROR_CODE_MASK;
/// The language specified was invalid.
pub const EFI_INVALID_LANGUAGE: usize = 32 | ERROR_CODE_MASK;
/// The security status of the data is unknown or compromised and the data must be updated or
/// replaced to restore a valid security status.
pub const EFI_COMPROMISED_DATA: usize = 33 | ERROR_CODE_MASK;
/// There is an address conflict address allocation
pub const EFI_IP_ADDRESS_CONFLICT: usize = 34 | ERROR_CODE_MASK;
/// A HTTP error occurred during the network operation.
pub const EFI_HTTP_ERROR: usize = 35 | ERROR_CODE_MASK;

// The following are warning codes and the High Bit for them is clear

/// The string contained one or more characters that the device could not render and were skipped.
pub const EFI_WARN_UNKNOWN_GLYPH: usize = 1;
/// The handle was closed, but the file was not deleted.
pub const EFI_WARN_DELETE_FAILURE: usize = 2;
/// The handle was closed, but the data to the file was not flushed properly.
pub const EFI_WARN_WRITE_FAILURE: usize = 3;
/// The resulting buffer was too small, and the data was truncated to the buffer size.
pub const EFI_WARN_BUFFER_TOO_SMALL: usize = 4;
/// The data has not been updated within the timeframe set by local policy for this type of data.
pub const EFI_WARN_STALE_DATA: usize = 5;
/// The resulting buffer contains UEFI-compliant file system.
pub const EFI_WARN_FILE_SYSTEM: usize = 6;
/// The operation will be processed across a system reset.
pub const EFI_WARN_RESET_REQUIRED: usize = 7;
