//! Runtime budgets for addons.
//!
//! Native addons declare their own budget in code; the helper runs the task on
//! a worker thread and returns [`TimedOut`] when the budget elapses. Wasm
//! guests run single-threaded, so the host enforces the same deadline through
//! wasmtime epoch interruption.

#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;
use std::time::Duration;

/// The task did not finish within its budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimedOut;

#[cfg(not(target_arch = "wasm32"))]
pub fn with_timeout<T: Send + 'static>(
    budget: Duration,
    task: impl FnOnce() -> T + Send + 'static,
) -> Result<T, TimedOut> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(task());
    });
    rx.recv_timeout(budget).map_err(|_| TimedOut)
}

#[cfg(target_arch = "wasm32")]
pub fn with_timeout<T>(_budget: Duration, task: impl FnOnce() -> T) -> Result<T, TimedOut> {
    Ok(task())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_completing_within_budget_returns_value() {
        assert_eq!(with_timeout(Duration::from_secs(5), || 42u32), Ok(42));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn slow_task_times_out() {
        let started = std::time::Instant::now();
        let result = with_timeout(Duration::from_millis(50), || {
            std::thread::sleep(Duration::from_secs(5));
            1u32
        });

        assert_eq!(result, Err(TimedOut));
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "timeout must fire promptly, took {:?}",
            started.elapsed()
        );
    }
}
