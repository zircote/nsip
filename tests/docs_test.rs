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
    fs::read_to_string(&path)
        .expect("doc file must be readable")
        .replace("\r\n", "\n")
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
fn releasing_runbook_does_not_send_readers_back_for_secrets() {
    // Regression guard for https://github.com/epicpast/nsip/issues/385:
    // RELEASING.md's own back-pointer to DEPLOYMENT.md once described the
    // overview it points at as covering "secrets", even though RELEASING.md
    // is itself the authoritative source for required secrets (see its own
    // "Required Secrets" table above). That sent a reader looking for
    // secrets on a RELEASING.md -> DEPLOYMENT.md -> RELEASING.md loop.
    //
    // Scoped to the clause describing what DEPLOYMENT.md covers, not the
    // whole paragraph: this file keeps a paragraph on one physical line, and
    // that same paragraph legitimately says RELEASING.md itself covers
    // secrets earlier in the sentence.
    let releasing = read_doc("runbooks/RELEASING.md");
    let link_marker = "](../DEPLOYMENT.md)";
    let after_link = releasing
        .split_once(link_marker)
        .expect("releasing_runbook_links_back_to_deployment_guide already asserts this link exists")
        .1;
    let overview_clause = after_link.split('.').next().unwrap_or("");
    assert!(
        !overview_clause.to_lowercase().contains("secret"),
        "docs/runbooks/RELEASING.md's pointer to docs/DEPLOYMENT.md must not \
         claim DEPLOYMENT.md covers secrets -- RELEASING.md's own Required \
         Secrets table is the authoritative source"
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
