# Contributing Guide

## Recommended Editor Settings

### VSCode

[.vscode/settings.recommended.json](/.vscode/settings.recommended.json) contains recommended settings for VSCode. Copy to `.vscode/settings.json` to use these settings.

### Zed

[./zed/settings.recommended.json](/.zed/settings.recommended.json) contains recommended settings for Zed. Copy to `.zed/settings.json` to use these settings.

### Rust-Analyzer

If you use `rust-analyzer` in your editor you will want to enable the `ffi_uniffi` when working on the functions to be used over the FFI. Otherwise `rust-analyzer` will see all the code behind that feature as dead code since it is disable by default. You could also enable `ffi_wasm`, but it is generally easier to work with the UniFFI feature enabled instead. This is done in the above recommended VSCode and Zed settings.
