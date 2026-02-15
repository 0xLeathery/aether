use std::env;
use std::process::Command;

/// Check if running as production build (not debug/dev mode)
#[cfg(target_os = "macos")]
fn is_production_build() -> bool {
    // Check TAURI_ENV_DEBUG environment variable
    // Default to production if not set or if set to "false"
    match env::var("TAURI_ENV_DEBUG") {
        Ok(val) => val != "true",
        Err(_) => true, // Not set = production
    }
}

/// Add the application executable to the keychain item's ACL (trusted apps list)
///
/// This eliminates password prompts when accessing keychain items in production builds.
/// On macOS in production mode, executes: security add-generic-password -U -s <service> -a <account> -T <exe_path>
/// On macOS in dev mode, skips ACL modification (dev builds have changing exe paths).
/// On non-macOS platforms, this is a no-op.
///
/// Failures are logged to stderr but do not propagate - the keychain item still works with prompts.
#[cfg(target_os = "macos")]
pub fn add_app_to_keychain_acl(service: &str, account: &str) {
    // Skip ACL modification in development builds
    if !is_production_build() {
        return;
    }

    // Get current executable path
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Warning: Failed to get executable path for keychain ACL: {}", e);
            return;
        }
    };

    // Convert path to string
    let exe_path_str = match exe_path.to_str() {
        Some(path) => path,
        None => {
            eprintln!("Warning: Executable path contains invalid UTF-8, cannot set keychain ACL");
            return;
        }
    };

    // Execute: security add-generic-password -U -s <service> -a <account> -T <exe_path>
    // -U = update existing item (required for post-creation ACL modification)
    // -T = add application to trusted apps list (specific executable only, not -A which allows any app)
    match Command::new("security")
        .arg("add-generic-password")
        .arg("-U")
        .arg("-s")
        .arg(service)
        .arg("-a")
        .arg(account)
        .arg("-T")
        .arg(exe_path_str)
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Warning: Failed to add app to keychain ACL (exit code {:?}): {}",
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to execute security command for keychain ACL: {}", e);
        }
    }
    // All failures are soft - return without error propagation
}

/// No-op implementation for non-macOS platforms
#[cfg(not(target_os = "macos"))]
pub fn add_app_to_keychain_acl(_service: &str, _account: &str) {
    // No keychain ACL modification needed on non-macOS platforms
}
