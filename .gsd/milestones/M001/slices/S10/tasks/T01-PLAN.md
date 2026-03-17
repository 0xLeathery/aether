# T01: 08.1-fix-ci-cmake-build-errors 01

**Slice:** S10 — **Milestone:** M001

## Description

Switch from the broken `opus` crate (audiopus_sys 0.2.2, CMake < 3.5) to `opus-codec` (opus 1.5.2, CMake 3.16) to fix CI build failures on GitHub Actions macOS runners with CMake 4.x. Add a PR check CI workflow and Linux to the release matrix.

Purpose: Unblock the CI/CD release pipeline and prevent future dependency build regressions.
Output: Working builds on all platforms, PR check workflow, Linux in release matrix.

## Must-Haves

- [ ] "Tauri app builds successfully on macOS (ARM + x64) without CMake errors"
- [ ] "Tauri app builds successfully on Windows without CMake errors"
- [ ] "CI check workflow runs on PRs and pushes to main, performing full tauri build on macOS + Windows"
- [ ] "Release workflow includes Linux (x86_64) in the build matrix with proper system dependencies"
- [ ] "Voice codec still encodes/decodes Opus audio at 48kHz mono (no quality regression)"

## Files

- `src-tauri/Cargo.toml`
- `src-tauri/src/voice/codec.rs`
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
