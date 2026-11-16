#![allow(clippy::unnecessary_cast)]

use crate::{App, El, Handler, ScrollPos, Ui, Value};
use oxent_sys as sys;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlexAlign {
    Auto = sys::XENT_FLEX_ALIGN_AUTO as u32,
    Start = sys::XENT_FLEX_ALIGN_START as u32,
    End = sys::XENT_FLEX_ALIGN_END as u32,
    Center = sys::XENT_FLEX_ALIGN_CENTER as u32,
    Stretch = sys::XENT_FLEX_ALIGN_STRETCH as u32,
    Baseline = sys::XENT_FLEX_ALIGN_BASELINE as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlexJustify {
    Start = sys::XENT_FLEX_JUSTIFY_START as u32,
    End = sys::XENT_FLEX_JUSTIFY_END as u32,
    Center = sys::XENT_FLEX_JUSTIFY_CENTER as u32,
    SpaceBetween = sys::XENT_FLEX_JUSTIFY_SPACE_BETWEEN as u32,
    SpaceAround = sys::XENT_FLEX_JUSTIFY_SPACE_AROUND as u32,
    SpaceEvenly = sys::XENT_FLEX_JUSTIFY_SPACE_EVENLY as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlexWrap {
    NoWrap = sys::XENT_FLEX_NO_WRAP as u32,
    Wrap = sys::XENT_FLEX_WRAP as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppBarMode {
    Compact = sys::XTK_APPBAR_COMPACT as u32,
    Minimal = sys::XTK_APPBAR_MINIMAL as u32,
    Hidden = sys::XTK_APPBAR_HIDDEN as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BadgeMode {
    Dot = sys::XTK_BADGE_DOT as u32,
    Number = sys::XTK_BADGE_NUMBER as u32,
    Icon = sys::XTK_BADGE_ICON as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonStyle {
    Standard = sys::XTK_BUTTON_STANDARD as u32,
    Subtle = sys::XTK_BUTTON_SUBTLE as u32,
    Text = sys::XTK_BUTTON_TEXT as u32,
    Accent = sys::XTK_BUTTON_ACCENT as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CalendarMode {
    Single = sys::XTK_CALENDAR_SINGLE as u32,
    None = sys::XTK_CALENDAR_NONE as u32,
    Multiple = sys::XTK_CALENDAR_MULTIPLE as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Clock {
    System = sys::XTK_CLOCK_SYSTEM as u32,
    H12 = sys::XTK_CLOCK_12H as u32,
    H24 = sys::XTK_CLOCK_24H as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DropOperation {
    None = sys::XTK_DROP_NONE as u32,
    Copy = sys::XTK_DROP_COPY as u32,
    Move = sys::XTK_DROP_MOVE as u32,
    Link = sys::XTK_DROP_LINK as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElementLabel {
    Default = sys::XTK_ELEMENT_LABEL_DEFAULT as u32,
    Collapsed = sys::XTK_ELEMENT_LABEL_COLLAPSED as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Inherit = sys::XTK_FONT_INHERIT as u32,
    Thin = sys::XTK_FONT_THIN as u32,
    ExtraLight = sys::XTK_FONT_EXTRA_LIGHT as u32,
    Light = sys::XTK_FONT_LIGHT as u32,
    Regular = sys::XTK_FONT_REGULAR as u32,
    Medium = sys::XTK_FONT_MEDIUM as u32,
    SemiBold = sys::XTK_FONT_SEMI_BOLD as u32,
    Bold = sys::XTK_FONT_BOLD as u32,
    ExtraBold = sys::XTK_FONT_EXTRA_BOLD as u32,
    Black = sys::XTK_FONT_BLACK as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImageStretch {
    None = sys::XTK_IMAGE_NONE as u32,
    Fill = sys::XTK_IMAGE_FILL as u32,
    Uniform = sys::XTK_IMAGE_UNIFORM as u32,
    UniformToFill = sys::XTK_IMAGE_UNIFORM_TO_FILL as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InfoBarSeverity {
    Informational = sys::XTK_INFOBAR_INFORMATIONAL as u32,
    Success = sys::XTK_INFOBAR_SUCCESS as u32,
    Warning = sys::XTK_INFOBAR_WARNING as u32,
    Error = sys::XTK_INFOBAR_ERROR as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LabelPosition {
    Bottom = sys::XTK_LABEL_BOTTOM as u32,
    Right = sys::XTK_LABEL_RIGHT as u32,
    Collapsed = sys::XTK_LABEL_COLLAPSED as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ListSelMode {
    Single = sys::XTK_LIST_SELECT_SINGLE as u32,
    None = sys::XTK_LIST_SELECT_NONE as u32,
    Multiple = sys::XTK_LIST_SELECT_MULTIPLE as u32,
    Extended = sys::XTK_LIST_SELECT_EXTENDED as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OverflowButton {
    Auto = sys::XTK_OVERFLOW_BUTTON_AUTO as u32,
    Visible = sys::XTK_OVERFLOW_BUTTON_VISIBLE as u32,
    Collapsed = sys::XTK_OVERFLOW_BUTTON_COLLAPSED as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PagerButtonVis {
    Auto = sys::XTK_PAGER_BUTTON_AUTO as u32,
    Always = sys::XTK_PAGER_BUTTON_ALWAYS as u32,
    HiddenEdge = sys::XTK_PAGER_BUTTON_HIDDEN_EDGE as u32,
    None = sys::XTK_PAGER_BUTTON_NONE as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PagerDisplayMode {
    Auto = sys::XTK_PAGER_DISPLAY_AUTO as u32,
    NumberPanel = sys::XTK_PAGER_DISPLAY_NUMBER_PANEL as u32,
    NumberText = sys::XTK_PAGER_DISPLAY_NUMBER_TEXT as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PipsNavVis {
    Collapsed = sys::XTK_PIPS_NAV_COLLAPSED as u32,
    Visible = sys::XTK_PIPS_NAV_VISIBLE as u32,
    OnHover = sys::XTK_PIPS_NAV_ON_HOVER as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProgressState {
    Normal = sys::XTK_PROGRESS_NORMAL as u32,
    Paused = sys::XTK_PROGRESS_PAUSED as u32,
    Error = sys::XTK_PROGRESS_ERROR as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PullDirection {
    TopToBottom = sys::XTK_PULL_TOP_TO_BOTTOM as u32,
    LeftToRight = sys::XTK_PULL_LEFT_TO_RIGHT as u32,
    RightToLeft = sys::XTK_PULL_RIGHT_TO_LEFT as u32,
    BottomToTop = sys::XTK_PULL_BOTTOM_TO_TOP as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScrollBarIndicator {
    None = sys::XTK_SCROLLBAR_INDICATOR_NONE as u32,
    Touch = sys::XTK_SCROLLBAR_INDICATOR_TOUCH as u32,
    Mouse = sys::XTK_SCROLLBAR_INDICATOR_MOUSE as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SliderSnap {
    Step = sys::XTK_SLIDER_SNAP_STEP as u32,
    Ticks = sys::XTK_SLIDER_SNAP_TICKS as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SortDirection {
    None = sys::XTK_SORT_NONE as u32,
    Ascending = sys::XTK_SORT_ASCENDING as u32,
    Descending = sys::XTK_SORT_DESCENDING as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Spectrum {
    HueSaturation = sys::XTK_SPECTRUM_HUE_SATURATION as u32,
    HueValue = sys::XTK_SPECTRUM_HUE_VALUE as u32,
    SaturationValue = sys::XTK_SPECTRUM_SATURATION_VALUE as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SplitViewDisplayMode {
    Inline = sys::XTK_SPLITVIEW_INLINE as u32,
    CompactInline = sys::XTK_SPLITVIEW_COMPACT_INLINE as u32,
    Overlay = sys::XTK_SPLITVIEW_OVERLAY as u32,
    CompactOverlay = sys::XTK_SPLITVIEW_COMPACT_OVERLAY as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SplitViewPanePlacement {
    Left = sys::XTK_SPLITVIEW_LEFT as u32,
    Right = sys::XTK_SPLITVIEW_RIGHT as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StackDirection {
    Column = sys::XTK_STACK_COLUMN as u32,
    Row = sys::XTK_STACK_ROW as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TeachingTipPlacement {
    Auto = sys::XTK_TIP_PLACE_AUTO as u32,
    Top = sys::XTK_TIP_PLACE_TOP as u32,
    Bottom = sys::XTK_TIP_PLACE_BOTTOM as u32,
    Left = sys::XTK_TIP_PLACE_LEFT as u32,
    Right = sys::XTK_TIP_PLACE_RIGHT as u32,
    TopRight = sys::XTK_TIP_PLACE_TOP_RIGHT as u32,
    TopLeft = sys::XTK_TIP_PLACE_TOP_LEFT as u32,
    BottomRight = sys::XTK_TIP_PLACE_BOTTOM_RIGHT as u32,
    BottomLeft = sys::XTK_TIP_PLACE_BOTTOM_LEFT as u32,
    LeftTop = sys::XTK_TIP_PLACE_LEFT_TOP as u32,
    LeftBottom = sys::XTK_TIP_PLACE_LEFT_BOTTOM as u32,
    RightTop = sys::XTK_TIP_PLACE_RIGHT_TOP as u32,
    RightBottom = sys::XTK_TIP_PLACE_RIGHT_BOTTOM as u32,
    Center = sys::XTK_TIP_PLACE_CENTER as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TeachingTipTail {
    Auto = sys::XTK_TIP_TAIL_AUTO as u32,
    Visible = sys::XTK_TIP_TAIL_VISIBLE as u32,
    Collapsed = sys::XTK_TIP_TAIL_COLLAPSED as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextAlign {
    Left = sys::XTK_TEXT_LEFT as u32,
    Center = sys::XTK_TEXT_CENTER as u32,
    Right = sys::XTK_TEXT_RIGHT as u32,
    Justify = sys::XTK_TEXT_JUSTIFY as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextTrim {
    None = sys::XTK_TRIM_NONE as u32,
    Char = sys::XTK_TRIM_CHAR as u32,
    CharEllipsis = sys::XTK_TRIM_CHAR_ELLIPSIS as u32,
    Word = sys::XTK_TRIM_WORD as u32,
    WordEllipsis = sys::XTK_TRIM_WORD_ELLIPSIS as u32,
    PathEllipsis = sys::XTK_TRIM_PATH_ELLIPSIS as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TickPlacement {
    Inline = sys::XTK_TICK_INLINE as u32,
    TopLeft = sys::XTK_TICK_TOP_LEFT as u32,
    BottomRight = sys::XTK_TICK_BOTTOM_RIGHT as u32,
    Outside = sys::XTK_TICK_OUTSIDE as u32,
    None = sys::XTK_TICK_NONE as u32,
}

pub struct Container<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn container(&'ui self) -> Container<'ui, A> {
        Container(self.el(sys::XTK_CTRL_CONTAINER, &raw const sys::flux_ctrl_container))
    }
}

impl<'ui, A: App> Container<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn items(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_ITEMS, Value::Enum(v as u32));
        self
    }
    pub fn justify(self, v: FlexJustify) -> Self {
        self.0.prop(sys::XTK_PROP_JUSTIFY_CONTENT, Value::Enum(v as u32));
        self
    }
    pub fn gap(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GAP, Value::F32(v));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn fill(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FILL, Value::Bool(v));
        self
    }
    pub fn fitw(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_WIDTH, Value::Bool(v));
        self
    }
    pub fn fith(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_HEIGHT, Value::Bool(v));
        self
    }
    pub fn wrap(self, v: FlexWrap) -> Self {
        self.0.prop(sys::XTK_PROP_WRAP, Value::Enum(v as u32));
        self
    }
    pub fn drop(self, v: DropOperation) -> Self {
        self.0.prop(sys::XTK_PROP_DROP_OPERATION, Value::Enum(v as u32));
        self
    }
    pub fn dragtext(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_TEXT, Value::Str(v.into()));
        self
    }
    pub fn dragfiles(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_FILES, Value::Str(v.into()));
        self
    }
    pub fn on_draghover(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_HOVER, Handler::Bool(f));
        self
    }
    pub fn on_dropfiles(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_FILES, Handler::Str(f));
        self
    }
    pub fn on_droptext(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_TEXT, Handler::Str(f));
        self
    }
    pub fn on_dragcompleted(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_COMPLETED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Container<'ui, A>> for El<'ui, A> {
    fn from(b: Container<'ui, A>) -> Self {
        b.0
    }
}

pub struct Text<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn text(&'ui self) -> Text<'ui, A> {
        Text(self.el(sys::XTK_CTRL_TEXT, &raw const sys::flux_ctrl_text))
    }
}

impl<'ui, A: App> Text<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn fontsize(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_FONT_SIZE, Value::F32(v));
        self
    }
    pub fn fontweight(self, v: FontWeight) -> Self {
        self.0.prop(sys::XTK_PROP_FONT_WEIGHT, Value::Enum(v as u32));
        self
    }
    pub fn textalign(self, v: TextAlign) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT_ALIGN, Value::Enum(v as u32));
        self
    }
    pub fn fg(self, v: u32) -> Self {
        self.0.prop(sys::XTK_PROP_FOREGROUND, Value::Color(v));
        self
    }
    pub fn maxlines(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_LINES, Value::I32(v));
        self
    }
    pub fn trim(self, v: TextTrim) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT_TRIM, Value::Enum(v as u32));
        self
    }
    pub fn textwrap(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT_WRAP, Value::Bool(v));
        self
    }
    pub fn selectable(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTABLE, Value::Bool(v));
        self
    }
}

impl<'ui, A: App> From<Text<'ui, A>> for El<'ui, A> {
    fn from(b: Text<'ui, A>) -> Self {
        b.0
    }
}

pub struct Button<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn button(&'ui self) -> Button<'ui, A> {
        Button(self.el(sys::XTK_CTRL_BUTTON, &raw const sys::flux_ctrl_button))
    }
}

impl<'ui, A: App> Button<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn style(self, v: ButtonStyle) -> Self {
        self.0.prop(sys::XTK_PROP_BUTTON_STYLE, Value::Enum(v as u32));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Button<'ui, A>> for El<'ui, A> {
    fn from(b: Button<'ui, A>) -> Self {
        b.0
    }
}

pub struct Toggle<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn toggle(&'ui self) -> Toggle<'ui, A> {
        Toggle(self.el(sys::XTK_CTRL_TOGGLE, &raw const sys::flux_ctrl_toggle))
    }
}

impl<'ui, A: App> Toggle<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn checked(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_CHECKED, Value::Bool(v));
        self
    }
    pub fn on_change(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_BOOL, Handler::Bool(f));
        self
    }
}

impl<'ui, A: App> From<Toggle<'ui, A>> for El<'ui, A> {
    fn from(b: Toggle<'ui, A>) -> Self {
        b.0
    }
}

pub struct Checkbox<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn checkbox(&'ui self) -> Checkbox<'ui, A> {
        Checkbox(self.el(sys::XTK_CTRL_CHECKBOX, &raw const sys::flux_ctrl_checkbox))
    }
}

impl<'ui, A: App> Checkbox<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn checked(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_CHECKED, Value::Bool(v));
        self
    }
    pub fn indeterminate(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_INDETERMINATE, Value::Bool(v));
        self
    }
    pub fn threestate(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_IS_THREE_STATE, Value::Bool(v));
        self
    }
    pub fn on_change(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_BOOL, Handler::Bool(f));
        self
    }
    pub fn on_state(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_I32, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Checkbox<'ui, A>> for El<'ui, A> {
    fn from(b: Checkbox<'ui, A>) -> Self {
        b.0
    }
}

pub struct Radio<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn radio(&'ui self) -> Radio<'ui, A> {
        Radio(self.el(sys::XTK_CTRL_RADIO, &raw const sys::flux_ctrl_radio))
    }
}

impl<'ui, A: App> Radio<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn checked(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_CHECKED, Value::Bool(v));
        self
    }
    pub fn on_change(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_BOOL, Handler::Bool(f));
        self
    }
}

impl<'ui, A: App> From<Radio<'ui, A>> for El<'ui, A> {
    fn from(b: Radio<'ui, A>) -> Self {
        b.0
    }
}

pub struct Switch<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn switch(&'ui self) -> Switch<'ui, A> {
        Switch(self.el(sys::XTK_CTRL_SWITCH, &raw const sys::flux_ctrl_switch))
    }
}

impl<'ui, A: App> Switch<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn checked(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_CHECKED, Value::Bool(v));
        self
    }
    pub fn on_change(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_BOOL, Handler::Bool(f));
        self
    }
}

impl<'ui, A: App> From<Switch<'ui, A>> for El<'ui, A> {
    fn from(b: Switch<'ui, A>) -> Self {
        b.0
    }
}

pub struct Slider<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn slider(&'ui self) -> Slider<'ui, A> {
        Slider(self.el(sys::XTK_CTRL_SLIDER, &raw const sys::flux_ctrl_slider))
    }
}

impl<'ui, A: App> Slider<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn min(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MIN_F32, Value::F32(v));
        self
    }
    pub fn max(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_F32, Value::F32(v));
        self
    }
    pub fn value(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_VALUE_F32, Value::F32(v));
        self
    }
    pub fn step(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_STEP_F32, Value::F32(v));
        self
    }
    pub fn tick(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_TICK_FREQUENCY, Value::F32(v));
        self
    }
    pub fn tickplace(self, v: TickPlacement) -> Self {
        self.0.prop(sys::XTK_PROP_TICK_PLACEMENT, Value::Enum(v as u32));
        self
    }
    pub fn snapsto(self, v: SliderSnap) -> Self {
        self.0.prop(sys::XTK_PROP_SLIDER_SNAPS_TO, Value::Enum(v as u32));
        self
    }
    pub fn smallchange(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SMALL_CHANGE, Value::F32(v));
        self
    }
    pub fn largechange(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_LARGE_CHANGE, Value::F32(v));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn range(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_IS_RANGE, Value::Bool(v));
        self
    }
    pub fn rangestart(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_RANGE_START, Value::F32(v));
        self
    }
    pub fn rangeend(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_RANGE_END, Value::F32(v));
        self
    }
    pub fn on_change(self, f: fn(f32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_F32, Handler::F32(f));
        self
    }
    pub fn on_range(self, f: fn(f32, f32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_RANGE, Handler::Range(f));
        self
    }
}

impl<'ui, A: App> From<Slider<'ui, A>> for El<'ui, A> {
    fn from(b: Slider<'ui, A>) -> Self {
        b.0
    }
}

pub struct Textbox<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn textbox(&'ui self) -> Textbox<'ui, A> {
        Textbox(self.el(sys::XTK_CTRL_TEXTBOX, &raw const sys::flux_ctrl_textbox))
    }
}

impl<'ui, A: App> Textbox<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn placeholder(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_PLACEHOLDER, Value::Str(v.into()));
        self
    }
    pub fn multiline(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_MULTILINE, Value::Bool(v));
        self
    }
    pub fn readonly(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_READ_ONLY, Value::Bool(v));
        self
    }
    pub fn on_change(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_STRING, Handler::Str(f));
        self
    }
}

impl<'ui, A: App> From<Textbox<'ui, A>> for El<'ui, A> {
    fn from(b: Textbox<'ui, A>) -> Self {
        b.0
    }
}

pub struct Scroll<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn scroll(&'ui self) -> Scroll<'ui, A> {
        Scroll(self.el(sys::XTK_CTRL_SCROLL, &raw const sys::flux_ctrl_scroll))
    }
}

impl<'ui, A: App> Scroll<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn items(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_ITEMS, Value::Enum(v as u32));
        self
    }
    pub fn justify(self, v: FlexJustify) -> Self {
        self.0.prop(sys::XTK_PROP_JUSTIFY_CONTENT, Value::Enum(v as u32));
        self
    }
    pub fn gap(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GAP, Value::F32(v));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn fill(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FILL, Value::Bool(v));
        self
    }
    pub fn fitw(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_WIDTH, Value::Bool(v));
        self
    }
    pub fn fith(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_HEIGHT, Value::Bool(v));
        self
    }
    pub fn wrap(self, v: FlexWrap) -> Self {
        self.0.prop(sys::XTK_PROP_WRAP, Value::Enum(v as u32));
        self
    }
    pub fn x(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SCROLL_X, Value::F32(v));
        self
    }
    pub fn y(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SCROLL_Y, Value::F32(v));
        self
    }
    pub fn on_scroll(self, f: fn(ScrollPos) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SCROLL, Handler::Scroll(f));
        self
    }
}

impl<'ui, A: App> From<Scroll<'ui, A>> for El<'ui, A> {
    fn from(b: Scroll<'ui, A>) -> Self {
        b.0
    }
}

pub struct Image<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn image(&'ui self) -> Image<'ui, A> {
        Image(self.el(sys::XTK_CTRL_IMAGE, &raw const sys::flux_ctrl_image))
    }
}

impl<'ui, A: App> Image<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn src(self, v: u64) -> Self {
        self.0.prop(sys::XTK_PROP_IMAGE_SOURCE, Value::Resource(v));
        self
    }
    pub fn fit(self, v: ImageStretch) -> Self {
        self.0.prop(sys::XTK_PROP_IMAGE_STRETCH, Value::Enum(v as u32));
        self
    }
    pub fn nl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_NINE_GRID_LEFT, Value::F32(v));
        self
    }
    pub fn nt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_NINE_GRID_TOP, Value::F32(v));
        self
    }
    pub fn nr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_NINE_GRID_RIGHT, Value::F32(v));
        self
    }
    pub fn nb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_NINE_GRID_BOTTOM, Value::F32(v));
        self
    }
    pub fn on_opened(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_IMAGE_OPENED, Handler::Value(msg));
        self
    }
    pub fn on_failed(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_IMAGE_FAILED, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Image<'ui, A>> for El<'ui, A> {
    fn from(b: Image<'ui, A>) -> Self {
        b.0
    }
}

pub struct Progress<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn progress(&'ui self) -> Progress<'ui, A> {
        Progress(self.el(sys::XTK_CTRL_PROGRESS, &raw const sys::flux_ctrl_progress))
    }
}

impl<'ui, A: App> Progress<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn min(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MIN_F32, Value::F32(v));
        self
    }
    pub fn max(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_F32, Value::F32(v));
        self
    }
    pub fn value(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_VALUE_F32, Value::F32(v));
        self
    }
    pub fn indeterminate(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_INDETERMINATE, Value::Bool(v));
        self
    }
    pub fn state(self, v: ProgressState) -> Self {
        self.0.prop(sys::XTK_PROP_PROGRESS_STATE, Value::Enum(v as u32));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn border_width(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PROGRESS_BORDER_WIDTH, Value::F32(v));
        self
    }
}

impl<'ui, A: App> From<Progress<'ui, A>> for El<'ui, A> {
    fn from(b: Progress<'ui, A>) -> Self {
        b.0
    }
}

pub struct List<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn list(&'ui self) -> List<'ui, A> {
        List(self.el(sys::XTK_CTRL_LIST, &raw const sys::flux_ctrl_list))
    }
}

impl<'ui, A: App> List<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn itemh(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_HEIGHT, Value::F32(v));
        self
    }
    pub fn itemw(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_WIDTH, Value::F32(v));
        self
    }
    pub fn ncol(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_COLUMN_COUNT, Value::I32(v));
        self
    }
    pub fn mode(self, v: ListSelMode) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTION_MODE, Value::Enum(v as u32));
        self
    }
    pub fn first(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_REALIZED_FIRST, Value::I32(v));
        self
    }
    pub fn rectsel(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_RECTANGLE_SELECTION, Value::Bool(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<List<'ui, A>> for El<'ui, A> {
    fn from(b: List<'ui, A>) -> Self {
        b.0
    }
}

pub struct Tab<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn tab(&'ui self) -> Tab<'ui, A> {
        Tab(self.el(sys::XTK_CTRL_TAB, &raw const sys::flux_ctrl_tab))
    }
}

impl<'ui, A: App> Tab<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
}

impl<'ui, A: App> From<Tab<'ui, A>> for El<'ui, A> {
    fn from(b: Tab<'ui, A>) -> Self {
        b.0
    }
}

pub struct Card<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn card(&'ui self) -> Card<'ui, A> {
        Card(self.el(sys::XTK_CTRL_CARD, &raw const sys::flux_ctrl_card))
    }
}

impl<'ui, A: App> Card<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn items(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_ITEMS, Value::Enum(v as u32));
        self
    }
    pub fn justify(self, v: FlexJustify) -> Self {
        self.0.prop(sys::XTK_PROP_JUSTIFY_CONTENT, Value::Enum(v as u32));
        self
    }
    pub fn gap(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GAP, Value::F32(v));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn fill(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FILL, Value::Bool(v));
        self
    }
    pub fn fitw(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_WIDTH, Value::Bool(v));
        self
    }
    pub fn fith(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_HEIGHT, Value::Bool(v));
        self
    }
    pub fn wrap(self, v: FlexWrap) -> Self {
        self.0.prop(sys::XTK_PROP_WRAP, Value::Enum(v as u32));
        self
    }
    pub fn drop(self, v: DropOperation) -> Self {
        self.0.prop(sys::XTK_PROP_DROP_OPERATION, Value::Enum(v as u32));
        self
    }
    pub fn dragtext(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_TEXT, Value::Str(v.into()));
        self
    }
    pub fn dragfiles(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_FILES, Value::Str(v.into()));
        self
    }
    pub fn on_draghover(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_HOVER, Handler::Bool(f));
        self
    }
    pub fn on_dropfiles(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_FILES, Handler::Str(f));
        self
    }
    pub fn on_droptext(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_TEXT, Handler::Str(f));
        self
    }
    pub fn on_dragcompleted(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_COMPLETED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Card<'ui, A>> for El<'ui, A> {
    fn from(b: Card<'ui, A>) -> Self {
        b.0
    }
}

pub struct Divider<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn divider(&'ui self) -> Divider<'ui, A> {
        Divider(self.el(sys::XTK_CTRL_DIVIDER, &raw const sys::flux_ctrl_divider))
    }
}

impl<'ui, A: App> Divider<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
}

impl<'ui, A: App> From<Divider<'ui, A>> for El<'ui, A> {
    fn from(b: Divider<'ui, A>) -> Self {
        b.0
    }
}

pub struct Canvas<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn canvas(&'ui self) -> Canvas<'ui, A> {
        Canvas(self.el(sys::XTK_CTRL_CANVAS, &raw const sys::flux_ctrl_canvas))
    }
}

impl<'ui, A: App> Canvas<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
}

impl<'ui, A: App> From<Canvas<'ui, A>> for El<'ui, A> {
    fn from(b: Canvas<'ui, A>) -> Self {
        b.0
    }
}

pub struct Password<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn password(&'ui self) -> Password<'ui, A> {
        Password(self.el(sys::XTK_CTRL_PASSWORD, &raw const sys::flux_ctrl_password))
    }
}

impl<'ui, A: App> Password<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn placeholder(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_PLACEHOLDER, Value::Str(v.into()));
        self
    }
    pub fn on_change(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_STRING, Handler::Str(f));
        self
    }
}

impl<'ui, A: App> From<Password<'ui, A>> for El<'ui, A> {
    fn from(b: Password<'ui, A>) -> Self {
        b.0
    }
}

pub struct Number<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn number(&'ui self) -> Number<'ui, A> {
        Number(self.el(sys::XTK_CTRL_NUMBER, &raw const sys::flux_ctrl_number))
    }
}

impl<'ui, A: App> Number<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn value(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_VALUE_F64, Value::F64(v));
        self
    }
    pub fn min(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_MIN_F64, Value::F64(v));
        self
    }
    pub fn max(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_F64, Value::F64(v));
        self
    }
    pub fn step(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_STEP_F64, Value::F64(v));
        self
    }
    pub fn hidespin(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_HIDE_SPIN, Value::Bool(v));
        self
    }
    pub fn on_change(self, f: fn(f64) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_F64, Handler::F64(f));
        self
    }
}

impl<'ui, A: App> From<Number<'ui, A>> for El<'ui, A> {
    fn from(b: Number<'ui, A>) -> Self {
        b.0
    }
}

pub struct Link<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn link(&'ui self) -> Link<'ui, A> {
        Link(self.el(sys::XTK_CTRL_LINK, &raw const sys::flux_ctrl_link))
    }
}

impl<'ui, A: App> Link<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn url(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_URL, Value::Str(v.into()));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Link<'ui, A>> for El<'ui, A> {
    fn from(b: Link<'ui, A>) -> Self {
        b.0
    }
}

pub struct Repeat<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn repeat(&'ui self) -> Repeat<'ui, A> {
        Repeat(self.el(sys::XTK_CTRL_REPEAT, &raw const sys::flux_ctrl_repeat))
    }
}

impl<'ui, A: App> Repeat<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn style(self, v: ButtonStyle) -> Self {
        self.0.prop(sys::XTK_PROP_BUTTON_STYLE, Value::Enum(v as u32));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Repeat<'ui, A>> for El<'ui, A> {
    fn from(b: Repeat<'ui, A>) -> Self {
        b.0
    }
}

pub struct Progressring<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn progressring(&'ui self) -> Progressring<'ui, A> {
        Progressring(self.el(sys::XTK_CTRL_PROGRESSRING, &raw const sys::flux_ctrl_progressring))
    }
}

impl<'ui, A: App> Progressring<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn min(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MIN_F32, Value::F32(v));
        self
    }
    pub fn max(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_F32, Value::F32(v));
        self
    }
    pub fn value(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_VALUE_F32, Value::F32(v));
        self
    }
    pub fn indeterminate(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_INDETERMINATE, Value::Bool(v));
        self
    }
    pub fn state(self, v: ProgressState) -> Self {
        self.0.prop(sys::XTK_PROP_PROGRESS_STATE, Value::Enum(v as u32));
        self
    }
    pub fn active(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_PROGRESS_ACTIVE, Value::Bool(v));
        self
    }
}

impl<'ui, A: App> From<Progressring<'ui, A>> for El<'ui, A> {
    fn from(b: Progressring<'ui, A>) -> Self {
        b.0
    }
}

pub struct Badge<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn badge(&'ui self) -> Badge<'ui, A> {
        Badge(self.el(sys::XTK_CTRL_BADGE, &raw const sys::flux_ctrl_badge))
    }
}

impl<'ui, A: App> Badge<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn badgemode(self, v: BadgeMode) -> Self {
        self.0.prop(sys::XTK_PROP_BADGE_MODE, Value::Enum(v as u32));
        self
    }
    pub fn value(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_BADGE_VALUE, Value::I32(v));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
}

impl<'ui, A: App> From<Badge<'ui, A>> for El<'ui, A> {
    fn from(b: Badge<'ui, A>) -> Self {
        b.0
    }
}

pub struct Tooltip<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn tooltip(&'ui self) -> Tooltip<'ui, A> {
        Tooltip(self.el(sys::XTK_CTRL_TOOLTIP, &raw const sys::flux_ctrl_tooltip))
    }
}

impl<'ui, A: App> Tooltip<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn open(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_OPEN, Value::Bool(v));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
}

impl<'ui, A: App> From<Tooltip<'ui, A>> for El<'ui, A> {
    fn from(b: Tooltip<'ui, A>) -> Self {
        b.0
    }
}

pub struct Flyout<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn flyout(&'ui self) -> Flyout<'ui, A> {
        Flyout(self.el(sys::XTK_CTRL_FLYOUT, &raw const sys::flux_ctrl_flyout))
    }
}

impl<'ui, A: App> Flyout<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn open(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_OPEN, Value::Bool(v));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
}

impl<'ui, A: App> From<Flyout<'ui, A>> for El<'ui, A> {
    fn from(b: Flyout<'ui, A>) -> Self {
        b.0
    }
}

pub struct Menufly<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn menufly(&'ui self) -> Menufly<'ui, A> {
        Menufly(self.el(sys::XTK_CTRL_MENUFLY, &raw const sys::flux_ctrl_menufly))
    }
}

impl<'ui, A: App> Menufly<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn open(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_OPEN, Value::Bool(v));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
}

impl<'ui, A: App> From<Menufly<'ui, A>> for El<'ui, A> {
    fn from(b: Menufly<'ui, A>) -> Self {
        b.0
    }
}

pub struct Dropdown<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn dropdown(&'ui self) -> Dropdown<'ui, A> {
        Dropdown(self.el(sys::XTK_CTRL_DROPDOWN, &raw const sys::flux_ctrl_dropdown))
    }
}

impl<'ui, A: App> Dropdown<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn style(self, v: ButtonStyle) -> Self {
        self.0.prop(sys::XTK_PROP_BUTTON_STYLE, Value::Enum(v as u32));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Dropdown<'ui, A>> for El<'ui, A> {
    fn from(b: Dropdown<'ui, A>) -> Self {
        b.0
    }
}

pub struct Split<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn split(&'ui self) -> Split<'ui, A> {
        Split(self.el(sys::XTK_CTRL_SPLIT, &raw const sys::flux_ctrl_split))
    }
}

impl<'ui, A: App> Split<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn style(self, v: ButtonStyle) -> Self {
        self.0.prop(sys::XTK_PROP_BUTTON_STYLE, Value::Enum(v as u32));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Split<'ui, A>> for El<'ui, A> {
    fn from(b: Split<'ui, A>) -> Self {
        b.0
    }
}

pub struct Combo<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn combo(&'ui self) -> Combo<'ui, A> {
        Combo(self.el(sys::XTK_CTRL_COMBO, &raw const sys::flux_ctrl_combo))
    }
}

impl<'ui, A: App> Combo<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn placeholder(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_PLACEHOLDER, Value::Str(v.into()));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn editable(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_IS_EDITABLE, Value::Bool(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
    pub fn on_submit(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SUBMIT_STRING, Handler::Str(f));
        self
    }
}

impl<'ui, A: App> From<Combo<'ui, A>> for El<'ui, A> {
    fn from(b: Combo<'ui, A>) -> Self {
        b.0
    }
}

pub struct Expander<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn expander(&'ui self) -> Expander<'ui, A> {
        Expander(self.el(sys::XTK_CTRL_EXPANDER, &raw const sys::flux_ctrl_expander))
    }
}

impl<'ui, A: App> Expander<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
    pub fn contenth(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_CONTENT_HEIGHT, Value::F32(v));
        self
    }
    pub fn on_toggle(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_TOGGLE, Handler::Bool(f));
        self
    }
}

impl<'ui, A: App> From<Expander<'ui, A>> for El<'ui, A> {
    fn from(b: Expander<'ui, A>) -> Self {
        b.0
    }
}

pub struct Exphead<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn exphead(&'ui self) -> Exphead<'ui, A> {
        Exphead(self.el(sys::XTK_CTRL_EXPHEAD, &raw const sys::flux_ctrl_exphead))
    }
}

impl<'ui, A: App> Exphead<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn selected(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED, Value::Bool(v));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
}

impl<'ui, A: App> From<Exphead<'ui, A>> for El<'ui, A> {
    fn from(b: Exphead<'ui, A>) -> Self {
        b.0
    }
}

pub struct Expbody<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn expbody(&'ui self) -> Expbody<'ui, A> {
        Expbody(self.el(sys::XTK_CTRL_EXPBODY, &raw const sys::flux_ctrl_expbody))
    }
}

impl<'ui, A: App> Expbody<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn items(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_ITEMS, Value::Enum(v as u32));
        self
    }
    pub fn justify(self, v: FlexJustify) -> Self {
        self.0.prop(sys::XTK_PROP_JUSTIFY_CONTENT, Value::Enum(v as u32));
        self
    }
    pub fn gap(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GAP, Value::F32(v));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn fill(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FILL, Value::Bool(v));
        self
    }
    pub fn fitw(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_WIDTH, Value::Bool(v));
        self
    }
    pub fn fith(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_HEIGHT, Value::Bool(v));
        self
    }
    pub fn wrap(self, v: FlexWrap) -> Self {
        self.0.prop(sys::XTK_PROP_WRAP, Value::Enum(v as u32));
        self
    }
    pub fn drop(self, v: DropOperation) -> Self {
        self.0.prop(sys::XTK_PROP_DROP_OPERATION, Value::Enum(v as u32));
        self
    }
    pub fn dragtext(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_TEXT, Value::Str(v.into()));
        self
    }
    pub fn dragfiles(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_FILES, Value::Str(v.into()));
        self
    }
    pub fn on_draghover(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_HOVER, Handler::Bool(f));
        self
    }
    pub fn on_dropfiles(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_FILES, Handler::Str(f));
        self
    }
    pub fn on_droptext(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_TEXT, Handler::Str(f));
        self
    }
    pub fn on_dragcompleted(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_COMPLETED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Expbody<'ui, A>> for El<'ui, A> {
    fn from(b: Expbody<'ui, A>) -> Self {
        b.0
    }
}

pub struct Infobar<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn infobar(&'ui self) -> Infobar<'ui, A> {
        Infobar(self.el(sys::XTK_CTRL_INFOBAR, &raw const sys::flux_ctrl_infobar))
    }
}

impl<'ui, A: App> Infobar<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn message(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_MESSAGE, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn severity(self, v: InfoBarSeverity) -> Self {
        self.0.prop(sys::XTK_PROP_SEVERITY, Value::Enum(v as u32));
        self
    }
    pub fn closable(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_CLOSABLE, Value::Bool(v));
        self
    }
    pub fn on_close(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CLOSE, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Infobar<'ui, A>> for El<'ui, A> {
    fn from(b: Infobar<'ui, A>) -> Self {
        b.0
    }
}

pub struct Dialog<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn dialog(&'ui self) -> Dialog<'ui, A> {
        Dialog(self.el(sys::XTK_CTRL_DIALOG, &raw const sys::flux_ctrl_dialog))
    }
}

impl<'ui, A: App> Dialog<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn primary(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_PRIMARY_TEXT, Value::Str(v.into()));
        self
    }
    pub fn secondary(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_SECONDARY_TEXT, Value::Str(v.into()));
        self
    }
    pub fn close(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_CLOSE_TEXT, Value::Str(v.into()));
        self
    }
    pub fn contenth(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_CONTENT_HEIGHT, Value::F32(v));
        self
    }
    pub fn open(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_OPEN, Value::Bool(v));
        self
    }
    pub fn on_close(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CLOSE, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Dialog<'ui, A>> for El<'ui, A> {
    fn from(b: Dialog<'ui, A>) -> Self {
        b.0
    }
}

pub struct Dialogbody<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn dialogbody(&'ui self) -> Dialogbody<'ui, A> {
        Dialogbody(self.el(sys::XTK_CTRL_DIALOGBODY, &raw const sys::flux_ctrl_dialogbody))
    }
}

impl<'ui, A: App> Dialogbody<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn items(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_ITEMS, Value::Enum(v as u32));
        self
    }
    pub fn justify(self, v: FlexJustify) -> Self {
        self.0.prop(sys::XTK_PROP_JUSTIFY_CONTENT, Value::Enum(v as u32));
        self
    }
    pub fn gap(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GAP, Value::F32(v));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn fill(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FILL, Value::Bool(v));
        self
    }
    pub fn fitw(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_WIDTH, Value::Bool(v));
        self
    }
    pub fn fith(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_HEIGHT, Value::Bool(v));
        self
    }
    pub fn wrap(self, v: FlexWrap) -> Self {
        self.0.prop(sys::XTK_PROP_WRAP, Value::Enum(v as u32));
        self
    }
    pub fn drop(self, v: DropOperation) -> Self {
        self.0.prop(sys::XTK_PROP_DROP_OPERATION, Value::Enum(v as u32));
        self
    }
    pub fn dragtext(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_TEXT, Value::Str(v.into()));
        self
    }
    pub fn dragfiles(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_FILES, Value::Str(v.into()));
        self
    }
    pub fn on_draghover(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_HOVER, Handler::Bool(f));
        self
    }
    pub fn on_dropfiles(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_FILES, Handler::Str(f));
        self
    }
    pub fn on_droptext(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_TEXT, Handler::Str(f));
        self
    }
    pub fn on_dragcompleted(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_COMPLETED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Dialogbody<'ui, A>> for El<'ui, A> {
    fn from(b: Dialogbody<'ui, A>) -> Self {
        b.0
    }
}

pub struct Menubar<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn menubar(&'ui self) -> Menubar<'ui, A> {
        Menubar(self.el(sys::XTK_CTRL_MENUBAR, &raw const sys::flux_ctrl_menubar))
    }
}

impl<'ui, A: App> Menubar<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
}

impl<'ui, A: App> From<Menubar<'ui, A>> for El<'ui, A> {
    fn from(b: Menubar<'ui, A>) -> Self {
        b.0
    }
}

pub struct Menuitem<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn menuitem(&'ui self) -> Menuitem<'ui, A> {
        Menuitem(self.el(sys::XTK_CTRL_MENUITEM, &raw const sys::flux_ctrl_menuitem))
    }
}

impl<'ui, A: App> Menuitem<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn selected(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED, Value::Bool(v));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Menuitem<'ui, A>> for El<'ui, A> {
    fn from(b: Menuitem<'ui, A>) -> Self {
        b.0
    }
}

pub struct Nav<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn nav(&'ui self) -> Nav<'ui, A> {
        Nav(self.el(sys::XTK_CTRL_NAV, &raw const sys::flux_ctrl_nav))
    }
}

impl<'ui, A: App> Nav<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Nav<'ui, A>> for El<'ui, A> {
    fn from(b: Nav<'ui, A>) -> Self {
        b.0
    }
}

pub struct Navitem<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn navitem(&'ui self) -> Navitem<'ui, A> {
        Navitem(self.el(sys::XTK_CTRL_NAVITEM, &raw const sys::flux_ctrl_navitem))
    }
}

impl<'ui, A: App> Navitem<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn selected(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED, Value::Bool(v));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Navitem<'ui, A>> for El<'ui, A> {
    fn from(b: Navitem<'ui, A>) -> Self {
        b.0
    }
}

pub struct Tabs<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn tabs(&'ui self) -> Tabs<'ui, A> {
        Tabs(self.el(sys::XTK_CTRL_TABS, &raw const sys::flux_ctrl_tabs))
    }
}

impl<'ui, A: App> Tabs<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_close(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CLOSE, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Tabs<'ui, A>> for El<'ui, A> {
    fn from(b: Tabs<'ui, A>) -> Self {
        b.0
    }
}

pub struct Tabitem<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn tabitem(&'ui self) -> Tabitem<'ui, A> {
        Tabitem(self.el(sys::XTK_CTRL_TABITEM, &raw const sys::flux_ctrl_tabitem))
    }
}

impl<'ui, A: App> Tabitem<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn selected(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED, Value::Bool(v));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Tabitem<'ui, A>> for El<'ui, A> {
    fn from(b: Tabitem<'ui, A>) -> Self {
        b.0
    }
}

pub struct Listitem<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn listitem(&'ui self) -> Listitem<'ui, A> {
        Listitem(self.el(sys::XTK_CTRL_LISTITEM, &raw const sys::flux_ctrl_listitem))
    }
}

impl<'ui, A: App> Listitem<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn selected(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED, Value::Bool(v));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Listitem<'ui, A>> for El<'ui, A> {
    fn from(b: Listitem<'ui, A>) -> Self {
        b.0
    }
}

pub struct Listbox<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn listbox(&'ui self) -> Listbox<'ui, A> {
        Listbox(self.el(sys::XTK_CTRL_LISTBOX, &raw const sys::flux_ctrl_listbox))
    }
}

impl<'ui, A: App> Listbox<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn itemh(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_HEIGHT, Value::F32(v));
        self
    }
    pub fn itemw(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_WIDTH, Value::F32(v));
        self
    }
    pub fn ncol(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_COLUMN_COUNT, Value::I32(v));
        self
    }
    pub fn mode(self, v: ListSelMode) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTION_MODE, Value::Enum(v as u32));
        self
    }
    pub fn first(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_REALIZED_FIRST, Value::I32(v));
        self
    }
    pub fn rectsel(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_RECTANGLE_SELECTION, Value::Bool(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Listbox<'ui, A>> for El<'ui, A> {
    fn from(b: Listbox<'ui, A>) -> Self {
        b.0
    }
}

pub struct Gridview<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn gridview(&'ui self) -> Gridview<'ui, A> {
        Gridview(self.el(sys::XTK_CTRL_GRIDVIEW, &raw const sys::flux_ctrl_gridview))
    }
}

impl<'ui, A: App> Gridview<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn itemh(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_HEIGHT, Value::F32(v));
        self
    }
    pub fn itemw(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_WIDTH, Value::F32(v));
        self
    }
    pub fn ncol(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_COLUMN_COUNT, Value::I32(v));
        self
    }
    pub fn mode(self, v: ListSelMode) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTION_MODE, Value::Enum(v as u32));
        self
    }
    pub fn first(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_REALIZED_FIRST, Value::I32(v));
        self
    }
    pub fn rectsel(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_RECTANGLE_SELECTION, Value::Bool(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Gridview<'ui, A>> for El<'ui, A> {
    fn from(b: Gridview<'ui, A>) -> Self {
        b.0
    }
}

pub struct Repeater<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn repeater(&'ui self) -> Repeater<'ui, A> {
        Repeater(self.el(sys::XTK_CTRL_REPEATER, &raw const sys::flux_ctrl_repeater))
    }
}

impl<'ui, A: App> Repeater<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn itemh(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_HEIGHT, Value::F32(v));
        self
    }
    pub fn itemw(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_WIDTH, Value::F32(v));
        self
    }
    pub fn ncol(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_COLUMN_COUNT, Value::I32(v));
        self
    }
    pub fn mode(self, v: ListSelMode) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTION_MODE, Value::Enum(v as u32));
        self
    }
    pub fn first(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_REALIZED_FIRST, Value::I32(v));
        self
    }
    pub fn rectsel(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_RECTANGLE_SELECTION, Value::Bool(v));
        self
    }
}

impl<'ui, A: App> From<Repeater<'ui, A>> for El<'ui, A> {
    fn from(b: Repeater<'ui, A>) -> Self {
        b.0
    }
}

pub struct Itemsview<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn itemsview(&'ui self) -> Itemsview<'ui, A> {
        Itemsview(self.el(sys::XTK_CTRL_ITEMSVIEW, &raw const sys::flux_ctrl_itemsview))
    }
}

impl<'ui, A: App> Itemsview<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn itemh(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_HEIGHT, Value::F32(v));
        self
    }
    pub fn itemw(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_WIDTH, Value::F32(v));
        self
    }
    pub fn ncol(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_COLUMN_COUNT, Value::I32(v));
        self
    }
    pub fn mode(self, v: ListSelMode) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTION_MODE, Value::Enum(v as u32));
        self
    }
    pub fn invoked(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_INVOKED_ENABLED, Value::Bool(v));
        self
    }
    pub fn first(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_REALIZED_FIRST, Value::I32(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Itemsview<'ui, A>> for El<'ui, A> {
    fn from(b: Itemsview<'ui, A>) -> Self {
        b.0
    }
}

pub struct Flip<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn flip(&'ui self) -> Flip<'ui, A> {
        Flip(self.el(sys::XTK_CTRL_FLIP, &raw const sys::flux_ctrl_flip))
    }
}

impl<'ui, A: App> Flip<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Flip<'ui, A>> for El<'ui, A> {
    fn from(b: Flip<'ui, A>) -> Self {
        b.0
    }
}

pub struct Pips<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn pips(&'ui self) -> Pips<'ui, A> {
        Pips(self.el(sys::XTK_CTRL_PIPS, &raw const sys::flux_ctrl_pips))
    }
}

impl<'ui, A: App> Pips<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn maxvisible(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_VISIBLE, Value::I32(v));
        self
    }
    pub fn nav(self, v: PipsNavVis) -> Self {
        self.0.prop(sys::XTK_PROP_NAV_VISIBILITY, Value::Enum(v as u32));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Pips<'ui, A>> for El<'ui, A> {
    fn from(b: Pips<'ui, A>) -> Self {
        b.0
    }
}

pub struct Suggest<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn suggest(&'ui self) -> Suggest<'ui, A> {
        Suggest(self.el(sys::XTK_CTRL_SUGGEST, &raw const sys::flux_ctrl_suggest))
    }
}

impl<'ui, A: App> Suggest<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn placeholder(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_PLACEHOLDER, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn on_change(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_STRING, Handler::Str(f));
        self
    }
    pub fn on_query(self, f: fn(String, i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_QUERY, Handler::Query(f));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Suggest<'ui, A>> for El<'ui, A> {
    fn from(b: Suggest<'ui, A>) -> Self {
        b.0
    }
}

pub struct Togglesplit<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn togglesplit(&'ui self) -> Togglesplit<'ui, A> {
        Togglesplit(self.el(sys::XTK_CTRL_TOGGLESPLIT, &raw const sys::flux_ctrl_togglesplit))
    }
}

impl<'ui, A: App> Togglesplit<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn checked(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_CHECKED, Value::Bool(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
    pub fn on_change(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_BOOL, Handler::Bool(f));
        self
    }
}

impl<'ui, A: App> From<Togglesplit<'ui, A>> for El<'ui, A> {
    fn from(b: Togglesplit<'ui, A>) -> Self {
        b.0
    }
}

pub struct Radiogroup<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn radiogroup(&'ui self) -> Radiogroup<'ui, A> {
        Radiogroup(self.el(sys::XTK_CTRL_RADIOGROUP, &raw const sys::flux_ctrl_radiogroup))
    }
}

impl<'ui, A: App> Radiogroup<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn ncol(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_COLUMN_COUNT, Value::I32(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Radiogroup<'ui, A>> for El<'ui, A> {
    fn from(b: Radiogroup<'ui, A>) -> Self {
        b.0
    }
}

pub struct Rating<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn rating(&'ui self) -> Rating<'ui, A> {
        Rating(self.el(sys::XTK_CTRL_RATING, &raw const sys::flux_ctrl_rating))
    }
}

impl<'ui, A: App> Rating<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn value(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_VALUE_F64, Value::F64(v));
        self
    }
    pub fn max(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_F64, Value::F64(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn placeholder(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_PLACEHOLDER_F64, Value::F64(v));
        self
    }
    pub fn initial(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_INITIAL_VALUE, Value::I32(v));
        self
    }
    pub fn clear(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_CLEAR_ENABLED, Value::Bool(v));
        self
    }
    pub fn readonly(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_READ_ONLY, Value::Bool(v));
        self
    }
    pub fn on_rating(self, f: fn(f64, i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_RATING_CHANGED, Handler::Rating(f));
        self
    }
}

impl<'ui, A: App> From<Rating<'ui, A>> for El<'ui, A> {
    fn from(b: Rating<'ui, A>) -> Self {
        b.0
    }
}

pub struct Selector<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn selector(&'ui self) -> Selector<'ui, A> {
        Selector(self.el(sys::XTK_CTRL_SELECTOR, &raw const sys::flux_ctrl_selector))
    }
}

impl<'ui, A: App> Selector<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Selector<'ui, A>> for El<'ui, A> {
    fn from(b: Selector<'ui, A>) -> Self {
        b.0
    }
}

pub struct Selectoritem<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn selectoritem(&'ui self) -> Selectoritem<'ui, A> {
        Selectoritem(self.el(sys::XTK_CTRL_SELECTORITEM, &raw const sys::flux_ctrl_selectoritem))
    }
}

impl<'ui, A: App> Selectoritem<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn selected(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED, Value::Bool(v));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Selectoritem<'ui, A>> for El<'ui, A> {
    fn from(b: Selectoritem<'ui, A>) -> Self {
        b.0
    }
}

pub struct Breadcrumb<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn breadcrumb(&'ui self) -> Breadcrumb<'ui, A> {
        Breadcrumb(self.el(sys::XTK_CTRL_BREADCRUMB, &raw const sys::flux_ctrl_breadcrumb))
    }
}

impl<'ui, A: App> Breadcrumb<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Breadcrumb<'ui, A>> for El<'ui, A> {
    fn from(b: Breadcrumb<'ui, A>) -> Self {
        b.0
    }
}

pub struct Crumb<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn crumb(&'ui self) -> Crumb<'ui, A> {
        Crumb(self.el(sys::XTK_CTRL_CRUMB, &raw const sys::flux_ctrl_crumb))
    }
}

impl<'ui, A: App> Crumb<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn selected(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED, Value::Bool(v));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Crumb<'ui, A>> for El<'ui, A> {
    fn from(b: Crumb<'ui, A>) -> Self {
        b.0
    }
}

pub struct Teachtip<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn teachtip(&'ui self) -> Teachtip<'ui, A> {
        Teachtip(self.el(sys::XTK_CTRL_TEACHTIP, &raw const sys::flux_ctrl_teachtip))
    }
}

impl<'ui, A: App> Teachtip<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn subtitle(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_SUBTITLE, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn action(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ACTION_TEXT, Value::Str(v.into()));
        self
    }
    pub fn actionaccent(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ACTION_ACCENT, Value::Bool(v));
        self
    }
    pub fn close(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_CLOSE_TEXT, Value::Str(v.into()));
        self
    }
    pub fn placement(self, v: TeachingTipPlacement) -> Self {
        self.0.prop(sys::XTK_PROP_PLACEMENT, Value::Enum(v as u32));
        self
    }
    pub fn untargeted(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_UNTARGETED, Value::Bool(v));
        self
    }
    pub fn lightdismiss(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_LIGHT_DISMISS, Value::Bool(v));
        self
    }
    pub fn tail(self, v: TeachingTipTail) -> Self {
        self.0.prop(sys::XTK_PROP_TAIL_VISIBILITY, Value::Enum(v as u32));
        self
    }
    pub fn placementmargin(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PLACEMENT_MARGIN, Value::F32(v));
        self
    }
    pub fn open(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_OPEN, Value::Bool(v));
        self
    }
    pub fn on_close(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CLOSE, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Teachtip<'ui, A>> for El<'ui, A> {
    fn from(b: Teachtip<'ui, A>) -> Self {
        b.0
    }
}

pub struct Tree<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn tree(&'ui self) -> Tree<'ui, A> {
        Tree(self.el(sys::XTK_CTRL_TREE, &raw const sys::flux_ctrl_tree))
    }
}

impl<'ui, A: App> Tree<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn itemh(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_HEIGHT, Value::F32(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
    pub fn on_expand(self, f: fn(i32, bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_TREE_EXPAND, Handler::IndexBool(f));
        self
    }
    pub fn on_select(self, f: fn(i32, bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_TREE_SELECT, Handler::IndexBool(f));
        self
    }
}

impl<'ui, A: App> From<Tree<'ui, A>> for El<'ui, A> {
    fn from(b: Tree<'ui, A>) -> Self {
        b.0
    }
}

pub struct Treeitem<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn treeitem(&'ui self) -> Treeitem<'ui, A> {
        Treeitem(self.el(sys::XTK_CTRL_TREEITEM, &raw const sys::flux_ctrl_treeitem))
    }
}

impl<'ui, A: App> Treeitem<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn selected(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED, Value::Bool(v));
        self
    }
    pub fn expanded(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_EXPANDED, Value::Bool(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Treeitem<'ui, A>> for El<'ui, A> {
    fn from(b: Treeitem<'ui, A>) -> Self {
        b.0
    }
}

pub struct Refresh<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn refresh(&'ui self) -> Refresh<'ui, A> {
        Refresh(self.el(sys::XTK_CTRL_REFRESH, &raw const sys::flux_ctrl_refresh))
    }
}

impl<'ui, A: App> Refresh<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn direction(self, v: PullDirection) -> Self {
        self.0.prop(sys::XTK_PROP_PULL_DIRECTION, Value::Enum(v as u32));
        self
    }
    pub fn visualizer(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_VISUALIZER_SIZE, Value::F32(v));
        self
    }
    pub fn threshold(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_THRESHOLD_RATIO, Value::F32(v));
        self
    }
    pub fn refreshing(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_REFRESHING, Value::Bool(v));
        self
    }
    pub fn on_refresh(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_REFRESH, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Refresh<'ui, A>> for El<'ui, A> {
    fn from(b: Refresh<'ui, A>) -> Self {
        b.0
    }
}

pub struct Avatar<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn avatar(&'ui self) -> Avatar<'ui, A> {
        Avatar(self.el(sys::XTK_CTRL_AVATAR, &raw const sys::flux_ctrl_avatar))
    }
}

impl<'ui, A: App> Avatar<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn initials(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_INITIALS, Value::Str(v.into()));
        self
    }
    pub fn src(self, v: u64) -> Self {
        self.0.prop(sys::XTK_PROP_IMAGE_SOURCE, Value::Resource(v));
        self
    }
    pub fn badgeglyph(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_BADGE_GLYPH, Value::Str(v.into()));
        self
    }
    pub fn badgevalue(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_BADGE_VALUE, Value::I32(v));
        self
    }
    pub fn group(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_IS_GROUP, Value::Bool(v));
        self
    }
}

impl<'ui, A: App> From<Avatar<'ui, A>> for El<'ui, A> {
    fn from(b: Avatar<'ui, A>) -> Self {
        b.0
    }
}

pub struct Pager<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn pager(&'ui self) -> Pager<'ui, A> {
        Pager(self.el(sys::XTK_CTRL_PAGER, &raw const sys::flux_ctrl_pager))
    }
}

impl<'ui, A: App> Pager<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn display(self, v: PagerDisplayMode) -> Self {
        self.0.prop(sys::XTK_PROP_DISPLAY_MODE, Value::Enum(v as u32));
        self
    }
    pub fn first(self, v: PagerButtonVis) -> Self {
        self.0.prop(sys::XTK_PROP_FIRST_BUTTON, Value::Enum(v as u32));
        self
    }
    pub fn prev(self, v: PagerButtonVis) -> Self {
        self.0.prop(sys::XTK_PROP_PREVIOUS_BUTTON, Value::Enum(v as u32));
        self
    }
    pub fn next(self, v: PagerButtonVis) -> Self {
        self.0.prop(sys::XTK_PROP_NEXT_BUTTON, Value::Enum(v as u32));
        self
    }
    pub fn last(self, v: PagerButtonVis) -> Self {
        self.0.prop(sys::XTK_PROP_LAST_BUTTON, Value::Enum(v as u32));
        self
    }
    pub fn prefix(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_PREFIX, Value::Str(v.into()));
        self
    }
    pub fn suffix(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_SUFFIX, Value::Str(v.into()));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Pager<'ui, A>> for El<'ui, A> {
    fn from(b: Pager<'ui, A>) -> Self {
        b.0
    }
}

pub struct Splitview<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn splitview(&'ui self) -> Splitview<'ui, A> {
        Splitview(self.el(sys::XTK_CTRL_SPLITVIEW, &raw const sys::flux_ctrl_splitview))
    }
}

impl<'ui, A: App> Splitview<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn gap(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GAP, Value::F32(v));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn fill(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FILL, Value::Bool(v));
        self
    }
    pub fn open(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_OPEN, Value::Bool(v));
        self
    }
    pub fn display(self, v: SplitViewDisplayMode) -> Self {
        self.0.prop(sys::XTK_PROP_DISPLAY_MODE, Value::Enum(v as u32));
        self
    }
    pub fn placement(self, v: SplitViewPanePlacement) -> Self {
        self.0.prop(sys::XTK_PROP_PANE_PLACEMENT, Value::Enum(v as u32));
        self
    }
    pub fn openlen(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_OPEN_PANE_LENGTH, Value::F32(v));
        self
    }
    pub fn compactlen(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_COMPACT_PANE_LENGTH, Value::F32(v));
        self
    }
}

impl<'ui, A: App> From<Splitview<'ui, A>> for El<'ui, A> {
    fn from(b: Splitview<'ui, A>) -> Self {
        b.0
    }
}

pub struct Splitpane<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn splitpane(&'ui self) -> Splitpane<'ui, A> {
        Splitpane(self.el(sys::XTK_CTRL_SPLITPANE, &raw const sys::flux_ctrl_splitpane))
    }
}

impl<'ui, A: App> Splitpane<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn items(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_ITEMS, Value::Enum(v as u32));
        self
    }
    pub fn justify(self, v: FlexJustify) -> Self {
        self.0.prop(sys::XTK_PROP_JUSTIFY_CONTENT, Value::Enum(v as u32));
        self
    }
    pub fn gap(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GAP, Value::F32(v));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn fill(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FILL, Value::Bool(v));
        self
    }
    pub fn fitw(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_WIDTH, Value::Bool(v));
        self
    }
    pub fn fith(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_HEIGHT, Value::Bool(v));
        self
    }
    pub fn wrap(self, v: FlexWrap) -> Self {
        self.0.prop(sys::XTK_PROP_WRAP, Value::Enum(v as u32));
        self
    }
    pub fn drop(self, v: DropOperation) -> Self {
        self.0.prop(sys::XTK_PROP_DROP_OPERATION, Value::Enum(v as u32));
        self
    }
    pub fn dragtext(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_TEXT, Value::Str(v.into()));
        self
    }
    pub fn dragfiles(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_FILES, Value::Str(v.into()));
        self
    }
    pub fn on_draghover(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_HOVER, Handler::Bool(f));
        self
    }
    pub fn on_dropfiles(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_FILES, Handler::Str(f));
        self
    }
    pub fn on_droptext(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_TEXT, Handler::Str(f));
        self
    }
    pub fn on_dragcompleted(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_COMPLETED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Splitpane<'ui, A>> for El<'ui, A> {
    fn from(b: Splitpane<'ui, A>) -> Self {
        b.0
    }
}

pub struct Splitbody<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn splitbody(&'ui self) -> Splitbody<'ui, A> {
        Splitbody(self.el(sys::XTK_CTRL_SPLITBODY, &raw const sys::flux_ctrl_splitbody))
    }
}

impl<'ui, A: App> Splitbody<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn items(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_ITEMS, Value::Enum(v as u32));
        self
    }
    pub fn justify(self, v: FlexJustify) -> Self {
        self.0.prop(sys::XTK_PROP_JUSTIFY_CONTENT, Value::Enum(v as u32));
        self
    }
    pub fn gap(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GAP, Value::F32(v));
        self
    }
    pub fn pl(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_LEFT, Value::F32(v));
        self
    }
    pub fn pt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_TOP, Value::F32(v));
        self
    }
    pub fn pr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_RIGHT, Value::F32(v));
        self
    }
    pub fn pb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_PADDING_BOTTOM, Value::F32(v));
        self
    }
    pub fn fill(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FILL, Value::Bool(v));
        self
    }
    pub fn fitw(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_WIDTH, Value::Bool(v));
        self
    }
    pub fn fith(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_FIT_HEIGHT, Value::Bool(v));
        self
    }
    pub fn wrap(self, v: FlexWrap) -> Self {
        self.0.prop(sys::XTK_PROP_WRAP, Value::Enum(v as u32));
        self
    }
    pub fn drop(self, v: DropOperation) -> Self {
        self.0.prop(sys::XTK_PROP_DROP_OPERATION, Value::Enum(v as u32));
        self
    }
    pub fn dragtext(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_TEXT, Value::Str(v.into()));
        self
    }
    pub fn dragfiles(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_DRAG_FILES, Value::Str(v.into()));
        self
    }
    pub fn on_draghover(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_HOVER, Handler::Bool(f));
        self
    }
    pub fn on_dropfiles(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_FILES, Handler::Str(f));
        self
    }
    pub fn on_droptext(self, f: fn(String) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DROP_TEXT, Handler::Str(f));
        self
    }
    pub fn on_dragcompleted(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DRAG_COMPLETED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Splitbody<'ui, A>> for El<'ui, A> {
    fn from(b: Splitbody<'ui, A>) -> Self {
        b.0
    }
}

pub struct Titlebar<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn titlebar(&'ui self) -> Titlebar<'ui, A> {
        Titlebar(self.el(sys::XTK_CTRL_TITLEBAR, &raw const sys::flux_ctrl_titlebar))
    }
}

impl<'ui, A: App> Titlebar<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn subtitle(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_SUBTITLE, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn showback(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SHOW_BACK, Value::Bool(v));
        self
    }
    pub fn backdisabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_BACK_DISABLED, Value::Bool(v));
        self
    }
    pub fn showtoggle(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SHOW_PANE_TOGGLE, Value::Bool(v));
        self
    }
    pub fn integrate(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_INTEGRATE_WINDOW, Value::Bool(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
    pub fn on_toggle(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_TOGGLE, Handler::Bool(f));
        self
    }
}

impl<'ui, A: App> From<Titlebar<'ui, A>> for El<'ui, A> {
    fn from(b: Titlebar<'ui, A>) -> Self {
        b.0
    }
}

pub struct Scrollbar<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn scrollbar(&'ui self) -> Scrollbar<'ui, A> {
        Scrollbar(self.el(sys::XTK_CTRL_SCROLLBAR, &raw const sys::flux_ctrl_scrollbar))
    }
}

impl<'ui, A: App> Scrollbar<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn dir(self, v: StackDirection) -> Self {
        self.0.prop(sys::XTK_PROP_ORIENTATION, Value::Enum(v as u32));
        self
    }
    pub fn min(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_MIN_F64, Value::F64(v));
        self
    }
    pub fn max(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_F64, Value::F64(v));
        self
    }
    pub fn value(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_VALUE_F64, Value::F64(v));
        self
    }
    pub fn viewport(self, v: f64) -> Self {
        self.0.prop(sys::XTK_PROP_VIEWPORT_SIZE, Value::F64(v));
        self
    }
    pub fn smallchange(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SMALL_CHANGE, Value::F32(v));
        self
    }
    pub fn largechange(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_LARGE_CHANGE, Value::F32(v));
        self
    }
    pub fn indicator(self, v: ScrollBarIndicator) -> Self {
        self.0.prop(sys::XTK_PROP_INDICATOR_MODE, Value::Enum(v as u32));
        self
    }
    pub fn on_change(self, f: fn(f64) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_F64, Handler::F64(f));
        self
    }
}

impl<'ui, A: App> From<Scrollbar<'ui, A>> for El<'ui, A> {
    fn from(b: Scrollbar<'ui, A>) -> Self {
        b.0
    }
}

pub struct Looping<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn looping(&'ui self) -> Looping<'ui, A> {
        Looping(self.el(sys::XTK_CTRL_LOOPING, &raw const sys::flux_ctrl_looping))
    }
}

impl<'ui, A: App> Looping<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn nitem(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_COUNT, Value::I32(v));
        self
    }
    pub fn sel(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTED_INDEX, Value::I32(v));
        self
    }
    pub fn itemh(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_HEIGHT, Value::F32(v));
        self
    }
    pub fn itemw(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_ITEM_WIDTH, Value::F32(v));
        self
    }
    pub fn loop_(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SHOULD_LOOP, Value::Bool(v));
        self
    }
    pub fn on_select(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_SELECT, Handler::I32(f));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Looping<'ui, A>> for El<'ui, A> {
    fn from(b: Looping<'ui, A>) -> Self {
        b.0
    }
}

pub struct Datepicker<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn datepicker(&'ui self) -> Datepicker<'ui, A> {
        Datepicker(self.el(sys::XTK_CTRL_DATEPICKER, &raw const sys::flux_ctrl_datepicker))
    }
}

impl<'ui, A: App> Datepicker<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn header(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_HEADER, Value::Str(v.into()));
        self
    }
    pub fn date(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_DATE, Value::I32(v));
        self
    }
    pub fn minyear(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_MIN_YEAR, Value::I32(v));
        self
    }
    pub fn maxyear(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_YEAR, Value::I32(v));
        self
    }
    pub fn on_value(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_VALUE_CHANGED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Datepicker<'ui, A>> for El<'ui, A> {
    fn from(b: Datepicker<'ui, A>) -> Self {
        b.0
    }
}

pub struct Timepicker<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn timepicker(&'ui self) -> Timepicker<'ui, A> {
        Timepicker(self.el(sys::XTK_CTRL_TIMEPICKER, &raw const sys::flux_ctrl_timepicker))
    }
}

impl<'ui, A: App> Timepicker<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn header(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_HEADER, Value::Str(v.into()));
        self
    }
    pub fn time(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_TIME, Value::I32(v));
        self
    }
    pub fn clock(self, v: Clock) -> Self {
        self.0.prop(sys::XTK_PROP_CLOCK, Value::Enum(v as u32));
        self
    }
    pub fn step(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_MINUTE_INCREMENT, Value::I32(v));
        self
    }
    pub fn on_value(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_VALUE_CHANGED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Timepicker<'ui, A>> for El<'ui, A> {
    fn from(b: Timepicker<'ui, A>) -> Self {
        b.0
    }
}

pub struct Calendar<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn calendar(&'ui self) -> Calendar<'ui, A> {
        Calendar(self.el(sys::XTK_CTRL_CALENDAR, &raw const sys::flux_ctrl_calendar))
    }
}

impl<'ui, A: App> Calendar<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn date(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_DATE, Value::I32(v));
        self
    }
    pub fn mode(self, v: CalendarMode) -> Self {
        self.0.prop(sys::XTK_PROP_SELECTION_MODE, Value::Enum(v as u32));
        self
    }
    pub fn mindate(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_MIN_DATE, Value::I32(v));
        self
    }
    pub fn maxdate(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_DATE, Value::I32(v));
        self
    }
    pub fn firstday(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_FIRST_DAY, Value::I32(v));
        self
    }
    pub fn on_date(self, f: fn(i32, bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_DATE_SELECT, Handler::IndexBool(f));
        self
    }
}

impl<'ui, A: App> From<Calendar<'ui, A>> for El<'ui, A> {
    fn from(b: Calendar<'ui, A>) -> Self {
        b.0
    }
}

pub struct Colorpicker<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn colorpicker(&'ui self) -> Colorpicker<'ui, A> {
        Colorpicker(self.el(sys::XTK_CTRL_COLORPICKER, &raw const sys::flux_ctrl_colorpicker))
    }
}

impl<'ui, A: App> Colorpicker<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn color(self, v: u32) -> Self {
        self.0.prop(sys::XTK_PROP_COLOR, Value::Color(v));
        self
    }
    pub fn alpha(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ALPHA_ENABLED, Value::Bool(v));
        self
    }
    pub fn morebutton(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_MORE_BUTTON_VISIBLE, Value::Bool(v));
        self
    }
    pub fn spectrum(self, v: Spectrum) -> Self {
        self.0.prop(sys::XTK_PROP_SPECTRUM, Value::Enum(v as u32));
        self
    }
    pub fn on_value(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_VALUE_CHANGED, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Colorpicker<'ui, A>> for El<'ui, A> {
    fn from(b: Colorpicker<'ui, A>) -> Self {
        b.0
    }
}

pub struct Colheader<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn colheader(&'ui self) -> Colheader<'ui, A> {
        Colheader(self.el(sys::XTK_CTRL_COLHEADER, &raw const sys::flux_ctrl_colheader))
    }
}

impl<'ui, A: App> Colheader<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
}

impl<'ui, A: App> From<Colheader<'ui, A>> for El<'ui, A> {
    fn from(b: Colheader<'ui, A>) -> Self {
        b.0
    }
}

pub struct Colitem<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn colitem(&'ui self) -> Colitem<'ui, A> {
        Colitem(self.el(sys::XTK_CTRL_COLITEM, &raw const sys::flux_ctrl_colitem))
    }
}

impl<'ui, A: App> Colitem<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn minw(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MIN_WIDTH, Value::F32(v));
        self
    }
    pub fn maxw(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MAX_WIDTH, Value::F32(v));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn align(self, v: TextAlign) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT_ALIGN, Value::Enum(v as u32));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn resizable(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_RESIZABLE, Value::Bool(v));
        self
    }
    pub fn sort(self, v: SortDirection) -> Self {
        self.0.prop(sys::XTK_PROP_SORT_DIRECTION, Value::Enum(v as u32));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
    pub fn on_resize(self, f: fn(f32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_RESIZE, Handler::F32(f));
        self
    }
    pub fn on_fit(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_FIT, Handler::Value(msg));
        self
    }
    pub fn on_reorder(self, f: fn(i32) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_REORDER, Handler::I32(f));
        self
    }
}

impl<'ui, A: App> From<Colitem<'ui, A>> for El<'ui, A> {
    fn from(b: Colitem<'ui, A>) -> Self {
        b.0
    }
}

pub struct Commandbar<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn commandbar(&'ui self) -> Commandbar<'ui, A> {
        Commandbar(self.el(sys::XTK_CTRL_COMMANDBAR, &raw const sys::flux_ctrl_commandbar))
    }
}

impl<'ui, A: App> Commandbar<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn open(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_OPEN, Value::Bool(v));
        self
    }
    pub fn sticky(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_IS_STICKY, Value::Bool(v));
        self
    }
    pub fn mode(self, v: AppBarMode) -> Self {
        self.0.prop(sys::XTK_PROP_CLOSED_DISPLAY_MODE, Value::Enum(v as u32));
        self
    }
    pub fn labels(self, v: LabelPosition) -> Self {
        self.0.prop(sys::XTK_PROP_DEFAULT_LABEL_POSITION, Value::Enum(v as u32));
        self
    }
    pub fn more(self, v: OverflowButton) -> Self {
        self.0.prop(sys::XTK_PROP_OVERFLOW_BUTTON_VISIBILITY, Value::Enum(v as u32));
        self
    }
    pub fn dynamic(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_DYNAMIC_OVERFLOW, Value::Bool(v));
        self
    }
    pub fn on_open(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_OPEN, Handler::Bool(f));
        self
    }
}

impl<'ui, A: App> From<Commandbar<'ui, A>> for El<'ui, A> {
    fn from(b: Commandbar<'ui, A>) -> Self {
        b.0
    }
}

pub struct Appbarbutton<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn appbarbutton(&'ui self) -> Appbarbutton<'ui, A> {
        Appbarbutton(self.el(sys::XTK_CTRL_APPBARBUTTON, &raw const sys::flux_ctrl_appbarbutton))
    }
}

impl<'ui, A: App> Appbarbutton<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn labelpos(self, v: ElementLabel) -> Self {
        self.0.prop(sys::XTK_PROP_LABEL_POSITION, Value::Enum(v as u32));
        self
    }
    pub fn accel(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ACCELERATOR_TEXT, Value::Str(v.into()));
        self
    }
    pub fn secondary(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SECONDARY, Value::Bool(v));
        self
    }
    pub fn order(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_DYNAMIC_OVERFLOW_ORDER, Value::I32(v));
        self
    }
    pub fn on_invoke(self, msg: A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_INVOKE, Handler::Value(msg));
        self
    }
}

impl<'ui, A: App> From<Appbarbutton<'ui, A>> for El<'ui, A> {
    fn from(b: Appbarbutton<'ui, A>) -> Self {
        b.0
    }
}

pub struct Appbartoggle<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn appbartoggle(&'ui self) -> Appbartoggle<'ui, A> {
        Appbartoggle(self.el(sys::XTK_CTRL_APPBARTOGGLE, &raw const sys::flux_ctrl_appbartoggle))
    }
}

impl<'ui, A: App> Appbartoggle<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn text(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TEXT, Value::Str(v.into()));
        self
    }
    pub fn icon(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ICON, Value::Str(v.into()));
        self
    }
    pub fn enabled(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_ENABLED, Value::Bool(v));
        self
    }
    pub fn labelpos(self, v: ElementLabel) -> Self {
        self.0.prop(sys::XTK_PROP_LABEL_POSITION, Value::Enum(v as u32));
        self
    }
    pub fn accel(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_ACCELERATOR_TEXT, Value::Str(v.into()));
        self
    }
    pub fn checked(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_CHECKED, Value::Bool(v));
        self
    }
    pub fn secondary(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SECONDARY, Value::Bool(v));
        self
    }
    pub fn order(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_DYNAMIC_OVERFLOW_ORDER, Value::I32(v));
        self
    }
    pub fn on_change(self, f: fn(bool) -> A::Msg) -> Self {
        self.0.on(sys::XTK_EVENT_CHANGE_BOOL, Handler::Bool(f));
        self
    }
}

impl<'ui, A: App> From<Appbartoggle<'ui, A>> for El<'ui, A> {
    fn from(b: Appbartoggle<'ui, A>) -> Self {
        b.0
    }
}

pub struct Appbarsep<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn appbarsep(&'ui self) -> Appbarsep<'ui, A> {
        Appbarsep(self.el(sys::XTK_CTRL_APPBARSEP, &raw const sys::flux_ctrl_appbarsep))
    }
}

impl<'ui, A: App> Appbarsep<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
    pub fn secondary(self, v: bool) -> Self {
        self.0.prop(sys::XTK_PROP_SECONDARY, Value::Bool(v));
        self
    }
    pub fn order(self, v: i32) -> Self {
        self.0.prop(sys::XTK_PROP_DYNAMIC_OVERFLOW_ORDER, Value::I32(v));
        self
    }
}

impl<'ui, A: App> From<Appbarsep<'ui, A>> for El<'ui, A> {
    fn from(b: Appbarsep<'ui, A>) -> Self {
        b.0
    }
}

pub struct Groupheader<'ui, A: App>(pub(crate) El<'ui, A>);

impl<'ui, A: App> Ui<'ui, A> {
    pub fn groupheader(&'ui self) -> Groupheader<'ui, A> {
        Groupheader(self.el(sys::XTK_CTRL_GROUPHEADER, &raw const sys::flux_ctrl_groupheader))
    }
}

impl<'ui, A: App> Groupheader<'ui, A> {
    pub fn key(self, key: u64) -> Self {
        self.0.key(key);
        self
    }
    pub fn child(self, child: impl Into<El<'ui, A>>) -> Self {
        self.0.child(child.into());
        self
    }
    pub fn children<I: IntoIterator<Item = El<'ui, A>>>(self, kids: I) -> Self {
        for kid in kids {
            self.0.child(kid);
        }
        self
    }
    pub fn w(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_WIDTH, Value::F32(v));
        self
    }
    pub fn h(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_HEIGHT, Value::F32(v));
        self
    }
    pub fn grow(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_GROW, Value::F32(v));
        self
    }
    pub fn shrink(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_SHRINK, Value::F32(v));
        self
    }
    pub fn self_(self, v: FlexAlign) -> Self {
        self.0.prop(sys::XTK_PROP_ALIGN_SELF, Value::Enum(v as u32));
        self
    }
    pub fn ml(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_LEFT, Value::F32(v));
        self
    }
    pub fn mt(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_TOP, Value::F32(v));
        self
    }
    pub fn mr(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_RIGHT, Value::F32(v));
        self
    }
    pub fn mb(self, v: f32) -> Self {
        self.0.prop(sys::XTK_PROP_MARGIN_BOTTOM, Value::F32(v));
        self
    }
    pub fn tip(self, v: &str) -> Self {
        self.0.prop(sys::XTK_PROP_TOOLTIP, Value::Str(v.into()));
        self
    }
}

impl<'ui, A: App> From<Groupheader<'ui, A>> for El<'ui, A> {
    fn from(b: Groupheader<'ui, A>) -> Self {
        b.0
    }
}
