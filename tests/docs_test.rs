//! Guards the supersession boundary between `docs/DEPLOYMENT.md` and
//! `docs/runbooks/RELEASING.md` established for
//! <https://github.com/epicpast/nsip/issues/356>.
//!
//! `docs/runbooks/RELEASING.md` is the single authoritative source for the
//! release *procedure* (creating, monitoring, rolling back, and
//! troubleshooting a release). `docs/DEPLOYMENT.md` is scoped to deployment
//! targets and distribution channels, and must point at RELEASING.md rather
//! than re-describing the procedure. These tests fail if either file drifts
//! back into duplicating the other's content.

use std::fs;
use std::path::PathBuf;

fn docs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs")
}

#[allow(clippy::expect_used)] // test-support helper, not production code
fn read_doc(relative: &str) -> String {
    let path = docs_dir().join(relative);
    fs::read_to_string(&path).expect("doc file must be readable")
}

#[test]
fn deployment_guide_points_at_releasing_runbook() {
    let deployment = read_doc("DEPLOYMENT.md");
    assert!(
        deployment.contains("runbooks/RELEASING.md"),
        "docs/DEPLOYMENT.md must link to docs/runbooks/RELEASING.md as the \
         authoritative release procedure"
    );
}

#[test]
fn releasing_runbook_links_back_to_deployment_guide() {
    let releasing = read_doc("runbooks/RELEASING.md");
    assert!(
        releasing.contains("DEPLOYMENT.md"),
        "docs/runbooks/RELEASING.md must link back to docs/DEPLOYMENT.md for \
         the broader deployment/distribution overview"
    );
}

#[test]
fn deployment_guide_does_not_duplicate_the_release_procedure() {
    let deployment = read_doc("DEPLOYMENT.md");

    // These headings/strings identify the step-by-step release procedure,
    // rollback steps, and troubleshooting table that live exclusively in
    // docs/runbooks/RELEASING.md. Their reappearance in DEPLOYMENT.md means
    // the two documents have drifted back into duplication.
    let duplicated_markers = [
        "### 1. Prepare Release",
        "### 2. Open and Merge the Release PR",
        "### 3. Tag the Release on",
        "## Rollback\n",
        "### Release Workflow Fails",
        "### Docker Build Fails",
        "### Publish to crates.io Fails",
        "## Best Practices",
    ];

    for marker in duplicated_markers {
        assert!(
            !deployment.contains(marker),
            "docs/DEPLOYMENT.md re-introduced duplicated release-procedure \
             content ({marker:?}); this content belongs solely in \
             docs/runbooks/RELEASING.md"
        );
    }
}

#[test]
fn deployment_guide_states_the_scope_boundary() {
    let deployment = read_doc("DEPLOYMENT.md");
    assert!(
        deployment.contains("Scope boundary"),
        "docs/DEPLOYMENT.md must explicitly state its scope boundary against \
         docs/runbooks/RELEASING.md"
    );
}
