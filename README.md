# Tavru Presenter

Tavru Presenter is a presentation editor forked from [DeckCraft by the ArtCraft team](https://github.com/storytold/deckcraft), preserving its Rust/egui architecture, source history, and open-source licensing.

**Status:** development fork with Tavru Presenter app/installer branding and an initial Oqelith integration. Tavru Docs, Tavru Sheets and Tavru PDF exchange remains planned. No Tavru Presenter binaries have been released or platform compatibility certified.

The inherited editor includes slides, themes, shapes, charts, speaker notes, slide shows, PowerPoint `.pptx` import/export, PDF export, a CLI and MCP automation. These are upstream capabilities; fidelity and platform behavior need validation in this fork. See the [upstream roadmap](ROADMAP.md) and [Inanna integration roadmap](docs/inanna-roadmap.md).

Latest upstream integration: [DeckCraft synchronization, 2026-11-02](docs/upstream-sync-2026-11-02.md).

[EriduOS interface brief](docs/ERIDU_INTERFACE.md)

## Build from source

```sh
git clone https://github.com/tomblanchard312/Inanna.git
cd Inanna
cargo run --release -p deckcraft -- --sample
```

Linux builds need ALSA development headers (`libasound2-dev` on Debian/Ubuntu). Optional fonts come from [craft-fonts](https://github.com/storytold/craft-fonts), checked out alongside Inanna; otherwise system fonts are used. See [PDF dependency requirements](crates/pdf/README.md).

Internal crate names and `.deckcraft` files retain upstream compatibility. Executables are `inanna` and `inanna-cli`; settings and recovery use a separate Inanna directory. Installers have independent app and upgrade identities. Upstream release downloads and signing claims do not apply to Inanna.

## Enki

Open **Enki** from the title bar or the **View** tab for text recommendations, text improvements, layout proposals, image generation and logo generation. It uses the shared folded blue Enki mark. Start a compatible local Enki daemon; Enki selects a local Phi-4 or Qwen model automatically and uses Enki’s image recommendation. Review each proposal before applying. Edits are undoable; changed slides reject stale proposals.

See [Enki setup and verification](docs/scribe.md). The running app does not download models or install Enki.

## Direction

- Enki: optional local assistance for outlines, slide text and speaker notes.
- Nisaba: exchange document outlines, headings, notes and editable text.
- GridCraft: exchange typed tables and charts with explicit formatting rules.
- Nabu: PDF handoff and review, using existing PDF export first.
- Windows, Linux and FreeBSD: native builds, installation and real desktop validation.

The visual direction is dark surfaces, blue accents, clear sidebar navigation, generous spacing and readable controls, consistent with the suite. Keep the existing Rust/egui implementation and command architecture. See [branding and provenance](docs/inanna-branding.md).

## License and credits

The inherited code is licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE), at your option. Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors. Required notices remain in [NOTICE](NOTICE); asset licenses and sources remain in [ATTRIBUTION.md](ATTRIBUTION.md).

Inanna is an independent fork, with no claim of upstream sponsorship or endorsement. ArtCraft trademark graphics were removed from this branch; their original [terms](docs/brand/LICENSE-brand.txt) remain for provenance. The upstream commit history retains the original materials. Changes in this fork are recorded in Git and the [branding record](docs/inanna-branding.md).

Voice accessibility preview: see [dictation, transcription, translation and safe voice controls](docs/VOICE.md).
