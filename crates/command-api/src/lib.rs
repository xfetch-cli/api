//! Shared command protocol types and IO helpers for xfetch command addons.
//!
//! Command addons receive a JSON request and return plain text. Native
//! commands may also use process arguments; wasm commands read the JSON
//! request from stdin (or from the component `run` request string).

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{Read, Write};

pub use xfetch_api_common::with_timeout;

/// Current JSON command protocol version.
pub const COMMAND_PROTOCOL_VERSION: u32 = 1;

/// Request kind used by command addons.
pub const KIND_COMMAND: &str = "command";

/// One command invocation sent by the core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default = "default_kind")]
    pub kind: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

impl CommandRequest {
    pub fn is_current(&self) -> bool {
        self.version == COMMAND_PROTOCOL_VERSION && self.kind == KIND_COMMAND
    }
}

fn default_version() -> u32 {
    COMMAND_PROTOCOL_VERSION
}

fn default_kind() -> String {
    KIND_COMMAND.to_string()
}

/// Reads one command request from stdin.
pub fn read_command_request() -> Result<CommandRequest, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|err| format!("failed to read command request: {err}"))?;
    parse_command_request(&input)
}

/// Parses one command request from a string.
pub fn parse_command_request(input: &str) -> Result<CommandRequest, String> {
    let request: CommandRequest =
        serde_json::from_str(input).map_err(|err| format!("invalid command request: {err}"))?;
    if !request.is_current() {
        return Err(format!(
            "unsupported command request version/kind: {}/{}",
            request.version, request.kind
        ));
    }
    Ok(request)
}

/// Reads the request and returns its arguments, or an empty list when stdin is
/// empty. This is convenient for commands that only need positional arguments.
pub fn read_command_args_or_default() -> Result<Vec<String>, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|err| format!("failed to read command request: {err}"))?;
    if input.trim().is_empty() {
        return Ok(Vec::new());
    }
    Ok(parse_command_request(&input)?.args)
}

/// Writes plain text to stdout. The caller is responsible for adding newlines.
pub fn write_command_text(text: &str) -> Result<(), String> {
    let mut stdout = std::io::stdout();
    stdout
        .write_all(text.as_bytes())
        .map_err(|err| format!("failed to write command output: {err}"))?;
    stdout
        .flush()
        .map_err(|err| format!("failed to flush command output: {err}"))
}

/// Formats one line per item with a trailing newline.
pub fn format_lines(lines: &[String]) -> String {
    let mut text = lines.join("\n");
    if !text.is_empty() {
        text.push('\n');
    }
    text
}

/// Writes one line per item, terminated by a newline.
pub fn write_command_lines(lines: &[String]) -> Result<(), String> {
    write_command_text(&format_lines(lines))
}

/// Parses JSON with a command-oriented error message.
pub fn parse_json<T: DeserializeOwned>(input: &str) -> Result<T, String> {
    serde_json::from_str(input).map_err(|err| format!("invalid JSON: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_command_request() {
        let request = parse_command_request(
            r#"{"version":1,"kind":"command","command":"github","args":["owner/repo"]}"#,
        )
        .expect("request");
        assert_eq!(request.command, "github");
        assert_eq!(request.args, vec!["owner/repo"]);
    }

    #[test]
    fn rejects_unknown_kinds() {
        let error = parse_command_request(r#"{"version":1,"kind":"effect"}"#).unwrap_err();
        assert!(error.contains("unsupported"));
    }

    #[test]
    fn formats_lines_with_a_trailing_newline() {
        assert_eq!(
            format_lines(&["repo: x".to_string(), "author: y".to_string()]),
            "repo: x\nauthor: y\n"
        );
        assert_eq!(format_lines(&[]), "");
    }
}
