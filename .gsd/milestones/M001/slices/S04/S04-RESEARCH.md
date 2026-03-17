# Phase 3.1: Fix Keychain Password Prompts - Research

**Researched:** 2026-02-16
**Domain:** macOS Keychain Access Control Lists (ACLs) and secure credential storage
**Confidence:** HIGH

## Summary

The password prompts occur because keychain items created by the `keyring` crate don't include the application executable in the Access Control List (ACL). On macOS, when an application attempts to access a keychain item but isn't in the trusted applications list, the system prompts for the keychain password on every access for security reasons.

The solution is to use macOS's native `security` command-line tool to modify the ACL after creating keychain items. This approach leverages stable, well-tested infrastructure (the `keyring` crate + Apple's security tool) rather than attempting to use incomplete Rust bindings for ACL management.

**Primary recommendation:** Post-creation ACL modification via `security add-generic-password -U -T <exe_path>` in production builds. Accept password prompts in development mode (executable path changes on every rebuild).

## Problem Analysis

### Root Cause (HIGH Confidence)

Both `keyring` crate and `security-framework` crate create keychain items without adding the current application to the keychain item's access control list (ACL). This is confirmed by:

1. **Debug investigation** (file: `.planning/debug/keychain-password-prompts.md`): Documents complete root cause analysis with evidence from code, macOS Security framework documentation, and git history
2. **Current implementation**: `storage.rs` and `display.rs` use basic `Entry::new()` and `set_password()` without any ACL configuration
3. **Apple's security mechanism**: macOS requires password authentication when an untrusted app accesses a keychain item

### Why Previous Fix Failed (HIGH Confidence)

The `security-framework` approach (commit 9b403ef) was reverted twice (commits 4242c23 and d23d0d7) because it caused application crashes or instability. Analysis reveals:

1. **Incomplete Rust bindings**: The `security-framework` crate v3.x doesn't fully expose macOS ACL APIs. The `SecAccess` API is minimal and doesn't provide simple trusted app configuration.
2. **Wrong approach**: `ProtectionMode::AccessibleWhenUnlocked` controls WHEN items can be accessed (device locked vs unlocked), NOT WHO can access them (trusted applications).
3. **ACL vs Protection**: `SecAccessControl::create_with_protection()` sets data protection class, not the trusted applications list.

Sources:
- [Access Control Lists | Apple Developer Documentation](https://developer.apple.com/documentation/security/keychain_services/access_control_lists)
- [Any way to avoid 2 keychain prompts | Apple Developer Forums](https://developer.apple.com/forums/thread/649081)

## Standard Stack

### Current Implementation (No Changes Required)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| keyring | 3.6.3 | Cross-platform keychain access | Industry standard, stable API, well-maintained |
| keyring apple-native | feature flag | macOS platform-native backend | Uses Apple Security framework directly, supports iCloud sync |
| base64 | 0.22 | Binary data encoding | keyring stores strings, secret key is 32 bytes |

**No new dependencies required.** The solution uses macOS's built-in `security` command-line tool.

### Platform-Specific Mechanisms

| Platform | Credential Store | ACL Support | Notes |
|----------|------------------|-------------|-------|
| macOS | Keychain (Security framework) | Yes - via ACL trusted apps list | Requires `security` CLI or Security framework API |
| Windows | Windows Credential Manager | Partial - via CredentialAttribute | Different mechanism, not covered in this phase |
| Linux | Secret Service (GNOME/KDE) | Limited - application-based | Different mechanism, not covered in this phase |

**Cross-platform note:** ACL modification is macOS-specific. Windows and Linux have different credential access patterns that don't exhibit the same password prompt behavior.

Sources:
- [keyring crate documentation](https://docs.rs/keyring/latest/keyring/)
- [keyring windows module](https://docs.rs/keyring/latest/x86_64-pc-windows-msvc/keyring/windows/index.html)

## Architecture Patterns

### Pattern 1: Post-Creation ACL Modification (RECOMMENDED)

**What:** After creating a keychain item with `keyring`, call macOS `security` command to add the application executable to the ACL.

**When to use:** Production builds only. Skip in development mode to avoid complications with changing executable paths.

**Implementation location:**
- `src-tauri/src/identity/storage.rs::store_secret_key()` - after `entry.set_password()`
- `src-tauri/src/identity/display.rs::store_display_name()` - after `entry.set_password()`

**Example:**
```rust
use std::process::Command;
use std::path::PathBuf;

fn add_app_to_keychain_acl(service: &str, account: &str) -> Result<(), IdentityError> {
    // Get current executable path
    let exe_path = std::env::current_exe()
        .map_err(|e| IdentityError::KeychainDenied(format!("Failed to get exe path: {}", e)))?;

    // Call macOS security tool to update ACL
    let output = Command::new("security")
        .args(&[
            "add-generic-password",
            "-U",           // Update existing item
            "-s", service,  // Service name
            "-a", account,  // Account name
            "-T", exe_path.to_str().unwrap()  // Add this exe to trusted apps
        ])
        .output()
        .map_err(|e| IdentityError::KeychainDenied(format!("Failed to update ACL: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(IdentityError::KeychainDenied(format!("ACL update failed: {}", stderr)));
    }

    Ok(())
}
```

**Call sites:**
```rust
pub fn store_secret_key(secret_bytes: &[u8]) -> Result<(), IdentityError> {
    let entry = Entry::new(SERVICE_NAME, SECRET_KEY_USERNAME)?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(secret_bytes);
    entry.set_password(&encoded)?;

    // Add app to ACL (macOS only, production only)
    #[cfg(target_os = "macos")]
    if is_production_build() {
        add_app_to_keychain_acl(SERVICE_NAME, SECRET_KEY_USERNAME)?;
    }

    Ok(())
}
```

Sources:
- [security command documentation](https://ss64.com/mac/security.html)
- [Working with credentials in CLI on macOS](https://gist.github.com/tamakiii/9c3eadc493597ed819b9ff96cbcf61d4)

### Pattern 2: Development Mode Detection

**What:** Detect whether the app is running in development mode and skip ACL modification.

**When to use:** Every keychain write operation.

**Implementation:**
```rust
fn is_production_build() -> bool {
    // Tauri sets TAURI_ENV_DEBUG for dev and build --debug
    std::env::var("TAURI_ENV_DEBUG")
        .map(|v| v == "false" || v.is_empty())
        .unwrap_or(true)  // Assume production if var not set
}
```

**Why:** Development builds use different executable paths on every rebuild (`target/debug/aether`), so ACL entries would become stale immediately. Production builds have stable paths (`/Applications/Aether.app/Contents/MacOS/aether`).

Sources:
- [Environment Variables | Tauri](https://v2.tauri.app/reference/environment-variables/)
- [Debug | Tauri](https://v2.tauri.app/develop/debug/)

### Pattern 3: Conditional Compilation for Platform-Specific Code

**What:** Use `#[cfg(target_os = "macos")]` to ensure ACL code only compiles/runs on macOS.

**When to use:** All ACL-related functions.

**Example:**
```rust
#[cfg(target_os = "macos")]
fn add_app_to_keychain_acl(service: &str, account: &str) -> Result<(), IdentityError> {
    // macOS-specific implementation
}

#[cfg(not(target_os = "macos"))]
fn add_app_to_keychain_acl(_service: &str, _account: &str) -> Result<(), IdentityError> {
    // No-op on other platforms
    Ok(())
}
```

**Why:** Windows and Linux don't have the same ACL mechanism and don't need this code. Conditional compilation prevents unused code warnings and reduces binary size.

### Anti-Patterns to Avoid

- **Don't use `-A` flag**: `security add-generic-password -A` allows ANY application to access the item without warning. This is insecure and defeats the purpose of keychain security.
- **Don't modify ACL on every access**: Only modify ACL during item creation (`set_password`), not during reads (`get_password`). ACL is persistent.
- **Don't use `security-framework` for ACL management**: The Rust bindings are incomplete and caused crashes in previous attempts (reverted in commits 4242c23 and d23d0d7).
- **Don't store secrets in config files as workaround**: This loses iCloud sync and platform security guarantees.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Keychain ACL API bindings | Custom FFI to Security framework | `security` CLI tool via `std::process::Command` | Apple's CLI tool is stable, well-tested, and guaranteed to work. FFI bindings are incomplete and error-prone. |
| Cross-platform credential storage | Custom encryption + file storage | `keyring` crate | Handles platform differences, iCloud sync, OS security policies. Edge cases like keychain migration are handled. |
| Development mode detection | Parse executable paths or timestamps | `TAURI_ENV_DEBUG` environment variable | Tauri provides this officially. Path-based detection is brittle and unreliable. |

**Key insight:** The macOS Security framework is complex with subtle edge cases (code signing requirements, partition lists, designated requirements). Using Apple's own CLI tool delegates this complexity to tested code rather than attempting to reimplement it.

## Common Pitfalls

### Pitfall 1: Modifying ACL Requires User Authorization (CRITICAL)

**What goes wrong:** When the `security add-generic-password -U -T <path>` command modifies an existing keychain item's ACL, macOS will prompt the user for their password or Touch ID authorization.

**Why it happens:** macOS security policy prevents applications from silently granting themselves keychain access. Any ACL modification requires user consent.

**How to avoid:** This is unavoidable and is EXPECTED behavior. The prompt will appear once when the user first creates their identity. After that, the ACL is set and no more prompts occur.

**Warning signs:** User sees password prompt when running app for the first time or after reinstalling.

**Impact:** This is acceptable UX. The prompt explains "aether wants to modify the keychain item" which is accurate and expected.

Sources:
- [Apple Developer Forums - Access to keychain via Security](https://developer.apple.com/forums/thread/116579)
- [How to modify access control for private key](https://developer.apple.com/forums/thread/747366)

### Pitfall 2: Development Executable Paths Change on Every Build

**What goes wrong:** In development mode (`cargo tauri dev`), the executable is in `target/debug/aether`. Each rebuild may change the binary, making ACL entries stale.

**Why it happens:** Rust rebuilds binaries, and even if the path is the same, the binary hash changes, which can affect code signing-based ACL checks.

**How to avoid:** Skip ACL modification entirely in development mode. Accept password prompts during development.

**Warning signs:** Password prompts continue even after "fixing" ACL. ACL entries accumulate for the same path.

**Detection code:**
```rust
fn is_production_build() -> bool {
    std::env::var("TAURI_ENV_DEBUG")
        .map(|v| v == "false" || v.is_empty())
        .unwrap_or(true)
}
```

Sources:
- [Debug | Tauri](https://v2.tauri.app/develop/debug/)
- [Environment Variables | Tauri](https://v2.tauri.app/reference/environment-variables/)

### Pitfall 3: Production App Bundle Path vs Raw Executable

**What goes wrong:** After building and installing the app, `std::env::current_exe()` returns the path INSIDE the macOS app bundle (e.g., `/Applications/Aether.app/Contents/MacOS/aether`), not the bundle itself (`/Applications/Aether.app`).

**Why it happens:** macOS app bundles are directories that contain the executable at `Contents/MacOS/<name>`. The ACL must reference the actual executable, not the bundle.

**How to avoid:** Use `std::env::current_exe()` directly. It returns the correct executable path. Don't try to compute the bundle path.

**Warning signs:** ACL modification succeeds but prompts still occur. Keychain Access shows wrong application in ACL list.

**Verification:** After installation, open Keychain Access, find the `com.aether.identity` item, check "Access Control" tab. The executable path should match what `current_exe()` returns.

Sources:
- [macOS Application Bundle | Tauri](https://v2.tauri.app/distribute/macos-application-bundle/)
- [current_exe in std::env - Rust](https://doc.rust-lang.org/std/env/fn.current_exe.html)

### Pitfall 4: Code Signing and ACL Interaction

**What goes wrong:** If the application is not properly code signed, macOS may ignore ACL entries or prompt anyway due to security policies.

**Why it happens:** Modern macOS (10.12.5+) uses "partition lists" which are essentially ACLs based on code signing identities. An unsigned or ad-hoc signed app may not satisfy ACL requirements even if added to the trusted list.

**How to avoid:** For development, this is acceptable (prompts are expected). For production distribution, ensure the app is properly code signed with a Developer ID certificate.

**Warning signs:** ACL modification succeeds but prompts continue in production builds. Console logs show code signing verification failures.

**Future consideration:** If distributing via DMG or App Store, ensure code signing is configured. Phase 3.1 focuses on the ACL mechanism itself; proper distribution setup is a separate concern.

Sources:
- [macOS Code Signing at 20 - Michael Tsai Blog](https://mjtsai.com/blog/2026/01/20/mac-code-signing-at-20/)
- [macOS Code Signing | Tauri](https://v2.tauri.app/distribute/sign/macos/)
- [Scripting the macOS Keychain - Partition IDs](https://mostlikelee.com/blog-1/2017/9/16/scripting-the-macos-keychain-partition-ids)

### Pitfall 5: Error Handling for Missing `security` Command

**What goes wrong:** The `security` command might not be available (theoretically) or might fail in unexpected ways.

**Why it happens:** Command execution can fail for various reasons: command not found, permission denied, unexpected exit codes.

**How to avoid:** Wrap `Command::new("security").output()` in proper error handling. Check `output.status.success()` and capture stderr.

**Warning signs:** Application crashes on keychain operations instead of showing error message. Silent failures where ACL isn't set but no error is reported.

**Example error handling:**
```rust
let output = Command::new("security")
    .args(&[/* ... */])
    .output()
    .map_err(|e| IdentityError::KeychainDenied(format!("Failed to run security command: {}", e)))?;

if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(IdentityError::KeychainDenied(format!("security command failed: {}", stderr)));
}
```

### Pitfall 6: Multiple ACL Modifications Accumulate Prompts

**What goes wrong:** If ACL modification code is called on every `store_secret_key()` call (not just first creation), user gets prompted every time they update their display name or regenerate identity.

**Why it happens:** `-U` flag updates the keychain item AND modifies the ACL, triggering authorization prompt each time.

**How to avoid:** Only call ACL modification during initial creation, not on updates. Alternatively, check if ACL already includes the app before modifying (see Open Questions).

**Warning signs:** User reports repeated password prompts after initial setup. Multiple authorization dialogs during normal app operation.

**Potential solution:**
```rust
// Only modify ACL if this is a new item (not an update)
// This requires checking if item exists before calling set_password
// OR accepting one prompt per identity operation and documenting it
```

## Code Examples

### Complete Implementation Pattern

```rust
use std::process::Command;
use keyring::Entry;
use base64::Engine;

const SERVICE_NAME: &str = "com.aether.identity";
const SECRET_KEY_USERNAME: &str = "secret_key";
const DISPLAY_NAME_USERNAME: &str = "display_name";

/// Check if running in production build
#[cfg(target_os = "macos")]
fn is_production_build() -> bool {
    std::env::var("TAURI_ENV_DEBUG")
        .map(|v| v == "false" || v.is_empty())
        .unwrap_or(true)
}

/// Add current executable to keychain item's ACL (macOS only)
#[cfg(target_os = "macos")]
fn add_app_to_keychain_acl(service: &str, account: &str) -> Result<(), IdentityError> {
    let exe_path = std::env::current_exe()
        .map_err(|e| IdentityError::KeychainDenied(format!("Failed to get exe path: {}", e)))?;

    let output = Command::new("security")
        .args(&[
            "add-generic-password",
            "-U",  // Update existing item
            "-s", service,
            "-a", account,
            "-T", exe_path.to_str().ok_or_else(|| {
                IdentityError::KeychainDenied("Invalid executable path".to_string())
            })?
        ])
        .output()
        .map_err(|e| IdentityError::KeychainDenied(format!("Failed to run security command: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Don't fail if ACL update fails - keychain item still works, just prompts
        eprintln!("Warning: Failed to update keychain ACL: {}", stderr);
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn add_app_to_keychain_acl(_service: &str, _account: &str) -> Result<(), IdentityError> {
    Ok(())  // No-op on other platforms
}

/// Store secret key with ACL configuration
pub fn store_secret_key(secret_bytes: &[u8]) -> Result<(), IdentityError> {
    let entry = Entry::new(SERVICE_NAME, SECRET_KEY_USERNAME)
        .map_err(|e| IdentityError::KeychainDenied(format!("Failed to access keychain: {}", e)))?;

    let encoded = base64::engine::general_purpose::STANDARD.encode(secret_bytes);
    entry.set_password(&encoded)
        .map_err(|e| IdentityError::KeychainDenied(format!("Failed to store key: {}", e)))?;

    // Add app to ACL (macOS production builds only)
    #[cfg(target_os = "macos")]
    if is_production_build() {
        add_app_to_keychain_acl(SERVICE_NAME, SECRET_KEY_USERNAME)?;
    }

    Ok(())
}

/// Store display name with ACL configuration
pub fn store_display_name(name: &str) -> Result<(), IdentityError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(IdentityError::DisplayNameRequired);
    }

    let entry = Entry::new(SERVICE_NAME, DISPLAY_NAME_USERNAME)
        .map_err(|e| IdentityError::KeychainDenied(format!("Failed to access keychain: {}", e)))?;

    entry.set_password(trimmed)
        .map_err(|e| IdentityError::KeychainDenied(format!("Failed to store display name: {}", e)))?;

    // Add app to ACL (macOS production builds only)
    #[cfg(target_os = "macos")]
    if is_production_build() {
        add_app_to_keychain_acl(SERVICE_NAME, DISPLAY_NAME_USERNAME)?;
    }

    Ok(())
}
```

### Manual Verification Command

After implementing the fix, users or developers can manually verify the ACL configuration:

```bash
# Query keychain item and check access control
security find-generic-password -s "com.aether.identity" -a "secret_key" -g

# View in Keychain Access GUI
open -a "Keychain Access"
# Navigate to login keychain → Passwords → com.aether.identity
# Double-click item → Access Control tab
# Verify /Applications/Aether.app/Contents/MacOS/aether is in the list
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| keyring crate with no ACL config | keyring + post-creation ACL via security CLI | Phase 3.1 (2026-02-16) | Eliminates password prompts in production |
| security-framework Rust bindings | Native security command-line tool | Phase 3.1 | More stable, leverages Apple's tested code |
| Accept prompts as unavoidable | Fix prompts for production, accept in dev | Phase 3.1 | Better UX in production without compromising dev experience |

**Deprecated/outdated:**
- **security-framework approach**: Attempted in commit 9b403ef, reverted in 4242c23 and d23d0d7. Don't retry this approach without significant changes to the Rust bindings.
- **Global ACL policies**: Pre-10.12.5 macOS allowed simpler ACL management. Modern macOS uses partition lists based on code signing, requiring more sophisticated handling.

## Open Questions

### 1. Should ACL modification fail silently or return error?

**What we know:** ACL modification can fail (command not found, permission denied, etc.). Keychain item still works even if ACL update fails, just prompts for password.

**What's unclear:** Should this be a hard error (fail identity creation) or a soft warning (log but continue)?

**Recommendation:** Soft warning. Log failure to stderr but don't fail `store_secret_key()`. The keychain item is still functional, just requires password prompts.

**Confidence:** MEDIUM - depends on product decision about UX vs strictness.

### 2. Can we check if app is already in ACL before modifying?

**What we know:** We could potentially query the ACL first to avoid repeated prompts on every identity update.

**What's unclear:** How to query ACL programmatically. The `security` CLI has `find-generic-password` but ACL details may not be easily parseable.

**Recommendation:** For Phase 3.1, accept the authorization prompt on every ACL modification. Future optimization could check first.

**Confidence:** LOW - would require additional research into `security` CLI output parsing or Security framework API.

### 3. Does the fix work for app updates?

**What we know:** If the app executable is replaced during an update, the code signing hash changes. ACL might become invalid.

**What's unclear:** Whether ACL uses path-based or hash-based matching. If hash-based, updates would require re-authorization.

**Recommendation:** Test after implementing. If updates break ACL, may need to update keychain items during app update process.

**Confidence:** LOW - requires real-world testing with signed production builds and updates.

### 4. Should development builds skip ACL or use a flag?

**What we know:** Development builds have unstable executable paths. ACL modification in dev mode is problematic.

**What's unclear:** Should we use `TAURI_ENV_DEBUG` environment variable, or a command-line flag, or cargo feature?

**Recommendation:** Use `TAURI_ENV_DEBUG` environment variable. This is Tauri's official way to detect debug builds.

**Confidence:** HIGH - Tauri documentation confirms this is the standard approach.

### 5. Windows and Linux credential prompt behavior?

**What we know:** The `keyring` crate uses Windows Credential Manager and Linux Secret Service. These don't have the same ACL prompt behavior.

**What's unclear:** Do Windows or Linux exhibit any similar prompt issues that need addressing?

**Recommendation:** Phase 3.1 is macOS-only. If Windows/Linux have issues, address in separate phase after user reports.

**Confidence:** MEDIUM - preliminary research suggests Windows and Linux don't have this specific issue, but not definitively tested.

## Dev vs Production Strategy

### Development Mode (cargo tauri dev)

**Behavior:**
- Skip ACL modification entirely
- Accept password prompts on every app launch
- Prompt will typically appear once per keychain unlock (after Mac wakes up)

**Rationale:**
- Executable path is `target/debug/aether` which changes on rebuild
- ACL entries would accumulate and become stale
- Development experience is already slower (debugging), prompts are acceptable
- Avoids complexity and potential ACL-related bugs during development

**Implementation:**
```rust
#[cfg(target_os = "macos")]
if is_production_build() {
    add_app_to_keychain_acl(SERVICE_NAME, SECRET_KEY_USERNAME)?;
}
```

### Production Mode (cargo tauri build)

**Behavior:**
- Modify ACL after creating keychain items
- User sees authorization prompt ONCE when creating identity
- No prompts on subsequent app launches

**Rationale:**
- Executable path is stable (`/Applications/Aether.app/Contents/MacOS/aether`)
- Code signing identity is consistent across launches
- Provides seamless user experience for production use
- ACL modification is worth the one-time authorization prompt

**Implementation:**
Same code as above - `is_production_build()` returns `true` in release builds.

### Detection Logic (HIGH Confidence)

Tauri provides the `TAURI_ENV_DEBUG` environment variable:
- Set to `"true"` for `tauri dev` and `tauri build --debug`
- Set to `"false"` for `tauri build` (production)

```rust
fn is_production_build() -> bool {
    std::env::var("TAURI_ENV_DEBUG")
        .map(|v| v == "false" || v.is_empty())
        .unwrap_or(true)  // Default to production if env var not set
}
```

**Edge case:** If `TAURI_ENV_DEBUG` is not set (shouldn't happen in Tauri apps), assume production to ensure ACL is set for end users.

Sources:
- [Environment Variables | Tauri](https://v2.tauri.app/reference/environment-variables/)
- [Debug | Tauri](https://v2.tauri.app/develop/debug/)

## Cross-Platform Considerations

### macOS (Phase 3.1 Focus)

**Applies:** Full ACL modification implementation
**Mechanism:** Security framework via `security` CLI
**Status:** HIGH confidence solution

### Windows

**Applies:** No ACL modification needed
**Mechanism:** Windows Credential Manager uses different access control model
**Behavior:** Applications with the same executable name can typically access credentials without prompting
**Status:** No action required for Phase 3.1

Source: [keyring windows module documentation](https://docs.rs/keyring/latest/x86_64-pc-windows-msvc/keyring/windows/index.html)

### Linux

**Applies:** No ACL modification needed
**Mechanism:** Secret Service (GNOME Keyring, KDE Wallet) uses application-based access control
**Behavior:** Secret Service typically allows access based on D-Bus application identity, not executable path
**Status:** No action required for Phase 3.1

Source: [keyring crate documentation](https://docs.rs/keyring/latest/keyring/)

### Implementation Pattern

Use conditional compilation to ensure ACL code only exists on macOS:

```rust
#[cfg(target_os = "macos")]
fn add_app_to_keychain_acl(service: &str, account: &str) -> Result<(), IdentityError> {
    // macOS-specific implementation
}

#[cfg(not(target_os = "macos"))]
fn add_app_to_keychain_acl(_service: &str, _account: &str) -> Result<(), IdentityError> {
    Ok(())  // No-op on Windows and Linux
}
```

**Benefits:**
- Zero runtime overhead on Windows/Linux
- No need for runtime platform checks in hot paths
- Clear separation of platform-specific logic
- Prevents accidental execution of macOS-specific code on other platforms

## Testing Strategy

### 1. Development Build Testing

**Objective:** Verify that ACL modification is skipped in dev mode

**Steps:**
1. Run `npm run tauri dev`
2. Create a new identity (triggers `store_secret_key` and `store_display_name`)
3. Verify password prompt appears (expected behavior)
4. Restart the app
5. Verify password prompt appears again (confirms ACL was not modified)

**Expected result:** Password prompts on every launch. No errors in console.

### 2. Production Build Testing

**Objective:** Verify ACL modification eliminates prompts in production

**Steps:**
1. Run `npm run tauri build`
2. Install the built app to `/Applications/`
3. Launch the app
4. Create a new identity
5. Expect one authorization prompt: "aether wants to modify keychain item"
6. Approve the prompt (enter password or Touch ID)
7. Restart the app multiple times
8. Verify NO password prompts on subsequent launches

**Expected result:** One authorization prompt during setup, then no prompts on future launches.

### 3. ACL Verification

**Objective:** Confirm the app executable is in the keychain item's ACL

**Steps:**
1. After production build setup, open Keychain Access app
2. Navigate to "login" keychain → "Passwords" category
3. Find "com.aether.identity" items
4. Double-click "secret_key" item
5. Click "Access Control" tab
6. Verify `/Applications/Aether.app/Contents/MacOS/aether` is in the list

**Expected result:** App executable is listed in trusted applications.

### 4. Error Handling Testing

**Objective:** Verify graceful handling of ACL failures

**Test cases:**
- **security command not found**: Rename or move `security` binary (requires SIP disabled, test in VM)
- **Permission denied**: Test with restrictive security policies
- **Invalid executable path**: Mock `current_exe()` to return invalid path (unit test)

**Expected result:** Warning logged but identity creation succeeds. Prompts appear on access.

### 5. Cross-Platform Smoke Test

**Objective:** Ensure code compiles and runs on all platforms

**Steps:**
1. Build on macOS: `cargo build --target x86_64-apple-darwin`
2. Build on Windows: `cargo build --target x86_64-pc-windows-msvc` (if available)
3. Build on Linux: `cargo build --target x86_64-unknown-linux-gnu` (if available)
4. Verify no compilation errors
5. Run basic identity creation on each platform

**Expected result:** Compiles and runs on all platforms. ACL code only executes on macOS.

### 6. Update Scenario Testing (Future)

**Objective:** Verify ACL persists or can be updated after app updates

**Steps:**
1. Install version 1 of app
2. Create identity
3. Install version 2 of app (simulated update)
4. Launch version 2
5. Verify password prompt behavior

**Expected result:** TBD - depends on code signing and ACL hash vs path matching. Document findings.

**Note:** This is lower priority for Phase 3.1. Can be tested after initial implementation is verified.

## Sources

### Primary (HIGH confidence)

- [keyring crate v3.6.3](https://docs.rs/keyring/latest/keyring/) - Core library documentation
- [Tauri v2 Environment Variables](https://v2.tauri.app/reference/environment-variables/) - TAURI_ENV_DEBUG usage
- [Tauri v2 Debug Documentation](https://v2.tauri.app/develop/debug/) - Development vs production detection
- [macOS security command reference](https://ss64.com/mac/security.html) - Official command documentation
- [security add-generic-password flags](https://ss64.com/mac/security-password.html) - Detailed flag documentation
- [Rust std::env::current_exe](https://doc.rust-lang.org/std/env/fn.current_exe.html) - Executable path resolution
- Local debug investigation: `.planning/debug/keychain-password-prompts.md` - Complete root cause analysis

### Secondary (MEDIUM confidence)

- [Working with credentials in CLI on macOS](https://gist.github.com/tamakiii/9c3eadc493597ed819b9ff96cbcf61d4) - Practical command examples
- [Apple Developer Forums - Keychain Access](https://developer.apple.com/forums/thread/116579) - Authorization requirements
- [Apple Developer Forums - Keychain Prompts](https://developer.apple.com/forums/thread/649081) - Modern ACL mechanisms
- [macOS Application Bundle | Tauri](https://v2.tauri.app/distribute/macos-application-bundle/) - App bundle structure
- [macOS Code Signing | Tauri](https://v2.tauri.app/distribute/sign/macos/) - Code signing for distribution
- [macOS Code Signing at 20 - Michael Tsai](https://mjtsai.com/blog/2026/01/20/mac-code-signing-at-20/) - Historical context and evolution
- [Scripting the macOS Keychain - Partition IDs](https://mostlikelee.com/blog-1/2017/9/16/scripting-the-macos-keychain-partition-ids) - Modern ACL partition lists

### Tertiary (LOW confidence - flagged for validation)

- [Access Control Lists | Apple Developer Documentation](https://developer.apple.com/documentation/security/keychain_services/access_control_lists) - Could not fully access due to JavaScript requirement, but confirms ACL concepts
- [keyring windows module](https://docs.rs/keyring/latest/x86_64-pc-windows-msvc/keyring/windows/index.html) - Windows behavior (not directly tested)

## Metadata

**Confidence breakdown:**
- Root cause analysis: HIGH - Confirmed by debug investigation, git history, and macOS documentation
- Solution approach (security CLI): HIGH - Well-documented, recommended by Apple community, avoids failed security-framework approach
- Development vs production strategy: HIGH - Tauri provides official environment variable for detection
- Cross-platform considerations: MEDIUM - Windows/Linux behavior inferred from documentation, not directly tested
- Code signing implications: MEDIUM - Theory is sound, but real-world testing with signed apps needed
- ACL query mechanism: LOW - Would require additional research or experimentation

**Research date:** 2026-02-16
**Valid until:** 2026-03-16 (30 days - stable domain, macOS keychain mechanisms change slowly)

**Limitations:**
- Have not tested actual signed production builds (no Apple Developer certificate in current environment)
- Cross-platform testing limited to documentation review, not hands-on verification
- ACL query mechanism not fully researched (Open Question #2)
- App update scenario not tested (Open Question #3)