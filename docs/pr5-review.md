# Inanna upstream PR #5 review fixes

Last updated: 2026-11-01. Review preview; platform release acceptance is unchanged.

The original PR points at `storytold/deckcraft:main` with maintainer edits disabled.
This integration uses an Inanna-owned branch, merging snapshot
`5fb694d9917d232fbf5b2e0f42ac15d73febde7c` into Inanna main `9fae11e6`.
Upstream history is preserved. Inanna branding, Scribe, Voice, pinned public MIT
SDK and binary-only Oqelith packaging are retained. No private Oqelith source is added.

| Review finding | Resolution and regression coverage |
| --- | --- |
| OCI script sources missing env.sh | Source packaging/common.sh; clean-checkout help test. |
| OCI push defaults to upstream registry | Require REGISTRY before any build/push; no-default-push regression. Artifact basename uses Inanna. |
| NOSUGGEST bypass on uppercase corrections | Check flags on the matched dictionary spelling; uppercase NOSUGGEST and KEEPCASE regressions. |
| Update-result dialog permits background edits | Retain Inanna's existing explicit release-page link. The upstream updater/dialog/runtime is excluded, so no modal or background update behavior is introduced. |
| Run formatting includes zero-length terminal paragraph | Use the effective half-open formatting span for selected levels; regression checks the next level/run remains untouched. |
| Grow/shrink ignores paragraph defaults | Runs inherit paragraph defRPr before level style; list-style sizes still step independently. Regression checks 40→44 pt runs and 32→36 pt level defaults. |
| Nonuniform animation scaling uses slide axes | Invert the local animation through the original shape/group transforms, including parent-space offsets; rotated-shape and existing hit tests. |
| Gradient flip preserved but not rendered | Tile gradients and flip alternating tiles for x/y/xy fills and outlines. Preserve the primary tile's orientation; cap hostile tiling work. Pixel and tile-boundary regressions. |
| Custom miter limit ignored | Convert stored DrawingML percentage to width ratio, retaining default 8; rendered corner regression. |
| Inside-aligned outline spills outside | Clip doubled closed-path strokes to the original path, preserving open-path behavior; pixel regression. |
| Motion-path edit silently succeeds on other classes | Reject non-path targets transactionally, including mixed selections; unchanged-document regression. |
| Duplicate roadmap status headers | Keep one current status line and the existing detailed progress log. |

Gradient flip follows the DrawingML tile definition, rather than reversing the
primary tile: https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.gradientfill?view=openxml-3.0.1

Additional integration fixes adapt imported closed-output/MCP tests to Inanna's
executable and server identities. Picture Format now uses a compact original-icon
More menu when inline styles would push Crop/Size off-screen; every style remains
available in the popup. Visibility, resize settling and applying a style are tested.
The 1440 px layout was inspected in a headless screenshot. To reproduce it, set
`INANNA_TEST_PICTURE_SCREENSHOT=/tmp/picture-format.png` while running the
`ribbon::picture_styles_tests` tests.

Validation results are recorded in the replacement PR. Native Windows/FreeBSD,
OCI image construction/publishing, physical audio, signing and production
acceptance are separate from local Rust/pixel/Python checks. Existing Oqelith runtime
platform/glibc/lifecycle release blockers remain documented in BUNDLED_OQELITH.md.
