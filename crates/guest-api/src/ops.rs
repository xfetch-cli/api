//! Typed wrappers for the built-in host operations.
//!
//! These helpers encode request bodies as base64 so binary payloads survive
//! the JSON bridge, and decode responses back into owned Rust values.

use crate::error::HostCallError;
use crate::host::{OP_EXEC, OP_HTTP, OP_LOG, host_call};
use base64::Engine as _;
use serde_json::{Value, json};

/// An HTTP response returned by the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// The result of running an external process through the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecResult {
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// Performs an HTTP request subject to the manifest `http.allow` patterns.
///
/// `headers` entries are sent as given (duplicates preserved); `body` is
/// optional; `timeout_ms` is clamped by the guest deadline.
pub fn http_request(
    method: &str,
    url: &str,
    headers: &[(String, String)],
    body: Option<&[u8]>,
    timeout_ms: Option<u64>,
) -> Result<HttpResponse, HostCallError> {
    let headers: Vec<Value> = headers
        .iter()
        .map(|(name, value)| json!([name, value]))
        .collect();
    let args = json!({
        "method": method,
        "url": url,
        "headers": headers,
        "body_base64": body.map(encode),
        "timeout_ms": timeout_ms,
    });

    let value = host_call(OP_HTTP, &args)?;
    parse_http_response(&value)
}

/// Runs an external program subject to the manifest `exec.allow` patterns.
///
/// The child environment is empty except for `env` entries that the manifest
/// also allowlists. No shell is involved.
pub fn exec(
    program: &str,
    args: &[&str],
    stdin: Option<&[u8]>,
    env: &[(String, String)],
    timeout_ms: Option<u64>,
) -> Result<ExecResult, HostCallError> {
    let env: Value = env
        .iter()
        .map(|(name, value)| (name.clone(), Value::String(value.clone())))
        .collect();
    let request = json!({
        "program": program,
        "args": args,
        "stdin_base64": stdin.map(encode),
        "env": env,
        "timeout_ms": timeout_ms,
    });

    let value = host_call(OP_EXEC, &request)?;
    parse_exec_result(&value)
}

/// Writes a diagnostic line through the host log (stderr).
///
/// Logging is best-effort: failures are ignored so guests never abort because
/// a log line could not be delivered.
pub fn log(level: &str, message: &str) {
    let _ = host_call(OP_LOG, &json!({ "level": level, "message": message }));
}

/// Encodes bytes as standard base64.
fn encode(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

/// Decodes a base64 field from a host response.
fn decode(value: &Value, field: &str) -> Result<Vec<u8>, HostCallError> {
    match value.get(field).and_then(Value::as_str) {
        None => Ok(Vec::new()),
        Some(encoded) => base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|err| {
                HostCallError::malformed(format!("invalid base64 in '{}': {}", field, err))
            }),
    }
}

/// Converts the wire form into [`HttpResponse`].
fn parse_http_response(value: &Value) -> Result<HttpResponse, HostCallError> {
    let status = value
        .get("status")
        .and_then(Value::as_u64)
        .ok_or_else(|| HostCallError::malformed("http response is missing 'status'"))?;
    let headers = parse_header_pairs(value.get("headers"));
    let body = decode(value, "body_base64")?;

    Ok(HttpResponse {
        status: status as u16,
        headers,
        body,
    })
}

/// Converts the wire form into [`ExecResult`].
fn parse_exec_result(value: &Value) -> Result<ExecResult, HostCallError> {
    let code = value
        .get("code")
        .and_then(Value::as_i64)
        .ok_or_else(|| HostCallError::malformed("exec response is missing 'code'"))?;
    let stdout = decode(value, "stdout_base64")?;
    let stderr = decode(value, "stderr_base64")?;

    Ok(ExecResult {
        code: code as i32,
        stdout,
        stderr,
    })
}

/// Parses `[[name, value], ...]` header arrays.
fn parse_header_pairs(value: Option<&Value>) -> Vec<(String, String)> {
    value
        .and_then(Value::as_array)
        .map(|pairs| {
            pairs
                .iter()
                .filter_map(|pair| {
                    let pair = pair.as_array()?;
                    Some((
                        pair.first()?.as_str()?.to_string(),
                        pair.get(1)?.as_str()?.to_string(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_http_response_payload() {
        let value = json!({
            "status": 200,
            "headers": [["content-type", "text/plain"], ["x", "1"]],
            "body_base64": encode(b"hello"),
        });
        let response = parse_http_response(&value).expect("parse");
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.len(), 2);
        assert_eq!(response.body, b"hello");
    }

    #[test]
    fn parses_exec_payload() {
        let value = json!({
            "code": 0,
            "stdout_base64": encode(b"out"),
            "stderr_base64": encode(b"err"),
        });
        let result = parse_exec_result(&value).expect("parse");
        assert_eq!(result.code, 0);
        assert_eq!(result.stdout, b"out");
        assert_eq!(result.stderr, b"err");
    }

    #[test]
    fn missing_status_is_malformed() {
        assert!(parse_http_response(&json!({})).is_err());
    }
}
