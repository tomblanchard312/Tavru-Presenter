# Inanna integration roadmap

Status: staged plan, updated 2026-10-30. App/installer branding and initial native Scribe integration are implemented; acceptance results and remaining limits are in [Scribe](scribe.md) and the review PR.

## 1. Establish the baseline and release identity

Preserve engine commands, file formats, undo/recovery, CLI and MCP. Record a synthetic `.pptx` round-trip corpus and PDF/render baselines before changing behavior. Run upstream format, lint, tests, layering, asset and WASM checks. Audit runtime/installer branding and update URLs before distributing an Inanna build. Preserve license and dependency notices in every package.

Acceptance: a reviewed baseline report identifies exact revision, checks passed, failures and untested platforms; no claims borrowed from upstream releases. Branding and updater checks pass for each deliverable.

## 2. Oqelith local assistance

First inspect Oqelith's current SDK, transport, supported platforms and model lifecycle. Use its existing text/chat interface for a single outline-to-slides flow, followed by notes and text revision. The current image-capable SDK snapshot supports image and logo generation; the native Scribe panel uses that verified API. Keep Oqelith optional; editing and export must work without a runtime or model. Use explicit user initiation, bounded input/output, cancellation, timeouts and visible errors. No silent network fallback or model downloads.

Route accepted changes through existing engine commands so they are undoable. Treat document and model text as untrusted; model output cannot authorize filesystem access or execute arbitrary MCP commands. Preview proposed edits before applying them.

Acceptance: offline outline generation succeeds with an available model; missing runtime, cancellation and malformed output preserve the deck; accepted edits undo correctly. Test the real runtime on every supported OS before claiming support.

## 3. Nisaba document exchange

Inspect actual export/import contracts first. Start with an explicit document outline interchange (headings, paragraphs and speaker notes), using existing supported file formats or a small versioned payload only if required. Preserve Unicode, ordering and source provenance. Preview slide boundaries and report unsupported formatting; do not silently discard source content or overwrite the document.

Acceptance: a synthetic document becomes an editable deck with expected headings and notes; exporting an outline preserves text and ordering. Unsupported constructs and failures are reported without data loss.

## 4. GridCraft tables and charts

Verify GridCraft's actual data interfaces before selecting an adapter. Begin with a user-selected range imported as a snapshot. Preserve typed values, locale-independent numbers, blanks, dates, labels and units; define formula-result and chart mappings explicitly. Bound input sizes and reject invalid dimensions. Defer live links until snapshot exchange is reliable.

Acceptance: representative tables and charts retain values, labels and units; edits undo correctly; stale data and unsupported chart features are visible. Spreadsheet import must not execute formulas or external links.

## 5. Nabu PDF workflows

Use inherited PDF export first; inspect Nabu's actual open/import contract for an explicit handoff. Preserve selectable text, slide order, page dimensions and requested notes/handouts. Clearly label PDF as a rendered exchange, with no promise of editable slide round trips. Use safe temporary files and explicit output selection; never overwrite an existing PDF without confirmation.

Acceptance: Nabu opens the exported PDF and displays the expected pages; inspect fonts, Unicode, transparency and charts. Failed handoff leaves the saved PDF accessible and the deck intact.

## 6. Platform acceptance

| Target | Required evidence before support is claimed |
|---|---|
| Windows | Native build/install/uninstall, ordinary-user launch, GPU fallback, fonts, file dialogs, `.pptx`, PDF, slideshow/media and Oqelith smoke tests; report signing status honestly. |
| Linux | Zorin/Ubuntu native desktop tests on X11 and Wayland; GPU/software fallback, ALSA/media, fonts, portals, install/uninstall and Oqelith; validate each distributed package separately. |
| FreeBSD | Native FreeBSD build and desktop run; graphics, audio, fonts, dialogs, `.pptx`, PDF, slideshow and Oqelith availability; package dependencies and clean install/uninstall in a disposable environment. Cross-compilation alone is insufficient. |

All three are targets pending fork validation. Preserve inherited macOS and WASM paths when changing shared code, without expanding the initial certification scope. Run GUIs as an ordinary user; privileged installation uses narrow helpers and explicit native authentication. Avoid broad passwordless privileges and GUI password capture.

## Delivery and review

Each milestone is a separate feature branch and PR with acceptance evidence and known limits. Inspect suite repositories before committing to APIs; project names here express intended roles, not verified interoperability. No automatic merge. Defer new dependencies, live spreadsheet links and shared-suite abstractions until a concrete integration requires them.
