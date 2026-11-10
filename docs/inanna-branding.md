# Inanna branding and provenance

Status: application and installer branding implemented; native package acceptance pending, 2026-10-30.

- Upstream: https://github.com/storytold/deckcraft
- Fork: https://github.com/tomblanchard312/Inanna
- Initial upstream commit: `3a4e30e10e83c5f988a074dba1a519984828b4fd`
- Identity: Inanna, an independent presentation editor based on DeckCraft by the ArtCraft team.
- Direction: dark surfaces, blue accents, readable controls, generous spacing and clear sidebar navigation. Retain Rust and egui/eframe; no Tk interface or rewrite.

This change replaces the repository landing page and removes the eight ArtCraft trademark graphics from the current source tree. LICENSE-MIT, LICENSE-APACHE, NOTICE, third-party notices and the brand terms are retained. Attribution rows for the removed graphics are removed; historical authorship stays in Git. The follow-up reuses credited, openly licensed app and Scribe artwork as described below.

Application titles, About/support links, Windows resources, CLI and MCP identity, desktop metadata, package names and installer labels now identify Inanna. The icons are blue derivatives of the openly licensed DeckCraft geometry; original credits remain. The shared Scribe icon retains its Nisaba source attribution.

Executable names are `inanna` and `inanna-cli`; internal crate names, document extensions, command IDs and built-in presentation theme IDs remain compatible. User settings and recovery use separate Inanna directories, without moving or deleting DeckCraft data. The Windows MSI has a new upgrade code, component GUID, registry namespace and Inanna install paths, so it does not upgrade DeckCraft. The desktop/Flatpak/macOS ID is `io.github.tomblanchard312.Inanna`.

Updates open the fork's release page for explicit download; the inherited automatic updater and restart helper are removed. No Inanna binary is claimed signed or notarized. The macOS package uses a plain drag-to-Applications DMG rather than an outdated branded background/layout.

Before release, validate native installers, signing and uninstall behavior on their actual operating systems. Also resolve distribution terms for Scribe's Oqelith SDK dependency, which is currently designated UNLICENSED. Platform claims remain unaccepted until those checks pass.

## Upstream synchronization

Keep `upstream` pointed at DeckCraft and `origin` at Inanna. Fetch upstream changes, integrate on a feature branch, rerun checks and review a PR against the fork default branch. Never rewrite inherited history or automatically merge. Do not restore restricted marks during synchronization. The original upstream roadmap and technical documentation remain as baseline material, not Inanna completion claims.
