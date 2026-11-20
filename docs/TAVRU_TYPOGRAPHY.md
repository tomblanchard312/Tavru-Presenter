# Tavru Presenter typography

Adopt **Inter** (SIL OFL) as the default application UI font, in coordination with [Typography tracking issue](https://github.com/tomblanchard312/TavruOS/issues/27).

- Use Inter 400 for body, 500 for controls, 600 for headings. Define semantic typography tokens for font family, size, weight, line height and spacing.
- Suggested UI sizes: 10.5–11 pt body, 10.5 pt menus, 11 pt window titles, 14–16 pt headings, adapted to toolkit DPI scaling.
- Use JetBrains Mono for code/terminal and Noto family for multilingual fallback. Do not override the fonts chosen inside user-authored documents or artwork.
- Respect accessibility text scaling and user font overrides; support 100%, 125%, 150% and 200% scaling, dark/light themes, international scripts and emoji.
- Prefer OS-installed font packages and documented portable fallback. Preserve OFL license notices and original font names.
- Map the shared tokens to this application's native UI toolkit; keep typography consistent across dialogs, menus, tooltips, settings and other chrome.
- Test Linux first, FreeBSD next, and illumos later. Implement changes in a separate PR; do not merge automatically.
