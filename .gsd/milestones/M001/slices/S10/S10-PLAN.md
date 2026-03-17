# S10: Fix Ci Cmake Build Errors

**Goal:** Switch from the broken `opus` crate to `opus-codec` to fix CI build failures on GitHub Actions macOS runners with CMake 4.x. Add PR check workflow and Linux to the release matrix.
**Demo:** CI builds pass on all platforms; PR check workflow runs; Linux included in release matrix.

## Must-Haves


## Tasks

- [x] **T01: 08.1-fix-ci-cmake-build-errors 01** `est:4min`
  - Switch from the broken `opus` crate (audiopus_sys 0.2.2, CMake < 3.5) to `opus-codec` (opus 1.5.2, CMake 3.16) to fix CI build failures on GitHub Actions macOS runners with CMake 4.x. Add a PR check CI workflow and Linux to the release matrix.

Purpose: Unblock the CI/CD release pipeline and prevent future dependency build regressions.
Output: Working builds on all platforms, PR check workflow, Linux in release matrix.

## Files Likely Touched

- `src-tauri/Cargo.toml`
- `src-tauri/src/voice/codec.rs`
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
