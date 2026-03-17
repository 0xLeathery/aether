# Phase 08.1: Fix CI CMake Build Errors - Research

**Researched:** 2026-02-17
**Domain:** Rust Opus codec dependency + CI/CD pipeline
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Proper dependency fix, not a quick workaround — eliminate the root cause
- Comfortable switching opus crates entirely if a better-maintained alternative exists, even if voice pipeline code needs updates
- Add a CI check workflow that runs on PRs (not just release tags) to catch dependency issues early
- PR check should run a full `cargo build` (not just `cargo check`) to catch native/C dependency issues
- Fix all platforms: macOS (ARM + x64), Windows, and Linux
- Add Linux to the CI matrix (currently missing from the release workflow)
- PR check workflow should build on macOS + Windows (full `tauri build`)
- Goal: get the entire build matrix green across all platforms
- Priority order: 1) Switch to a maintained opus crate, 2) Fall back to forking audiopus_sys
- Must use opus codec specifically — match current audio quality, don't downgrade
- If forking is necessary, a `[patch]` override in Cargo.toml pointing to a fixed fork is acceptable
- Research should evaluate available opus crates for active maintenance and clean CI builds

### Claude's Discretion
- Exact opus crate selection (based on research into what's maintained and builds cleanly)
- Linux CI runner configuration and system dependencies
- PR workflow trigger conditions (push vs PR, branch filters)
- Whether to cache Rust builds in CI for speed

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope
</user_constraints>

## Summary

The CI build failure is caused by `audiopus_sys 0.2.2`, which bundles libopus 1.3 with a `CMakeLists.txt` specifying `cmake_minimum_required` below version 3.5. CMake 4.0+ (rolled out to GitHub Actions macOS runners in September 2025) removed backward compatibility with versions below 3.5, breaking the build.

The dependency chain is: `opus 0.3.1` -> `audiopus_sys 0.2.2` -> `cmake 0.1.57` -> bundled opus 1.3 (broken CMakeLists.txt). The `audiopus_sys` crate is effectively unmaintained (open issue #22 asks about maintenance status, no fix merged for #21 CMake issue). The `opus` crate maintainer (SpaceManiac) updated to 0.3.1 in Jan 2026 but it still depends on `audiopus_sys ^0.2.0`.

The best fix is to switch from the `opus` crate to `opus-codec 0.1.2`, which bundles opus 1.5.2 (uses `cmake_minimum_required(VERSION 3.16)`, fully compatible with CMake 4.x). The API migration is straightforward — only `codec.rs` needs changes, and the method signatures are nearly identical.

**Primary recommendation:** Switch from `opus = "0.3"` to `opus-codec = "0.1"` in Cargo.toml and update `src-tauri/src/voice/codec.rs` to use the new type names. Add a PR check workflow and Linux to the build matrix.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `opus-codec` | 0.1.2 | Opus encode/decode | Bundles opus 1.5.2, CMake-compatible, actively maintained (Dec 2025), safe Rust API |
| `tauri-apps/tauri-action` | v0 | CI build action | Official Tauri v2 CI action for cross-platform builds |
| `swatinem/rust-cache` | v2 | CI build caching | Standard Rust CI cache, supports Tauri workspace layout |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `dtolnay/rust-toolchain` | stable | Rust installer | Already in use, standard for CI |
| `actions/setup-node` | v4 | Node.js installer | Already in use for frontend build |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `opus-codec` | `opus` + `[patch]` for audiopus_sys | Keeps current API but depends on amsam0's fork (git dep), less reliable long-term |
| `opus-codec` | `unsafe-libopus` | Pure Rust (no CMake at all), but 20% slower, only 11K downloads, transpiled unsafe code |
| `opus-codec` | Fork audiopus_sys ourselves | Maintenance burden, git dependency in Cargo.toml |

**Installation:**
```toml
# In src-tauri/Cargo.toml, replace:
# opus = "0.3"
# With:
opus-codec = "0.1"
```

## Architecture Patterns

### Recommended Project Structure
```
.github/
├── workflows/
│   ├── release.yml      # Existing: builds on tag push, add Linux
│   └── ci.yml           # NEW: PR check workflow
src-tauri/
├── Cargo.toml           # Switch opus -> opus-codec
└── src/voice/
    └── codec.rs         # Update imports and type names
```

### Pattern 1: opus-codec API Migration
**What:** The `opus-codec` crate uses typed enums instead of raw integers for sample rate
**When to use:** All Encoder/Decoder construction
**Example:**
```rust
// OLD (opus 0.3.1):
use opus::{Application, Channels, Decoder, Encoder};
let encoder = Encoder::new(48000, Channels::Mono, Application::Voip)?;
let decoder = Decoder::new(48000, Channels::Mono)?;

// NEW (opus-codec 0.1.2):
use opus_codec::{Application, Channels, Decoder, Encoder, SampleRate};
let encoder = Encoder::new(SampleRate::Hz48000, Channels::Mono, Application::Voip)?;
let decoder = Decoder::new(SampleRate::Hz48000, Channels::Mono)?;
```
Source: https://docs.rs/opus-codec/0.1.2/opus_codec/

### Pattern 2: encode_float / decode_float (UNCHANGED)
**What:** The encode/decode method signatures are identical between crates
**When to use:** No changes needed for encode/decode calls
**Example:**
```rust
// Both crates use the same signatures:
fn encode_float(&mut self, input: &[f32], output: &mut [u8]) -> Result<usize>
fn decode_float(&mut self, input: &[u8], output: &mut [f32], fec: bool) -> Result<usize>
```

### Pattern 3: Error Type Migration
**What:** `opus-codec` uses its own `opus_codec::Error` type instead of `opus::Error`
**When to use:** The current code already maps errors to `VoiceError::CodecError(String)` via `.map_err()`, so the error type change is transparent
**Example:**
```rust
// Current code already uses string mapping — no change needed:
.map_err(|e| VoiceError::CodecError(format!("Failed to create encoder: {}", e)))?;
```

### Pattern 4: Tauri v2 CI Workflow
**What:** Official Tauri v2 cross-platform CI pattern
**When to use:** PR check and release workflows
**Example:**
```yaml
# Linux dependencies for Tauri v2:
- name: Install dependencies (Ubuntu)
  if: matrix.platform == 'ubuntu-22.04'
  run: |
    sudo apt-get update
    sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```
Source: https://v2.tauri.app/distribute/pipelines/github/

### Anti-Patterns to Avoid
- **Setting CMAKE_POLICY_VERSION_MINIMUM=3.5:** This is the hacky workaround CMake suggests. It papers over the problem and will break again when CMake drops further compatibility.
- **Pinning CMake version in CI:** Installing an older CMake just to build one dependency creates fragile CI that hides real issues.
- **Using `system-lib` feature with pkg-config:** While `opus-codec` supports linking system libopus, bundled builds are more reliable for CI (no system package installation needed on macOS/Windows).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Opus bindings | Custom FFI bindings to libopus | `opus-codec` crate | Correctly bundles opus 1.5.2, handles all platform build nuances |
| CI workflow | Custom build scripts | `tauri-apps/tauri-action@v0` or direct `npx tauri build` | Handles platform-specific bundling, signing, artifact collection |
| Rust build cache | Manual cache keys | `swatinem/rust-cache@v2` | Smart key generation, workspace support, automatic cleanup |

**Key insight:** The entire problem exists because `audiopus_sys` bundled an old version of opus C source. Using a crate that bundles a current opus version eliminates the root cause.

## Common Pitfalls

### Pitfall 1: CMake Version on CI Runners
**What goes wrong:** Build fails because CMake 4.x rejects `cmake_minimum_required` below 3.5
**Why it happens:** GitHub Actions macOS runners upgraded to CMake 4.x in September 2025 (issue #12934)
**How to avoid:** Use `opus-codec` which bundles opus 1.5.2 (requires CMake 3.16, compatible with 4.x)
**Warning signs:** Error message: "Compatibility with CMake < 3.5 has been removed from CMake" at `audiopus_sys-0.2.2/opus/CMakeLists.txt:1`

### Pitfall 2: Linux Missing System Dependencies
**What goes wrong:** Tauri build fails on Linux with missing headers
**Why it happens:** Ubuntu runners don't have WebKit2GTK and other Tauri dependencies pre-installed
**How to avoid:** Install `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf` and also `cmake` for opus-codec build
**Warning signs:** `pkg-config` errors, missing header errors during compilation

### Pitfall 3: opus-codec Import Path
**What goes wrong:** Code fails to compile after dependency switch
**Why it happens:** The crate name `opus-codec` becomes `opus_codec` in Rust imports (hyphens become underscores)
**How to avoid:** Use `use opus_codec::{...}` not `use opus::{...}`
**Warning signs:** `unresolved import` compiler errors

### Pitfall 4: SampleRate Type Mismatch
**What goes wrong:** Encoder/Decoder construction fails to compile
**Why it happens:** `opus-codec` uses `SampleRate::Hz48000` enum instead of `u32` literal `48000`
**How to avoid:** Replace `48000` with `SampleRate::Hz48000` and add `SampleRate` to imports
**Warning signs:** Type mismatch errors on `Encoder::new()` / `Decoder::new()`

### Pitfall 5: Rust Cache Key Conflicts
**What goes wrong:** CI cache from one platform is used on another, causing build failures
**Why it happens:** Incorrect cache key configuration
**How to avoid:** `swatinem/rust-cache@v2` handles this automatically with platform-aware keys when used with `workspaces: './src-tauri -> target'`
**Warning signs:** Strange compile errors that only happen in CI, not locally

## Code Examples

Verified patterns from official sources:

### Migration: codec.rs Imports
```rust
// BEFORE (opus 0.3.1):
use opus::{Application, Channels, Decoder, Encoder};

// AFTER (opus-codec 0.1.2):
use opus_codec::{Application, Channels, Decoder, Encoder, SampleRate};
```

### Migration: Encoder Construction
```rust
// BEFORE:
let encoder = Encoder::new(SAMPLE_RATE, Channels::Mono, Application::Voip)
    .map_err(|e| VoiceError::CodecError(format!("Failed to create encoder: {}", e)))?;

// AFTER (SAMPLE_RATE is u32 = 48000, need SampleRate enum):
let encoder = Encoder::new(SampleRate::Hz48000, Channels::Mono, Application::Voip)
    .map_err(|e| VoiceError::CodecError(format!("Failed to create encoder: {}", e)))?;
```

### Migration: Decoder Construction
```rust
// BEFORE:
let decoder = Decoder::new(SAMPLE_RATE, Channels::Mono)
    .map_err(|e| VoiceError::CodecError(format!("Failed to create decoder: {}", e)))?;

// AFTER:
let decoder = Decoder::new(SampleRate::Hz48000, Channels::Mono)
    .map_err(|e| VoiceError::CodecError(format!("Failed to create decoder: {}", e)))?;
```

### encode_float / decode_float (NO CHANGE NEEDED)
```rust
// Both crates have identical signatures:
encoder.encode_float(pcm, &mut output)?;  // Returns usize
decoder.decode_float(data, &mut output, false)?;  // Returns usize
```

### PR Check Workflow (.github/workflows/ci.yml)
```yaml
name: CI

on:
  pull_request:
    branches: [main]
  push:
    branches: [main]

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: macos-latest
            target: aarch64-apple-darwin
            label: macOS-arm64
          - platform: macos-latest
            target: x86_64-apple-darwin
            label: macOS-x64
          - platform: windows-latest
            target: x86_64-pc-windows-msvc
            label: Windows-x64

    runs-on: ${{ matrix.platform }}
    name: Build (${{ matrix.label }})

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Rust cache
        uses: swatinem/rust-cache@v2
        with:
          workspaces: './src-tauri -> target'

      - name: Install Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install frontend dependencies
        run: npm install

      - name: Build Tauri app
        run: npx tauri build --target ${{ matrix.target }}
```
Source: https://v2.tauri.app/distribute/pipelines/github/

### Linux Entry in Release Workflow
```yaml
# Add to release.yml matrix:
- platform: ubuntu-22.04
  target: x86_64-unknown-linux-gnu
  label: Linux-x64

# Add step before build:
- name: Install dependencies (Ubuntu)
  if: matrix.platform == 'ubuntu-22.04'
  run: |
    sudo apt-get update
    sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `opus` crate (audiopus_sys 0.2.2, opus 1.3) | `opus-codec` (bundled opus 1.5.2) | Dec 2025 | Fixes CMake 4.x compatibility |
| CMake < 3.5 minimum | CMake 3.16 minimum (opus 1.5.2) | opus 1.5.x release | Compatible with CMake 4.x |
| macOS CMake 3.31.6 pinned | macOS CMake 4.x | Sep 8, 2025 (GitHub Actions) | Breaks old audiopus_sys |
| `macos-latest` = macOS 14 | `macos-latest` = macOS 15 | Aug 2025 (GitHub Actions) | New runner image |

**Deprecated/outdated:**
- `audiopus_sys 0.2.2`: Unmaintained, bundles opus 1.3 with broken CMake, open issue since Jun 2025
- `opus 0.3.1`: Still depends on broken `audiopus_sys ^0.2.0`, no fix planned
- `audiopus 0.3.0-rc.0`: Last updated Apr 2021, same underlying `audiopus_sys` issue

## Opus Crate Evaluation

### Recommendation: `opus-codec 0.1.2`

| Criteria | `opus` 0.3.1 | `opus-codec` 0.1.2 | `unsafe-libopus` 0.1.3 |
|----------|--------------|---------------------|------------------------|
| Bundled opus version | 1.3 (broken) | **1.5.2** (fixed) | 1.3.1 (transpiled) |
| CMake 4.x compatible | NO | **YES** | N/A (no CMake) |
| Last updated | Jan 2026 | **Dec 2025** | Nov 2024 |
| Downloads | 839K | 162 | 11.5K |
| API style | `u32` sample rate | `SampleRate` enum | Raw FFI |
| Build system | CMake (broken) | CMake (working) | None (pure Rust) |
| Opus quality | 1.3 | **1.5.2** (latest) | 1.3.1 |
| Safe API | Yes | **Yes** | No (unsafe) |
| Advanced config | Limited | **Full CTL access** | Raw FFI |

**Why `opus-codec`:** It bundles the latest opus (1.5.2), builds cleanly with CMake 4.x, exposes a safe API with full configuration access (bitrate, complexity, VBR, DTX), and the API migration from `opus` is minimal (only type name changes, no logic changes).

**Low download count is not a concern** because: (1) the crate is new (Dec 2025), (2) it wraps the same battle-tested libopus C library, (3) the Rust wrapper is straightforward FFI with tests validated against ffmpeg.

## Migration Scope Assessment

### Files Requiring Changes
| File | Change | Complexity |
|------|--------|------------|
| `src-tauri/Cargo.toml` | Replace `opus = "0.3"` with `opus-codec = "0.1"` | Trivial |
| `src-tauri/src/voice/codec.rs` | Update imports, change `SAMPLE_RATE` to `SampleRate::Hz48000` in constructors | Small |
| `.github/workflows/release.yml` | Add Linux to matrix, add Ubuntu dependencies step | Small |
| `.github/workflows/ci.yml` | New file: PR check workflow | Medium (new file) |

### Files NOT Requiring Changes
- `src-tauri/src/voice/session.rs` — Uses `VoiceEncoder`/`VoiceDecoder` wrappers, not opus directly
- `src-tauri/src/voice/protocol.rs` — Works with raw bytes, no opus dependency
- `src-tauri/src/voice/mod.rs` — Re-exports won't change
- `src-tauri/src/voice/mixer.rs` — PCM-level, no codec involvement
- `src-tauri/src/voice/capture.rs` — cpal capture, no codec
- `src-tauri/src/voice/playback.rs` — cpal playback, no codec

## Open Questions

1. **CMake on Windows CI**
   - What we know: Current release.yml uses `lukka/get-cmake@latest` for Windows; opus-codec uses cmake crate
   - What's unclear: Whether Windows runners have CMake pre-installed now (they may), making the step unnecessary
   - Recommendation: Keep the `lukka/get-cmake@latest` step for Windows as a safety measure, test removing it later

2. **opus-codec Error Type Compatibility**
   - What we know: Current code maps opus errors to strings via `format!()`, so the exact error type shouldn't matter
   - What's unclear: Whether `opus_codec::Error` implements `Display` (needed for `format!()`)
   - Recommendation: LOW risk — standard Rust error types implement Display; verify during build

3. **Linux Audio Dependencies in CI**
   - What we know: Tauri needs WebKit/GTK deps on Linux. ALSA/PulseAudio may be needed for cpal.
   - What's unclear: Whether ubuntu-22.04 has ALSA dev headers pre-installed
   - Recommendation: May need `libasound2-dev` for cpal on Linux; test and add if needed

## Sources

### Primary (HIGH confidence)
- https://docs.rs/opus-codec/0.1.2/opus_codec/ - API documentation, types, method signatures
- https://github.com/Deniskore/opus-codec - Source, build.rs, bundled opus 1.5.2
- https://github.com/xiph/opus/blob/v1.5.2/CMakeLists.txt - Confirms `cmake_minimum_required(VERSION 3.16)`
- https://v2.tauri.app/distribute/pipelines/github/ - Official Tauri v2 CI workflow patterns
- https://github.com/Lakelezz/audiopus_sys/issues/21 - Root cause CMake issue documented
- https://github.com/actions/runner-images/issues/12934 - CMake 4.x rollout to macOS runners

### Secondary (MEDIUM confidence)
- https://github.com/amsam0/audiopus_sys - Fork with opus 1.5.2 (alternative if opus-codec doesn't work)
- https://github.com/Swatinem/rust-cache - Rust CI cache action docs
- https://crates.io/api/v1/crates/opus - Download stats, last updated info
- https://crates.io/api/v1/crates/opus-codec - Crate metadata, version info

### Tertiary (LOW confidence)
- https://github.com/Lakelezz/audiopus_sys/issues/22 - Maintenance status question (no response from maintainer)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - opus-codec API verified via docs.rs, opus 1.5.2 CMakeLists.txt verified, migration path mapped
- Architecture: HIGH - Only codec.rs needs changes, CI workflow patterns verified from official Tauri docs
- Pitfalls: HIGH - Root cause fully understood (CMake 4.x + opus 1.3 cmake_minimum_required), migration risks are minimal

**Research date:** 2026-02-17
**Valid until:** 2026-03-17 (stable domain, opus-codec unlikely to break)