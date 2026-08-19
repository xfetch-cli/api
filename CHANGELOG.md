# Changelog

## 2026-08-19
- Added `with_timeout(budget, task)` to `xfetch-plugin-api` and `xfetch-extension-api`. Plugins and extensions declare their own runtime budget in code; the task runs on a worker thread and `Err(TimedOut)` is returned when the budget elapses, so a hung plugin/extension can never hang xfetch.
