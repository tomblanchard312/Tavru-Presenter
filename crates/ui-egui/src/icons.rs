//! Ribbon and UI icons, drawn in code.
//!
//! Every icon here is an original drawing made from egui painter primitives on a 24×24 design
//! grid (contributor-original, `MIT OR Apache-2.0`). Nothing is traced from or modelled on any
//! vendor's artwork (AGENTS.md §1).
//!
//! Style: line icons in the theme's ink colour with one or two fixed accent colours. When an icon
//! is disabled everything is drawn in ink at ~35% alpha.

use std::f32::consts::PI;

use egui::epaint::{Mesh, PathShape, TextShape};
use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, pos2, vec2};

/// Accent blue.
pub const BLUE: Color32 = Color32::from_rgb(0x2E, 0x6F, 0xD8);
/// Accent orange.
pub const ORANGE: Color32 = Color32::from_rgb(0xF2, 0x6B, 0x1D);
/// Accent green.
pub const GREEN: Color32 = Color32::from_rgb(0x2E, 0xA3, 0x6F);
/// Accent purple.
pub const PURPLE: Color32 = Color32::from_rgb(0x8A, 0x5C, 0xD8);
/// Accent red.
pub const RED: Color32 = Color32::from_rgb(0xD8, 0x3F, 0x6B);
/// Accent gold.
pub const GOLD: Color32 = Color32::from_rgb(0xE3, 0xB2, 0x1C);
/// Fixed dark ink used for marks drawn on top of gold fills (readable in both themes).
const DARK: Color32 = Color32::from_rgb(0x2B, 0x2B, 0x2B);

macro_rules! icons {
    ($($(#[$m:meta])* $v:ident => $n:literal,)*) => {
        /// Every icon DeckCraft draws.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Icon {
            $($(#[$m])* $v,)*
        }

        impl Icon {
            /// All icons, in declaration order.
            pub const ALL: &'static [Icon] = &[$(Icon::$v,)*];

            /// Stable kebab-case name, e.g. `"format-painter"`.
            pub fn name(self) -> &'static str {
                match self {
                    $(Icon::$v => $n,)*
                }
            }

            /// Inverse of [`Icon::name`].
            pub fn from_name(s: &str) -> Option<Icon> {
                match s {
                    $($n => Some(Icon::$v),)*
                    _ => None,
                }
            }
        }
    };
}

mod draw;
mod geometry;

use draw::draw;
use geometry::Pen;
#[cfg(test)]
use geometry::{star_pts, triangulate};

icons! {
    // Clipboard & slides
    Paste => "paste",
    Cut => "cut",
    Copy => "copy",
    FormatPainter => "format-painter",
    NewSlide => "new-slide",
    Layout => "layout",
    ResetSlide => "reset-slide",
    Section => "section",
    DuplicateSlide => "duplicate-slide",
    DeleteSlide => "delete-slide",
    HideSlide => "hide-slide",
    Slide => "slide",
    // Font
    FontGrow => "font-grow",
    FontShrink => "font-shrink",
    ClearFormatting => "clear-formatting",
    Bold => "bold",
    Italic => "italic",
    Underline => "underline",
    Strikethrough => "strikethrough",
    Superscript => "superscript",
    Subscript => "subscript",
    CharSpacing => "char-spacing",
    ChangeCase => "change-case",
    Highlight => "highlight",
    FontColor => "font-color",
    TextShadow => "text-shadow",
    // Paragraph
    Bullets => "bullets",
    Numbering => "numbering",
    DecreaseIndent => "decrease-indent",
    IncreaseIndent => "increase-indent",
    LineSpacing => "line-spacing",
    Columns => "columns",
    AlignLeft => "align-left",
    AlignCenter => "align-center",
    AlignRight => "align-right",
    Justify => "justify",
    TextDirection => "text-direction",
    AlignText => "align-text",
    ConvertSmartArt => "convert-smart-art",
    // Drawing
    Shapes => "shapes",
    Arrange => "arrange",
    QuickStyles => "quick-styles",
    ShapeFill => "shape-fill",
    ShapeOutline => "shape-outline",
    ShapeEffects => "shape-effects",
    TextFill => "text-fill",
    TextOutline => "text-outline",
    TextEffects => "text-effects",
    EditShape => "edit-shape",
    MergeShapes => "merge-shapes",
    // Insert
    Table => "table",
    Picture => "picture",
    Screenshot => "screenshot",
    Icons => "icons",
    Models3d => "models-3d",
    SmartArt => "smart-art",
    Chart => "chart",
    Zoom => "zoom",
    Link => "link",
    Action => "action",
    Comment => "comment",
    TextBox => "text-box",
    HeaderFooter => "header-footer",
    WordArt => "word-art",
    DateTime => "date-time",
    SlideNumber => "slide-number",
    Object => "object",
    Equation => "equation",
    Symbol => "symbol",
    Video => "video",
    Audio => "audio",
    ScreenRecording => "screen-recording",
    Cameo => "cameo",
    // Draw
    Pen => "pen",
    Pencil => "pencil",
    HighlighterPen => "highlighter-pen",
    Eraser => "eraser",
    LassoSelect => "lasso-select",
    InkToShape => "ink-to-shape",
    InkToText => "ink-to-text",
    InkToMath => "ink-to-math",
    Ruler => "ruler",
    // Design
    Themes => "themes",
    Variants => "variants",
    Colors => "colors",
    Fonts => "fonts",
    BackgroundStyles => "background-styles",
    SlideSize => "slide-size",
    FormatBackground => "format-background",
    DesignIdeas => "design-ideas",
    // Transitions / animations
    Preview => "preview",
    EffectOptions => "effect-options",
    Sound => "sound",
    Duration => "duration",
    ApplyToAll => "apply-to-all",
    AnimEntrance => "anim-entrance",
    AnimEmphasis => "anim-emphasis",
    AnimExit => "anim-exit",
    AnimPath => "anim-path",
    AnimationPane => "animation-pane",
    Trigger => "trigger",
    AnimationPainter => "animation-painter",
    AddAnimation => "add-animation",
    // Slide show
    PlayFromStart => "play-from-start",
    PlayFromCurrent => "play-from-current",
    PresenterView => "presenter-view",
    CustomShow => "custom-show",
    SetUpShow => "set-up-show",
    RehearseTimings => "rehearse-timings",
    Record => "record",
    Subtitles => "subtitles",
    Coach => "coach",
    // Review
    Spelling => "spelling",
    Thesaurus => "thesaurus",
    Accessibility => "accessibility",
    Translate => "translate",
    Language => "language",
    NewComment => "new-comment",
    DeleteComment => "delete-comment",
    PrevComment => "prev-comment",
    NextComment => "next-comment",
    ShowComments => "show-comments",
    Compare => "compare",
    ReadOnly => "read-only",
    // View
    ViewNormal => "view-normal",
    ViewOutline => "view-outline",
    ViewSorter => "view-sorter",
    ViewNotesPage => "view-notes-page",
    ViewReading => "view-reading",
    SlideMaster => "slide-master",
    HandoutMaster => "handout-master",
    NotesMaster => "notes-master",
    Gridlines => "gridlines",
    Guides => "guides",
    Notes => "notes",
    ZoomGlass => "zoom-glass",
    FitToWindow => "fit-to-window",
    Grayscale => "grayscale",
    NewWindow => "new-window",
    Macros => "macros",
    // Arrange
    BringForward => "bring-forward",
    SendBackward => "send-backward",
    BringToFront => "bring-to-front",
    SendToBack => "send-to-back",
    Group => "group",
    Ungroup => "ungroup",
    AlignObjects => "align-objects",
    AlignObjLeft => "align-obj-left",
    AlignObjCenter => "align-obj-center",
    AlignObjRight => "align-obj-right",
    AlignObjTop => "align-obj-top",
    AlignObjMiddle => "align-obj-middle",
    AlignObjBottom => "align-obj-bottom",
    DistributeH => "distribute-h",
    DistributeV => "distribute-v",
    RotateLeft => "rotate-left",
    RotateRight => "rotate-right",
    FlipH => "flip-h",
    FlipV => "flip-v",
    SelectionPane => "selection-pane",
    Rotate => "rotate",
    // Picture
    Crop => "crop",
    Corrections => "corrections",
    ColorAdjust => "color-adjust",
    ArtisticEffects => "artistic-effects",
    Transparency => "transparency",
    CompressPictures => "compress-pictures",
    ChangePicture => "change-picture",
    ResetPicture => "reset-picture",
    RemoveBackground => "remove-background",
    AltText => "alt-text",
    // Table
    InsertAbove => "insert-above",
    InsertBelow => "insert-below",
    InsertLeft => "insert-left",
    InsertRight => "insert-right",
    DeleteTable => "delete-table",
    MergeCells => "merge-cells",
    SplitCells => "split-cells",
    Borders => "borders",
    Shading => "shading",
    // Chart
    ChartElements => "chart-elements",
    ChartStyles => "chart-styles",
    SwitchRowCol => "switch-row-col",
    EditData => "edit-data",
    ChangeChartType => "change-chart-type",
    // Media
    Play => "play",
    Pause => "pause",
    Stop => "stop",
    TrimMedia => "trim-media",
    Volume => "volume",
    Loop => "loop",
    Bookmark => "bookmark",
    // Window chrome
    Home => "home",
    Save => "save",
    Undo => "undo",
    Redo => "redo",
    More => "more",
    Search => "search",
    Share => "share",
    CommentsBubble => "comments-bubble",
    File => "file",
    Folder => "folder",
    Export => "export",
    Print => "print",
    Settings => "settings",
    Help => "help",
    Info => "info",
    Warning => "warning",
    Lock => "lock",
    /// Open padlock (Selection pane: an object that isn't locked).
    Unlock => "unlock",
    Eye => "eye",
    EyeOff => "eye-off",
    Close => "close",
    Check => "check",
    Plus => "plus",
    Minus => "minus",
    ChevronDown => "chevron-down",
    ChevronUp => "chevron-up",
    ChevronLeft => "chevron-left",
    ChevronRight => "chevron-right",
    DragHandle => "drag-handle",
    Star => "star",
    Image => "image",
    Sparkle => "sparkle",
    Palette => "palette",
    Grid => "grid",
    List => "list",
    Collapse => "collapse",
    Expand => "expand",
    Pin => "pin",
    // File tab (Backstage)
    BackCircle => "back-circle",
    SaveAs => "save-as",
    Account => "account",
    Recent => "recent",
    NewFile => "new-file",
    OpenFolder => "open-folder",
    // Status bar (small, monochrome)
    NotesSmall => "notes-small",
    CommentsSmall => "comments-small",
    ViewNormalSmall => "view-normal-small",
    ViewSorterSmall => "view-sorter-small",
    ViewReadingSmall => "view-reading-small",
    ViewShowSmall => "view-show-small",
    FitSmall => "fit-small",
    ZoomIn => "zoom-in",
    ZoomOut => "zoom-out",
}

/// Stroke width (in points) for an icon drawn at `size` points: ≈ size/16 at 16 pt, size/20 at
/// 32 pt and above, never thinner than 1.0.
fn stroke_width(size: f32) -> f32 {
    let t = ((size - 16.0) / 16.0).clamp(0.0, 1.0);
    (size / (16.0 + 4.0 * t)).max(1.0)
}

/// Paint `icon` into `rect` (square or not; draw centred in the largest square). `ink` is the
/// foreground (text) colour for the current theme; accents are fixed colours (see above), but when
/// `disabled` is true draw everything in `ink` at ~35% alpha.
pub fn paint(painter: &egui::Painter, rect: egui::Rect, icon: Icon, ink: egui::Color32, disabled: bool) {
    paint_state(painter, rect, icon, ink, disabled, false);
}

pub(crate) fn paint_state(painter: &egui::Painter, rect: egui::Rect, icon: Icon, ink: egui::Color32, disabled: bool, selected: bool) {
    let glyph = match icon {
        Icon::Paste => "\u{e196}",
        Icon::Cut => "\u{eae0}",
        Icon::Copy => "\u{e1ca}",
        Icon::FormatPainter => "\u{e6f0}",
        Icon::NewSlide => "\u{e236}",
        Icon::Layout => "\u{e6d6}",
        Icon::Bold => "\u{e5be}",
        Icon::Italic => "\u{e5c0}",
        Icon::Underline => "\u{e5c4}",
        Icon::Strikethrough => "\u{e5c2}",
        Icon::Bullets => "\u{e2f2}",
        Icon::Numbering => "\u{e2f6}",
        Icon::AlignLeft => "\u{e484}",
        Icon::AlignCenter => "\u{e480}",
        Icon::AlignRight => "\u{e486}",
        Icon::Justify => "\u{e482}",
        Icon::Shapes => "\u{ec5e}",
        Icon::Arrange => "\u{e466}",
        Icon::QuickStyles => "\u{e5b8}",
        Icon::ShapeFill => "\u{e392}",
        Icon::ShapeOutline => "\u{e45e}",
        Icon::Table => "\u{e476}",
        Icon::Picture => "\u{e2ca}",
        Icon::Screenshot => "\u{e10e}",
        Icon::Icons => "\u{e436}",
        Icon::Models3d => "\u{e1da}",
        Icon::Chart => "\u{e150}",
        Icon::Zoom => "\u{e30c}",
        Icon::Link => "\u{e2e2}",
        Icon::Comment => "\u{e168}",
        Icon::TextBox => "\u{eb0a}",
        Icon::DateTime => "\u{e108}",
        Icon::Equation => "\u{eab8}",
        Icon::Video => "\u{e4da}",
        Icon::Audio => "\u{e44a}",
        Icon::Pen => "\u{e3aa}",
        Icon::Pencil => "\u{e3ae}",
        Icon::Eraser => "\u{e21e}",
        Icon::Ruler => "\u{e6b8}",
        Icon::Themes => "\u{e5b8}",
        Icon::Colors => "\u{e6c8}",
        Icon::Fonts => "\u{e6ee}",
        Icon::SlideSize => "\u{e3f0}",
        Icon::Preview => "\u{e3d0}",
        Icon::Sound => "\u{e44a}",
        Icon::Duration => "\u{e492}",
        Icon::PlayFromStart => "\u{e3d0}",
        Icon::PlayFromCurrent => "\u{e3d0}",
        Icon::PresenterView => "\u{e32e}",
        Icon::Record => "\u{e3ee}",
        Icon::Subtitles => "\u{e1a4}",
        Icon::Accessibility => "\u{ecfe}",
        Icon::Translate => "\u{e4a2}",
        Icon::Language => "\u{e288}",
        Icon::NewComment => "\u{e16c}",
        Icon::ShowComments => "\u{e168}",
        Icon::Gridlines => "\u{e296}",
        Icon::Guides => "\u{e1d6}",
        Icon::Notes => "\u{e348}",
        Icon::ZoomGlass => "\u{e30c}",
        Icon::NewWindow => "\u{e5da}",
        Icon::BringForward => "\u{e08e}",
        Icon::SendBackward => "\u{e03e}",
        Icon::BringToFront => "\u{e066}",
        Icon::SendToBack => "\u{e05c}",
        Icon::Group => "\u{e746}",
        Icon::Ungroup => "\u{e69e}",
        Icon::RotateLeft => "\u{e038}",
        Icon::RotateRight => "\u{e036}",
        Icon::Rotate => "\u{e094}",
        Icon::SelectionPane => "\u{eab6}",
        Icon::Crop => "\u{e1d4}",
        Icon::Corrections => "\u{e432}",
        Icon::Transparency => "\u{e8c4}",
        Icon::ChangePicture => "\u{e2ca}",
        Icon::Borders => "\u{e476}",
        Icon::Shading => "\u{e392}",
        Icon::EditData => "\u{e3ae}",
        Icon::Play => "\u{e3d0}",
        Icon::Pause => "\u{e39e}",
        Icon::Stop => "\u{e46c}",
        Icon::Volume => "\u{e44a}",
        Icon::Loop => "\u{e3f6}",
        Icon::Bookmark => "\u{e0e8}",
        Icon::Home => "\u{e2c2}",
        Icon::Save => "\u{e248}",
        Icon::Undo => "\u{e08a}",
        Icon::Redo => "\u{e08c}",
        Icon::More => "\u{e1fe}",
        Icon::Search => "\u{e30c}",
        Icon::Share => "\u{e408}",
        Icon::CommentsBubble => "\u{e168}",
        Icon::File => "\u{e230}",
        Icon::Folder => "\u{e24a}",
        Icon::Export => "\u{eaf0}",
        Icon::Print => "\u{e3dc}",
        Icon::Settings => "\u{e270}",
        Icon::Help => "\u{e3e8}",
        Icon::Info => "\u{e2ce}",
        Icon::Warning => "\u{e4e0}",
        Icon::Lock => "\u{e2fa}",
        Icon::Unlock => "\u{e306}",
        Icon::Eye => "\u{e220}",
        Icon::EyeOff => "\u{e224}",
        Icon::Close => "\u{e4f6}",
        Icon::Check => "\u{e182}",
        Icon::Sparkle => "\u{e6a2}",
        Icon::Palette => "\u{e6c8}",
        Icon::Spelling => "\u{f1005}",
        _ => "",
    };
    let size = rect.width().min(rect.height());
    if !size.is_finite() || size <= 0.5 {
        return;
    }
    let family = egui::FontFamily::Name(if selected { "phosphor-fill".into() } else { "phosphor".into() });
    if !glyph.is_empty() && painter.ctx().fonts(|fonts| fonts.definitions().families.contains_key(&family)) {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            glyph,
            egui::FontId::new(size, family),
            if disabled { ink.gamma_multiply(0.35) } else { ink },
        );
        return;
    }
    let origin = rect.center() - vec2(size * 0.5, size * 0.5);
    let pen = Pen { p: painter, o: origin, k: size / 24.0, sw: stroke_width(size), ink, dis: disabled };
    draw(&pen, icon);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn names_round_trip_and_are_unique() {
        let mut seen = HashSet::new();
        let mut names = HashSet::new();
        for &icon in Icon::ALL {
            assert!(seen.insert(icon), "duplicate icon {icon:?} in ALL");
            let name = icon.name();
            assert!(names.insert(name), "duplicate name {name}");
            assert_eq!(Icon::from_name(name), Some(icon));
            assert!(name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'), "{name} is not kebab-case");
        }
        assert_eq!(Icon::from_name("no-such-icon"), None);
    }

    #[test]
    fn stroke_width_scales() {
        assert!((stroke_width(16.0) - 1.0).abs() < 1e-6);
        assert!((stroke_width(32.0) - 1.6).abs() < 1e-6);
        assert!((stroke_width(8.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn triangulates_concave_polygon() {
        let pts: Vec<Pos2> = star_pts((0.0, 0.0), 10.0, 4.0, 5, 0.0).into_iter().map(|(x, y)| pos2(x, y)).collect();
        assert_eq!(triangulate(&pts).len(), pts.len() - 2);
    }
}
