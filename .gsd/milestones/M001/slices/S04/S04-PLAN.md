# S04: Fix Keychain Password Prompts

**Goal:** Eliminate macOS keychain password prompts in production builds by adding post-creation ACL modification via the macOS `security` command-line tool.
**Demo:** Eliminate macOS keychain password prompts in production builds by adding post-creation ACL modification via the macOS `security` command-line tool.

## Must-Haves


## Tasks

- [x] **T01: 03.1-fix-keychain-password-prompts 01** `est:14min`
  - Eliminate macOS keychain password prompts in production builds by adding post-creation ACL modification via the macOS `security` command-line tool.

Purpose: keyring crate creates keychain items without adding the application to the Access Control List (ACL). macOS prompts for password when untrusted apps access keychain items. This fix adds the app executable to the trusted apps list after creating keychain entries, eliminating prompts in production while accepting them in development (where executable paths change on every rebuild).

Output: Modified identity storage module that automatically configures keychain ACL for production builds on macOS.

## Files Likely Touched

- `src-tauri/src/identity/keychain_acl.rs`
- `src-tauri/src/identity/storage.rs`
- `src-tauri/src/identity/display.rs`
- `src-tauri/src/identity/mod.rs`
