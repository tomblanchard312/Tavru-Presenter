# Oqelith in Inanna

Status: initial native integration, 2026-10-30.

Open Oqelith from the title bar or View tab/menu; closing the panel leaves those entry points available. The button and panel use the suite's original blue folded-page S icon, sourced from Nisaba at the immutable revision in ATTRIBUTION.md.

## Setup

Run a compatible local Oqelith daemon with registered language and diffusion models. Text uses Oqelith protocol v2; image/logo generation requires v3 and the image worker. Oqelith selects a registered Phi-4 model first and falls back to Qwen if it is unavailable or fails to load. Image generation uses the daemon’s current recommendation. Model selection and identifiers are not shown in the user interface. Missing services or models produce visible errors. Inanna neither downloads models nor installs or updates the daemon.

The SDK is pinned to the published Nisaba snapshot in Qanloom at commit 3d75b6a77231ade0c906140e16f9ef66613e0b29, because Oqelith's published default branch does not yet include those image APIs. Its client/protocol packages retain their UNLICENSED designation. Inanna's MIT/Apache terms do not grant rights to those dependencies; resolve their distribution terms before a public binary release.

## Tasks

- Recommend text or improve wording using the current slide and your instruction. Review and edit the output. If one shape was selected when the request started, Apply replaces its text; otherwise it inserts a new text box.
- Improve layout asks Oqelith to choose from existing safe engine arrangements. Only a returned ID from that allowlist can be applied. The proposal describes the arrangement, and Apply rearranges existing objects. Layouts outside the engine's current design ideas are not supported.
- Generate image or logo requests a PNG from Oqelith. Review the preview, then insert it as a picture. Logos are raster output; vector logo editing is outside this initial integration.

Only an explicit Generate action sends the current slide inspection and your instruction to the local service. Picture prompts use your instruction without sending slide content. Requests run off the UI thread and can be cancelled. Closing the application cancels outstanding work. Output is bounded, decoded images are checked before display, and model output never selects arbitrary engine commands or filesystem paths.

Each accepted result uses an existing undoable engine command. A changed document or slide invalidates the result; regenerate rather than applying it to another slide. Model selection is automatic. Existing editing, export and file recovery work without Oqelith.

## Development checks

Run the workspace checks with ALSA development headers installed on Linux:

```sh
sudo apt-get install libasound2-dev pkg-config
cargo xtask ci
```

The normal tests cover brand-icon decoding, the layout allowlist, undo and stale proposals. Real Oqelith/model validation must be recorded separately from unit tests; native Windows/FreeBSD runtime checks are separate release gates.

## Recorded Linux verification

Synthetic live-service checks passed text recommendation, improvement, allowlisted layout selection, and 64×64 one-step image/logo generation through the installed Oqelith service. Automatic text selection also passed without a supplied model ID. These are integration smoke checks, not image-quality or native Windows/FreeBSD acceptance.
