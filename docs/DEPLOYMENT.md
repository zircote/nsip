---
id: nsip-docs-deployment
type: semantic
created: 2026-02-07T14:26:06-05:00
namespace: nsip/docs
modified: '2026-08-11T19:08:46.262Z'
title: "Deployment Guide"
diataxis_type: reference
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

This document is a reference for the nsip project's deployment targets and
distribution channels — where releases land and how consumers install them.

**Scope boundary:** this document does not duplicate the release procedure.
[`docs/runbooks/RELEASING.md`](runbooks/RELEASING.md) is the single
authoritative source for how to cut, monitor, roll back, and troubleshoot a
release; it links back here for the broader distribution overview. If a
detail about "how to release" here and in RELEASING.md ever disagree,
RELEASING.md wins — file a docs bug rather than trusting this page's copy.

## Overview

The project includes automated deployment workflows for:

- **GitHub Releases** - Multi-platform binaries
- **Docker** - Container images on GitHub Container Registry
- **crates.io** - Rust package registry

## Prerequisites

See [`docs/runbooks/RELEASING.md#prerequisites`](runbooks/RELEASING.md#prerequisites)
for the authoritative list of required secrets and permissions
(`HOMEBREW_TAP_TOKEN`, crates.io Trusted Publishing, `GITHUB_TOKEN`, and the
GitHub Packages workflow permission needed for Docker publishing).

## Creating a Release

The step-by-step release procedure — version bump, promoting `develop` to
`main`, tagging, pushing, and the workflows a tag push triggers — lives in
[`docs/runbooks/RELEASING.md`](runbooks/RELEASING.md#step-by-step-promote-tag-and-push-a-release).
That runbook also covers monitoring, rollback, and troubleshooting.

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
# Linux (X.Y.Z = the release version, e.g. 0.7.3)
wget https://github.com/zircote/nsip/releases/download/vX.Y.Z/nsip-X.Y.Z-linux-amd64
chmod +x nsip-X.Y.Z-linux-amd64
./nsip-X.Y.Z-linux-amd64 --version
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

# Specific version (X.Y.Z = the release version, e.g. 0.7.3)
docker pull ghcr.io/zircote/nsip:vX.Y.Z
docker run --rm ghcr.io/zircote/nsip:vX.Y.Z --version

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

# Specific version (X.Y.Z = the release version, e.g. 0.7.3)
cargo install nsip@X.Y.Z

# From source
cargo install --git https://github.com/zircote/nsip
```

**Use in Project:**

```toml
[dependencies]
nsip = "0.7"
```

## Versioning

This project follows [Semantic Versioning 2.0.0](https://semver.org/) — see
[`docs/runbooks/RELEASING.md#version-numbering-semver`](runbooks/RELEASING.md#version-numbering-semver)
for the full policy, including the pre-1.0 exception.

## Changelog

Changelogs are generated automatically from conventional commits by
[git-cliff](https://git-cliff.org/) — see
[`docs/runbooks/RELEASING.md#changelog-generation`](runbooks/RELEASING.md#changelog-generation)
for the commit-prefix-to-section mapping.

## Monitoring

Release-workflow monitoring (which jobs run, expected duration, what to
watch for) lives in
[`docs/runbooks/RELEASING.md#monitoring-workflow-progress`](runbooks/RELEASING.md#monitoring-workflow-progress).

Outside the release path:

- **Security audits** — daily `cargo audit` scan; see
  [`docs/workflows/SECURITY-AUDIT.md`](workflows/SECURITY-AUDIT.md)
- **Dependency updates** — Dependabot policy and manual auditing; see
  [`docs/runbooks/DEPENDENCY-UPDATES.md`](runbooks/DEPENDENCY-UPDATES.md)

## Rollback and Troubleshooting

Rollback procedures (GitHub Release, Docker, crates.io) and the release
troubleshooting table live in
[`docs/runbooks/RELEASING.md`](runbooks/RELEASING.md#rollback-procedures) —
this document does not maintain a second copy.
