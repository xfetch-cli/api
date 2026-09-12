//! Host-call helpers for xfetch WebAssembly guests.
//!
//! Core-module guests (target `wasm32-wasip1`) cannot use the component
//! model's typed imports. xfetch therefore exposes a single JSON bridge
//! through the `xfetch` import module and this crate wraps it in a safe API:
//!
//! - [`host_call`] dispatches one operation and returns its JSON value.
//! - [`http_request`], [`exec`], [`log`] and [`protocol_version`] are typed
//!   helpers for the built-in operations.
//! - `xfetch_alloc` and `xfetch_free` are exported automatically so the host
//!   can place responses inside guest memory.
//!
//! Capabilities are deny-by-default and come from the guest manifest: `http`
//! and `exec` require allowlist entries, everything else is refused by the
//! host with [`HostErrorKind::Denied`]. See `xfetch/docs/WASM.md` for the
//! manifest schema.
//!
//! On non-wasm targets every call returns [`HostCallError`] with
//! [`HostErrorKind::Unsupported`] so the same guest source compiles for unit
//! tests and tooling without a wasm toolchain.
//!
//! # Example
//!
//! ```no_run
//! use xfetch_guest_api::{http_request, log, HostCallError};
//!
//! fn run() -> Result<(), HostCallError> {
//!     log("info", "fetching weather");
//!     let response = http_request("GET", "https://wttr.in/?format=3", &[], None, Some(5_000))?;
//!     let body = String::from_utf8_lossy(&response.body);
//!     log("info", &format!("status {} body {}", response.status, body));
//!     Ok(())
//! }
//! ```

mod alloc;
mod error;
mod host;
mod ops;

pub use error::{HostCallError, HostErrorKind};
pub use host::{host_call, protocol_version};
pub use ops::{ExecResult, HttpResponse, exec, http_request, log};

pub use serde_json::{Value, json};

/// Host runtime protocol version implemented by this crate.
pub const HOST_PROTOCOL_VERSION: u32 = 1;
