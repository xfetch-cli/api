//! The raw host-call bridge.
//!
//! The host imports live in the `xfetch` module using the core wasm ABI:
//!
//! ```text
//! host_call(op_ptr, op_len, args_ptr, args_len) -> (len << 32) | ptr
//! ```
//!
//! The returned pointer refers to a buffer the host allocated inside guest
//! memory through the `xfetch_alloc` export. A return value of `0` means the
//! request could not be served (bad memory, allocation failure).

use crate::error::HostCallError;
use serde_json::Value;

/// Operation name for HTTP requests.
pub const OP_HTTP: &str = "http";
/// Operation name for process execution.
pub const OP_EXEC: &str = "exec";
/// Operation name for logging.
pub const OP_LOG: &str = "log";
/// Operation name for the runtime version handshake.
pub const OP_VERSION: &str = "version";

#[cfg(target_arch = "wasm32")]
mod abi {
    #[link(wasm_import_module = "xfetch")]
    unsafe extern "C" {
        pub fn host_call(
            op_ptr: *const u8,
            op_len: usize,
            args_ptr: *const u8,
            args_len: usize,
        ) -> u64;
    }
}

/// Dispatches one host operation with JSON arguments and returns its JSON
/// value.
///
/// Errors are typed: [`crate::HostErrorKind::Denied`] means the manifest did
/// not allow the operation, `Timeout` means the deadline was hit, and so on.
pub fn host_call(op: &str, args: &Value) -> Result<Value, HostCallError> {
    let args = serde_json::to_vec(args)
        .map_err(|err| HostCallError::malformed(format!("failed to encode args: {}", err)))?;
    dispatch(op.as_bytes(), &args)
}

/// Returns the host runtime protocol version.
pub fn protocol_version() -> Result<u32, HostCallError> {
    let value = host_call(OP_VERSION, &Value::Null)?;
    value
        .get("protocol")
        .and_then(Value::as_u64)
        .map(|version| version as u32)
        .ok_or_else(|| HostCallError::malformed("version response is missing 'protocol'"))
}

#[cfg(target_arch = "wasm32")]
fn dispatch(op: &[u8], args: &[u8]) -> Result<Value, HostCallError> {
    let packed = unsafe { abi::host_call(op.as_ptr(), op.len(), args.as_ptr(), args.len()) };
    if packed == 0 {
        return Err(HostCallError::malformed(
            "host returned no buffer (missing xfetch_alloc export?)",
        ));
    }

    let ptr = (packed & 0xffff_ffff) as *mut u8;
    let len = (packed >> 32) as usize;
    // SAFETY: the host guarantees `ptr..ptr+len` is valid guest memory it
    // allocated through `xfetch_alloc`.
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };

    let parsed = match serde_json::from_slice::<Value>(bytes) {
        Ok(value) => value,
        Err(err) => {
            unsafe { crate::alloc::free_response(ptr, len) };
            return Err(HostCallError::malformed(format!(
                "host response is not valid JSON: {}",
                err
            )));
        }
    };
    unsafe { crate::alloc::free_response(ptr, len) };

    if parsed.get("ok").and_then(Value::as_bool).unwrap_or(false) {
        Ok(parsed.get("value").cloned().unwrap_or(Value::Null))
    } else {
        Err(HostCallError::from_wire(&parsed))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn dispatch(_op: &[u8], _args: &[u8]) -> Result<Value, HostCallError> {
    Err(HostCallError::not_wasm())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_host_calls_report_unsupported() {
        let error = host_call(OP_HTTP, &Value::Null).expect_err("no host bridge");
        assert_eq!(error.kind, crate::HostErrorKind::Unsupported);
    }
}
