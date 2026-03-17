# T01: 03.1-fix-keychain-password-prompts 01

**Slice:** S04 — **Milestone:** M001

## Description

Eliminate macOS keychain password prompts in production builds by adding post-creation ACL modification via the macOS `security` command-line tool.

Purpose: keyring crate creates keychain items without adding the application to the Access Control List (ACL). macOS prompts for password when untrusted apps access keychain items. This fix adds the app executable to the trusted apps list after creating keychain entries, eliminating prompts in production while accepting them in development (where executable paths change on every rebuild).

Output: Modified identity storage module that automatically configures keychain ACL for production builds on macOS.

## Must-Haves

- [ ] "Production builds modify keychain ACL after storing secret key, adding the app executable to the trusted apps list"
- [ ] "Production builds modify keychain ACL after storing display name, adding the app executable to the trusted apps list"
- [ ] "Development builds skip ACL modification entirely (no security CLI calls)"
- [ ] "ACL modification failure logs a warning but does not fail identity creation"
- [ ] "ACL code only compiles on macOS (conditional compilation with cfg)"
- [ ] "Non-macOS platforms get a no-op implementation"

## Files

- `src-tauri/src/identity/keychain_acl.rs`
- `src-tauri/src/identity/storage.rs`
- `src-tauri/src/identity/display.rs`
- `src-tauri/src/identity/mod.rs`
