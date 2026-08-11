//! Regression guard for issue #357: the explanation docs
//! (`docs/explanation/BREED-GROUPS-AND-TRAITS.md` and
//! `docs/explanation/EBV-EXPLAINED.md`) previously undercounted the EBV
//! traits relative to the canonical EBV trait set used by the reference docs
//! (`docs/reference/CLI.md`, `docs/reference/MCP-TOOLS.md`, `docs/MCP.md`)
//! and the code (e.g. `crates/format.rs::TRAIT_ORDER` and
//! `crates/mcp/analytics.rs::EBV_TRAITS` via `nsip::mcp::analytics::ebv_glossary()`),
//! and both omitted YEMD/YFAT entirely.
//!
//! These tests read the explanation docs from disk and assert every
//! canonical trait abbreviation is mentioned, so a future trait addition
//! that updates the reference docs/code but not the explanation docs fails
//! CI instead of drifting silently.

use std::fs;
use std::io;
use std::path::PathBuf;

/// Canonical trait abbreviations, mirrored from `crates/format.rs::TRAIT_ORDER`
/// and `crates/mcp/resources.rs::glossary_content_has_all_sixteen_traits`.
const CANONICAL_TRAITS: &[&str] = &[
    "BWT", "WWT", "PWWT", "YWT", "MWWT", "NLB", "NLW", "PEMD", "PFAT", "YEMD", "YFAT", "WFEC",
    "PFEC", "YFD", "YGFW", "YSL",
];

fn docs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/explanation")
}

fn read_doc(name: &str) -> io::Result<String> {
    fs::read_to_string(docs_dir().join(name))
}

/// True if `haystack` mentions `trait_abbrev` as a standalone token (not merely
/// as a substring of a longer abbreviation, e.g. "WWT" inside "MWWT").
fn mentions_trait(haystack: &str, trait_abbrev: &str) -> bool {
    haystack.match_indices(trait_abbrev).any(|(idx, _)| {
        let before_is_letter = haystack[..idx]
            .chars()
            .next_back()
            .is_some_and(char::is_alphabetic);
        let after_is_letter = haystack[idx + trait_abbrev.len()..]
            .chars()
            .next()
            .is_some_and(char::is_alphabetic);
        !before_is_letter && !after_is_letter
    })
}

#[test]
fn breed_groups_and_traits_covers_all_canonical_traits() {
    let content =
        read_doc("BREED-GROUPS-AND-TRAITS.md").expect("explanation doc should be readable");
    for trait_abbrev in CANONICAL_TRAITS {
        assert!(
            mentions_trait(&content, trait_abbrev),
            "BREED-GROUPS-AND-TRAITS.md is missing canonical trait {trait_abbrev}"
        );
    }
}

#[test]
fn ebv_explained_covers_all_canonical_traits() {
    let content = read_doc("EBV-EXPLAINED.md").expect("explanation doc should be readable");
    for trait_abbrev in CANONICAL_TRAITS {
        assert!(
            mentions_trait(&content, trait_abbrev),
            "EBV-EXPLAINED.md is missing canonical trait {trait_abbrev}"
        );
    }
}

#[test]
fn breed_groups_and_traits_heading_matches_canonical_count() {
    let content =
        read_doc("BREED-GROUPS-AND-TRAITS.md").expect("explanation doc should be readable");
    let expected = format!(
        "Understanding the {} Traits in Context",
        CANONICAL_TRAITS.len()
    );
    assert!(
        content.contains(&expected),
        "expected heading {expected:?} not found -- trait count in the heading has drifted \
         from the canonical trait list"
    );
}

#[test]
fn canonical_traits_matches_ebv_glossary() {
    let from_glossary: Vec<&'static str> = nsip::mcp::analytics::ebv_glossary()
        .into_iter()
        .map(|t| t.abbreviation)
        .collect();

    assert_eq!(
        from_glossary, CANONICAL_TRAITS,
        "CANONICAL_TRAITS should stay in sync with the canonical EBV glossary"
    );
}
