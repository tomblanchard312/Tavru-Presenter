# Inanna interface brief

Tavru Presenter is a TavruOS presentation workspace for people creating, editing and presenting slides. The slide canvas is the visual anchor. Slide thumbnails provide stable navigation on the left, the ribbon groups editing commands above the canvas, and the optional inspector stays beside the selection. File opens the backstage workspace for creating, opening and saving presentations.

On first run, the File workspace offers a blank presentation and recent files. In a populated workspace, the active slide, selected ribbon tab and Present action identify the current task. Open and save errors use the existing app feedback and recovery paths. Users can continue editing when Ask or Voice cannot connect. Ask runs on-device and presents changes for review and explicit application. Its existing service errors identify the missing local runtime or model.

The GUI uses local semantic colors and sizing from the pinned Tavru style revision `97efe362bb37da690f58bc953fba03d4b9ba2c21`. Present uses its purple action tokens. The initial appearance follows the desktop, with light as the fallback. Users can select System, Light or Dark in File → Account. Preferences retains keyboard access, a visible interface size label and scaling through 200%. The ribbon keeps its scroll controls, shortcut names and accessible labels. Inter and semibold Inter serve body and headings, Inter Tabular serves measurements, and JetBrains Mono serves technical text. The existing multilingual fallback pipeline remains available. Common controls use Phosphor regular and selected toggles use Phosphor fill. Slide content uses document fonts.

Linux is the primary validation platform. FreeBSD is a secondary target. Windows, macOS and the browser are existing build targets with separate runtime acceptance. Illumos remains planned.

[Eridu interface requirements](https://github.com/tomblanchard312/TavruOS/blob/53133f86434676a5bdcb249556d3c11b50e600e4/design-system/INTERFACE_RULES.md) · [Style guide](https://github.com/tomblanchard312/TavruOS/blob/53133f86434676a5bdcb249556d3c11b50e600e4/design-system/STYLE_GUIDE.md) · [Adoption checklist](https://github.com/tomblanchard312/TavruOS/blob/53133f86434676a5bdcb249556d3c11b50e600e4/design-system/ADOPTION.md)

## Review evidence

All nine local workspace gates passed. Automated checks cover the reported
desktop theme and manual overrides, Preferences cancellation, text and action
contrast, focus color contrast, and ribbon visibility at the tested widths.
The browser entry point compiled for WebAssembly. The light and dark editor
screenshots were inspected at 1440 × 900 using the headless test harness.

The native Inanna debug build ran in an EriduOS live guest under KDE Plasma
Wayland. With Enki open and the app set to System appearance, the same process
followed Breeze Light to Breeze Dark. At 200% KDE output scaling on a 1920 ×
1080 display, the ribbon, slide canvas, status bar, and Enki prompt and Generate
button remained visible. This run exercised the editor and assistant panel.

## Adoption still to verify

| Scope | Owner | Reason and follow-up |
|---|---|---|
| Specialized icons and display type | Inanna maintainers | Office commands without a matching Phosphor glyph retain their original icon. The compact ribbon keeps its sizing and large headings use Inter Semibold under the Source Serif asset exclusion. [Eridu typography tracking](https://github.com/tomblanchard312/TavruOS/issues/27) |
| Native accessibility and workflow acceptance | Inanna maintainers | Test screen readers, reduced motion, localized text expansion, and FreeBSD installation. Track findings using the adoption checklist linked above. |
