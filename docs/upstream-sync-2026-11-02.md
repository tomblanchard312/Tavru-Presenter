# DeckCraft synchronization — 2026-11-02

Inanna main at `a15b4a55` is integrated with upstream DeckCraft snapshot
`0d45a64db23baa0a4c8f9f7ac14c59c91b40fec9` (57 additional upstream commits).
The merge preserves upstream history and updates the inherited workspace to 0.14.1.
No changes are pushed to upstream or Inanna main by this review task.

## Included changes

- Multi-object resizing and hit testing through unfilled shape interiors.
- Text inset handling, combining-mark spell checking, Greek casing and numeric formatting fixes.
- Bounded PPTX tag imports, slide-background validation and H.264 motion-vector corrections.
- Portable font/render validation and even-odd holes in shadows, glow and reflections.
- A separate browser presenter window, with same-origin/source checks and bounded messages.

## Fork resolutions

Retained Inanna's README, a single roadmap status line, branding, Scribe/Voice,
explicit release-page updates and binary-only Oqelith distribution. The previous
PR #5/#6 review fixes remain in place, including OCI publishing safety,
NOSUGGEST/KEEPCASE, master/layout formatting, animation hit tests, gradient tile
flips, outline alignment/miter limits and the compact Picture Format menu.

Used upstream's bundled PDF test font fixture rather than a system font lookup.
The browser presenter window title uses Inanna; internal compatibility identifiers
remain unchanged. No private Oqelith implementation source is introduced.

## Validation

All nine `cargo xtask ci` stages passed, including strict native workspace Clippy,
2322 passing tests (7 existing ignores), assets, layering, doctests and 23 shared
WASM crates. Both Python OCI/Oqelith staging suites passed. A separate
`cargo check --locked --offline -p deckcraft-web --target wasm32-unknown-unknown`
passed for the new browser entry point.

Native Windows/macOS/FreeBSD
installation, browser popup behavior, real audio, signing and publishing require
separate acceptance; compilation and automated tests do not establish those.
Existing Oqelith runtime platform/glibc/AVX2 and cross-app lifecycle release blockers
remain documented in BUNDLED_ENKI.md. Keep the PR unmerged for review.
