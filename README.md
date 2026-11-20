# Tavru Presenter

Tavru Presenter is a presentation editor forked from [DeckCraft by the ArtCraft team](https://github.com/storytold/deckcraft). It keeps the Rust/egui architecture, source history and open-source licensing. The repository is [github.com/tomblanchard312/Tavru-Presenter](https://github.com/tomblanchard312/Tavru-Presenter). The earlier public name was Inanna.

**Status:** development fork. The window title, About and PDF creator string say Tavru Presenter. Oqelith can propose text, layout and images for review before they are applied. Exchange with Tavru Docs, Tavru Sheets and Tavru PDF is planned, not implemented. No Tavru Presenter binaries have been released, and platform compatibility is not certified.

The inherited editor includes slides, themes, shapes, charts, speaker notes, slide shows, PowerPoint `.pptx` import/export, PDF export, a CLI and MCP automation. Treat those as upstream capabilities until this fork validates them. See the [upstream roadmap](ROADMAP.md) and the [integration roadmap](docs/inanna-roadmap.md). PDF export and on-screen rectangles use `tavru-layout-core` 0.2.0 so shared edges stay on device pixels.

Latest upstream integration: [DeckCraft synchronization, 2026-11-02](docs/upstream-sync-2026-11-02.md).

[TavruOS interface brief](docs/TAVRU_INTERFACE.md). The filename is unchanged.

## Build from source

```sh
git clone https://github.com/tomblanchard312/Tavru-Presenter.git
cd Tavru-Presenter
cargo run --release -p deckcraft -- --sample
```

The Cargo package is `deckcraft`. The installed executables are still `inanna` and `inanna-cli`. Documents stay `.deckcraft`. Settings and recovery stay in the Inanna directory. The desktop id is `io.github.tomblanchard312.Inanna`. Update and issue links in the app still open `github.com/tomblanchard312/Inanna`, which GitHub redirects. Linux builds need ALSA development headers (`libasound2-dev` on Debian/Ubuntu). Optional fonts come from [craft-fonts](https://github.com/storytold/craft-fonts), checked out beside this repository; otherwise system fonts are used. See [PDF dependency requirements](crates/pdf/README.md).

Upstream release downloads and signing claims do not apply to Tavru Presenter.

## Oqelith

Open **Oqelith** from the title bar or the **View** tab for text recommendations, text improvements, layout proposals, image generation and logo generation. Start a compatible local Oqelith daemon. Oqelith selects a local Phi-4 or Qwen model automatically and uses Oqelith’s image recommendation. Review each proposal before applying. Edits are undoable; changed slides reject stale proposals.

See [Oqelith setup and verification](docs/scribe.md). The running app does not download models or install Oqelith.

## Direction

- Oqelith: optional local assistance for outlines, slide text and speaker notes. This part is in the app.
- Tavru Docs: exchange document outlines, headings, notes and editable text. Planned.
- Tavru Sheets: exchange typed tables and charts with explicit formatting rules. Planned.
- Tavru PDF: PDF handoff and review, using the existing PDF export first. Planned.
- Windows, Linux and FreeBSD: native builds, installation and real desktop validation. Linux is the current development target.

The interface uses Inter, Phosphor controls and the shared Tavru style crate (the Cargo package name is still `eridu-style` where that dependency has not been renamed). See [branding and provenance](docs/inanna-branding.md).

## License and credits

The inherited code is licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE), at your option. Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors. Required notices remain in [NOTICE](NOTICE); asset licenses and sources remain in [ATTRIBUTION.md](ATTRIBUTION.md).

Tavru Presenter is an independent fork, with no claim of upstream sponsorship or endorsement. ArtCraft trademark graphics were removed from this branch; their original [terms](docs/brand/LICENSE-brand.txt) remain for provenance. The upstream commit history retains the original materials. Changes in this fork are recorded in Git and the [branding record](docs/inanna-branding.md).

Voice accessibility preview: see [dictation, transcription, translation and safe voice controls](docs/VOICE.md).
