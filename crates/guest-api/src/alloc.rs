//! Guest-side memory exports used by the host bridge.
//!
//! When a guest calls `host_call`, the host allocates the response buffer by
//! invoking the guest's `xfetch_alloc` export, writes the JSON bytes into it
//! and returns the packed pointer/length. The helper then parses the buffer
//! and releases it through [`free_response`].
//!
//! The exported functions are only defined on wasm targets; native builds
//! compile the crate as a stub for tests and tooling.

#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::alloc::{Layout, alloc, dealloc};

    /// Allocates `size` bytes inside guest memory. Returns a null pointer when
    /// the allocation fails; the host then returns `0` from `host_call`.
    #[unsafe(no_mangle)]
    pub extern "C" fn xfetch_alloc(size: usize) -> *mut u8 {
        if size == 0 {
            return std::ptr::null_mut();
        }
        // Alignment 1 keeps the layout always valid and matches what the host
        // assumes when writing JSON bytes.
        let layout = Layout::from_size_align(size, 1);
        match layout {
            Ok(layout) => unsafe { alloc(layout) },
            Err(_) => std::ptr::null_mut(),
        }
    }

    /// Releases a buffer previously returned by `xfetch_alloc`.
    #[unsafe(no_mangle)]
    pub extern "C" fn xfetch_free(ptr: *mut u8, size: usize) {
        if ptr.is_null() || size == 0 {
            return;
        }
        if let Ok(layout) = Layout::from_size_align(size, 1) {
            unsafe { dealloc(ptr, layout) };
        }
    }
}

/// Releases a host response buffer after it has been parsed.
#[cfg(target_arch = "wasm32")]
pub(crate) unsafe fn free_response(ptr: *mut u8, size: usize) {
    wasm::xfetch_free(ptr, size);
}

/// No-op on targets without the host bridge (kept for API symmetry with the
/// wasm build, where the host-call path releases the response buffer).
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) unsafe fn free_response(_ptr: *mut u8, _size: usize) {}
