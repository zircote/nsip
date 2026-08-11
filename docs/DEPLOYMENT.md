---
id: nsip-docs-deployment
type: procedural
created: 2026-02-07T14:26:06-05:00
namespace: nsip/docs
modified: '2026-08-11T16:22:00.358Z'
title: "Deployment Guide"
diataxis_type: how-to
provenance:
  '@type': Provenance
  agent: claude-code/claude-sonnet-5
  wasGeneratedBy:
    '@id': urn:mif:activity:claude-code-session:f2ea9348-10db-44af-9ccb-a37844b8c1f2
    '@type': prov:Activity
  trustLevel: user_stated
  agentVersion: 2.1.227
---

# Deployment Guide

This document provides comprehensive deployment instructions for the nsip project.

## Overview

The project includes automated deployment workflows for:

- **GitHub Releases** - Multi-platform binaries
- **Docker** - Container images on GitHub Container Registry
- **crates.io** - Rust package registry

## Prerequisites

### Required Secrets

Configure these secrets in GitHub repository settings (Settings → Secrets and variables → Actions):

1. **crates.io Trusted Publishing** - publishing uses OIDC, not a stored token
   - Configure at: crates.io → crate Settings → Trusted Publishing
   - Repo `zircote/nsip`, workflow `publish.yml`, environment `copilot`

2. **GITHUB_TOKEN** - Automatically provided by GitHub Actions (no setup needed)

3. **HOMEBREW_TAP_TOKEN** - Fine-grained PAT with `contents:write` on `zircote/homebrew-tap`, stored in the `copilot` environment; used by `package-homebrew.yml` to push updated tap formulae

### GitHub Packages

Enable GitHub Packages for Docker image publishing:
- Settings → Actions → General → Workflow permissions → "Read and write permissions"

## Creating a Release

### 1. Prepare Release

Update version in `Cargo.toml`:

```toml
[package]
version = "0.6.0"  # Update this
```

Run checks locally:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo deny check
```

### 2. Open and Merge the Release PR

`develop` is the active development branch and `main` is the stable/release
branch. Never tag from `develop`. Promote the release through `main` first.

```bash
# Commit the version bump on develop
git add Cargo.toml
git commit -m "chore: bump version to 0.6.0"
git push origin develop
```

Open a release PR from `develop` into `main` (the `release-pr.yml` workflow can
open or update it via `workflow_dispatch`), let CI pass, then merge it.

### 3. Tag the Release on `main`

Tag the `main` merge commit and push the tag. The tag — not a branch push —
triggers the release automation.

```bash
git checkout main
git pull origin main

# Create the annotated tag on the main merge commit
git tag -a v0.6.0 -m "Release v0.6.0"
git push origin v0.6.0
```

### 4. Automated Workflows

Pushing the tag automatically triggers:

1. **Release Workflow** (`release.yml`)
   - Builds binaries for all platforms
   - Generates changelog from commits
   - Creates GitHub release with artifacts

2. **Changelog Workflow** (`changelog.yml`)
   - Updates CHANGELOG.md
   - Opens a PR into the develop branch

3. **Docker Workflow** (`docker.yml`)
   - Builds multi-platform images
   - Pushes to ghcr.io with version tag and 'latest'

4. **Publish Workflow** (`publish.yml`)
   - Runs all pre-publish checks
   - Publishes to crates.io

5. **Homebrew Workflow** (`package-homebrew.yml`)
   - Triggered by the published GitHub release
   - Regenerates `nsip.rb` and `nsip-source.rb` in `zircote/homebrew-tap`

6. **Back-Merge Workflow** (`back-merge.yml`)
   - Opens and auto-merges a `main` → `develop` PR
   - No manual step required

See [`docs/runbooks/RELEASING.md`](runbooks/RELEASING.md) for the full step-by-step procedure, including rollback and troubleshooting.

## Deployment Targets

### GitHub Releases

**Access:** https://github.com/zircote/nsip/releases

**Artifacts** (release-asset names carry the version; `<VERSION>` = tag minus the `v`):
- `nsip-<VERSION>-linux-amd64` - Linux x86_64
- `nsip-<VERSION>-linux-arm64` - Linux ARM64
- `nsip-<VERSION>-macos-amd64` - macOS x86_64
- `nsip-<VERSION>-macos-arm64` - macOS ARM64 (Apple Silicon)
- `nsip-<VERSION>-windows-amd64.exe` - Windows x86_64

**Download Example:**

```bash
# Linux
wget https://github.com/zircote/nsip/releases/download/v0.6.0/nsip-0.6.0-linux-amd64
chmod +x nsip-0.6.0-linux-amd64
./nsip-0.6.0-linux-amd64 --version
```

### Docker (GitHub Container Registry)

**Registry:** ghcr.io/zircote/nsip

**Supported Platforms:**
- linux/amd64
- linux/arm64

**Pull and Run:**

```bash
# Latest version
docker pull ghcr.io/zircote/nsip:latest
docker run --rm ghcr.io/zircote/nsip:latest --version

# Specific version
docker pull ghcr.io/zircote/nsip:v0.6.0
docker run --rm ghcr.io/zircote/nsip:v0.6.0 --version

# With volumes
docker run --rm -v $(pwd):/data ghcr.io/zircote/nsip:latest
```

**Image Details:**
- Base: distroless/cc-debian12 (minimal attack surface)
- User: nonroot:nonroot (unprivileged)
- Healthcheck: Built-in with `--version` command
- Size: ~10-15 MB (optimized multi-stage build)

### crates.io

**Package:** https://crates.io/crates/nsip

**Install:**

```bash
# Latest version
cargo install nsip

# Specific version
cargo install nsip@0.6.0

# From source
cargo install --git https://github.com/zircote/nsip
```

**Use in Project:**

```toml
[dependencies]
nsip = "0.6"
```

## Versioning

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0) - Incompatible API changes
- **MINOR** (0.1.0) - Backwards-compatible functionality
- **PATCH** (0.0.1) - Backwards-compatible bug fixes

## Changelog

Changelogs are automatically generated from conventional commits:

- `feat:` → Added section
- `fix:` → Fixed section
- `docs:` → Documentation section
- `perf:` → Performance section
- `refactor:` → Refactored section
- `test:` → Testing section
- `chore:` → Miscellaneous section

**Example Commit:**

```bash
git commit -m "feat(auth): add JWT token validation"
```

## Rollback

### GitHub Release

Delete the release and tag:

```bash
# Delete remote tag
git push --delete origin v0.6.0

# Delete local tag
git tag -d v0.6.0

# Delete release via GitHub UI or gh CLI
gh release delete v0.6.0
```

### Docker

Images are immutable; use previous version tags:

```bash
docker pull ghcr.io/zircote/nsip:v0.6.0
```

### crates.io Rollback

**Cannot unpublish** - crates.io doesn't allow unpublishing. Options:

1. Yank the version (prevents new projects from using it):
   ```bash
   cargo yank --vers 0.6.0
   ```

2. Publish a patch version with fixes:
   ```bash
   # Update to X.Y.Z+1
   git tag -a vX.Y.Z -m "Release vX.Y.Z (fixes vA.B.C)"
   git push origin vX.Y.Z
   ```

## Monitoring

### GitHub Actions

Monitor workflow runs:
- Actions tab: https://github.com/zircote/nsip/actions

### Security Audits

Daily automated security scans run at 00:00 UTC:
- Workflow: `.github/workflows/security-audit.yml`
- Uses: cargo-audit
- Notifications: GitHub Actions UI

### Dependencies

Dependabot automatically opens PRs for:
- Cargo dependencies
- GitHub Actions versions

## Troubleshooting

### Release Workflow Fails

**Build Error:**
- Check Cargo.toml version matches tag
- Verify MSRV compatibility (1.92+)
- Test locally: `cargo build --release`

**Cross-compilation Error:**
- Linux ARM64 requires `gcc-aarch64-linux-gnu`
- macOS ARM64 requires macOS 11+ runner

### Docker Build Fails

**Context Issue:**
- Verify .dockerignore excludes target/
- Check Dockerfile paths match `crates/` structure

**Push Permission:**
- Verify GitHub Actions workflow permissions
- Check ghcr.io login succeeds

### Publish to crates.io Fails

**Auth Issue:**
- Verify the Trusted Publishing config on crates.io matches repo `zircote/nsip`, workflow `publish.yml`, environment `copilot`

**Pre-publish Checks:**
- All tests must pass
- No clippy warnings
- cargo-deny checks must pass

## Best Practices

1. **Test Before Tagging**
   ```bash
   cargo build --release
   cargo test --all-features
   cargo clippy --all-targets --all-features -- -D warnings
   ```

2. **Use Conventional Commits**
   - Enables automatic changelog generation
   - Clearly communicates changes

3. **Version Bump in Separate Commit**
   ```bash
   git commit -m "chore: bump version to 0.6.0"
   ```
   Tag only after the `develop` → `main` release PR merges (see steps 2-3):
   ```bash
   git tag -a v0.6.0 -m "Release v0.6.0"
   ```

4. **Monitor Release Progress**
   - Watch GitHub Actions for workflow completion
   - Verify artifacts are uploaded
   - Test Docker image immediately after push

5. **Document Breaking Changes**
   - Use `BREAKING CHANGE:` in commit body
   - Update migration guide in CHANGELOG
