// Copyright 2021 The AccessKit Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Derived from Chromium's accessibility abstraction.
// Copyright 2018 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE.chromium file.

use std::ops::Range;

/// The type of an accessibility node.
///
/// The majority of these roles come from the ARIA specification. Reference
/// the latest draft for proper usage.
///
/// Like the AccessKit schema as a whole, this list is largely taken
/// from Chromium. However, unlike Chromium's alphabetized list, this list
/// is ordered roughly by expected usage frequency (with the notable exception
/// of [`Role::Unknown`]). This is more efficient in serialization formats
/// where integers use a variable-length encoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Unknown,
    InlineTextBox,
    Cell,
    StaticText,
    Image,
    Link,
    Row,
    ListItem,

    /// Contains the bullet, number, or other marker for a list item.
    ListMarker,

    TreeItem,
    ListBoxOption,
    MenuItem,
    MenuListOption,
    Paragraph,
    Group,

    /// Used for ARIA role="none"/"presentation" -- ignored in platform tree.
    Presentation,

    CheckBox,
    RadioButton,
    TextField,
    Button,
    LabelText,
    Pane,
    RowHeader,
    ColumnHeader,
    Column,
    RowGroup,
    List,
    Table,
    TableHeaderContainer,
    LayoutTableCell,
    LayoutTableRow,
    LayoutTable,
    Switch,
    ToggleButton,
    Menu,

    Abbr,
    Alert,
    AlertDialog,
    Application,
    Article,
    Audio,
    Banner,
    Blockquote,
    Canvas,
    Caption,
    Caret,
    Client,
    Code,
    ColorWell,
    ComboBoxGrouping,
    ComboBoxMenuButton,
    Complementary,
    Comment,
    ContentDeletion,
    ContentInsertion,
    ContentInfo,
    Date,
    DateTime,
    Definition,
    DescriptionList,
    DescriptionListDetail,
    DescriptionListTerm,
    Details,
    Dialog,
    Directory,
    DisclosureTriangle,
    Document,
    EmbeddedObject,
    Emphasis,
    Feed,
    FigureCaption,
    Figure,
    Footer,
    FooterAsNonLandmark,
    Form,
    GenericContainer,
    Grid,
    Header,
    HeaderAsNonLandmark,
    Heading,
    Iframe,
    IframePresentational,
    ImeCandidate,
    InputTime,
    Keyboard,
    Legend,
    LineBreak,
    ListBox,
    Log,
    Main,
    Mark,
    Marquee,
    Math,
    MenuBar,
    MenuItemCheckBox,
    MenuItemRadio,
    MenuListPopup,
    Meter,
    Navigation,
    Note,
    PluginObject,
    PopupButton,
    Portal,
    Pre,
    ProgressIndicator,
    RadioGroup,
    Region,
    RootWebArea,
    Ruby,
    RubyAnnotation,
    ScrollBar,
    ScrollView,
    Search,
    SearchBox,
    Section,
    Slider,
    SpinButton,
    Splitter,
    Status,
    Strong,
    Suggestion,
    SvgRoot,
    Tab,
    TabList,
    TabPanel,
    Term,
    TextFieldWithComboBox,
    Time,
    Timer,
    TitleBar,
    Toolbar,
    Tooltip,
    Tree,
    TreeGrid,
    Video,
    WebView,
    Window,

    PdfActionableHighlight,
    PdfRoot,

    // --------------------------------------------------------------
    // ARIA Graphics module roles:
    // https://rawgit.com/w3c/graphics-aam/master/#mapping_role_table
    GraphicsDocument,
    GraphicsObject,
    GraphicsSymbol,
    // End ARIA Graphics module roles.
    // --------------------------------------------------------------

    // --------------------------------------------------------------
    // DPub Roles:
    // https://www.w3.org/TR/dpub-aam-1.0/#mapping_role_table
    DocAbstract,
    DocAcknowledgements,
    DocAfterword,
    DocAppendix,
    DocBackLink,
    DocBiblioEntry,
    DocBibliography,
    DocBiblioRef,
    DocChapter,
    DocColophon,
    DocConclusion,
    DocCover,
    DocCredit,
    DocCredits,
    DocDedication,
    DocEndnote,
    DocEndnotes,
    DocEpigraph,
    DocEpilogue,
    DocErrata,
    DocExample,
    DocFootnote,
    DocForeword,
    DocGlossary,
    DocGlossRef,
    DocIndex,
    DocIntroduction,
    DocNoteRef,
    DocNotice,
    DocPageBreak,
    DocPageFooter,
    DocPageHeader,
    DocPageList,
    DocPart,
    DocPreface,
    DocPrologue,
    DocPullquote,
    DocQna,
    DocSubtitle,
    DocTip,
    DocToc,
    // End DPub roles.
    // --------------------------------------------------------------

    /// Behaves similar to an ARIA grid but is primarily used by Chromium's
    /// `TableView` and its subclasses, so they can be exposed correctly
    /// on certain platforms.
    ListGrid,
}

/// An action to be taken on an accessibility node.
/// In contrast to [`DefaultActionVerb`], these describe what happens to the
/// object, e.g. "focus".
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    /// Do the default action for an object, typically this means "click".
    Default,

    Focus,
    Blur,

    Collapse,
    Expand,

    CustomAction,

    /// Decrement a slider or range control by one step value.
    Decrement,
    /// Increment a slider or range control by one step value.
    Increment,

    /// Get the bounding rect for a range of text.
    GetTextLocation,

    HideTooltip,
    ShowTooltip,

    /// Request that the tree source invalidate its entire tree.
    InvalidateTree,

    /// Load inline text boxes for this subtree, providing information
    /// about word boundaries, line layout, and individual character
    /// bounding boxes.
    LoadInlineTextBoxes,

    /// Delete any selected text in the control's text value and
    /// insert |ActionRequest.value| in its place, like when typing or pasting.
    ReplaceSelectedText,

    // Scrolls by approximately one screen in a specific direction. Should be
    // called on a node that has scrollable boolean set to true.
    // TBD: Do we need a doc comment on each of the values below?
    // Or does this awkwardness suggest a refactor?
    ScrollBackward,
    ScrollDown,
    ScrollForward,
    ScrollLeft,
    ScrollRight,
    ScrollUp,

    /// Scroll any scrollable containers to make the target object visible
    /// on the screen.  Optionally pass a subfocus rect in
    /// ActionRequest.target_rect, in node-local coordinates.
    ScrollIntoView,

    /// Scroll the given object to a specified point on the screen in
    /// global screen coordinates. Pass a point in ActionRequest.target_point.
    ScrollToPoint,

    SetScrollOffset,
    SetSelection,

    /// Don't focus this node, but set it as the sequential focus navigation
    /// starting point, so that pressing Tab moves to the next element
    /// following this one, for example.
    SetSequentialFocusNavigationStartingPoint,

    /// Replace the value of the control with ActionRequest.value and
    /// reset the selection, if applicable.
    SetValue,

    ShowContextMenu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NameFrom {
    /// E.g. `aria-label`.
    Attribute,
    AttributeExplicitlyEmpty,
    /// E.g. in the case of a table, from a `caption` element.
    Caption,
    Contents,
    /// E.g. from an HTML placeholder attribute on a text field.
    Placeholder,
    /// E.g. from a `figcaption` element in a figure.
    RelatedElement,
    /// E.g. `<input type="text" title="title">`.
    Title,
    /// E.g. `<input type="button" value="Button's name">`.
    Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DescriptionFrom {
    AriaDescription,
    /// HTML-AAM 5.2.2
    ButtonLabel,
    RelatedElement,
    RubyAnnotation,
    /// HTML-AAM 5.8.2
    Summary,
    /// HTML-AAM 5.9.2
    TableCaption,
    Title,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MarkerType {
    SpellingError,
    GrammarError,
    SearchMatch,
    ActiveSuggestion,
    Suggestion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

/// Indicates if a form control has invalid input or
/// if a web DOM element has an aria-invalid attribute.
#[derive(Clone, Debug, PartialEq)]
pub enum InvalidState {
    False,
    True,
    Other(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CheckedState {
    False,
    True,
    Mixed,
}

/// Describes the action that will be performed on a given node when
/// executing the default action, which is a click.
/// In contrast to [`Action`], these describe what the user can do on the
/// object, e.g. "press", not what happens to the object as a result.
/// Only one verb can be used at a time to describe the default action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DefaultActionVerb {
    Activate,
    Check,
    Uncheck,
    Click,
    /// A click will be performed on one of the node's ancestors.
    /// This happens when the node itself is not clickable, but one of its
    /// ancestors has click handlers attached which are able to capture the click
    /// as it bubbles up.
    ClickAncestor,
    Jump,
    Open,
    Press,
    Select,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SortDirection {
    Unsorted,
    Ascending,
    Descending,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AriaCurrent {
    False,
    True,
    Page,
    Step,
    Location,
    Date,
    Time,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HasPopup {
    True,
    Menu,
    Listbox,
    Tree,
    Grid,
    Dialog,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ListStyle {
    Circle,
    Disc,
    Image,
    Numeric,
    Square,
    /// Language specific ordering (alpha, roman, cjk-ideographic, etc...)
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VerticalOffset {
    Subscript,
    Superscript,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextDecoration {
    Solid,
    Dotted,
    Dashed,
    Double,
    Wavy,
}

// TBD: Should this be a struct? We want it to serialize as just an integer.
// This is NonZeroU64 because we regularly store Option<NodeId>.
pub type NodeId = std::num::NonZeroU64;

#[derive(Clone, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

/// 4x4 transformation matrix.
#[derive(Clone, PartialEq)]
pub struct Transform {
    /// Column major order.
    pub matrix: [f32; 16],
}

/// The relative bounding box of a [`Node`].
///
/// This is an efficient, compact, serializable representation of a node's
/// bounding box that requires minimal changes to the tree when layers are
/// moved or scrolled. Computing the absolute bounding box of a node requires
/// walking up the tree and applying node offsets and transforms until reaching
/// the top.
///
/// If [`RelativeBounds::offset_container_id`] is present, the bounds
/// are relative to the node with that ID.
///
/// Otherwise, for a node other than the root, the bounds are relative to
/// the root of the tree, and for the root of a tree, the bounds are relative
/// to its immediate containing node.
#[derive(Clone, PartialEq)]
pub struct RelativeBounds {
    /// The ID of an ancestor node in the same Tree that this object's
    /// bounding box is relative to.
    pub offset_container_id: Option<NodeId>,
    /// The relative bounding box of this node.
    pub bounds: Rect,
    /// An additional transform to apply to position this object and its subtree.
    /// This is rarely used and should be omitted if not needed, i.e. if
    /// the transform would be the identity matrix. It's rare enough
    // that we box it to reduce memory usage.
    pub transform: Option<Box<Transform>>,
}

/// A marker spanning a range within text.
#[derive(Clone, PartialEq)]
pub struct TextMarker {
    pub marker_type: MarkerType,
    /// Indices are in UTF-8 code units.
    pub range: Range<usize>,
}

/// Defines a custom action for a UI element. For example, a list UI
/// can allow a user to reorder items in the list by dragging the items.
#[derive(Clone, PartialEq)]
pub struct CustomAction {
    pub id: i32,
    pub description: String,
}

#[derive(Clone, PartialEq)]
pub struct NodeState {
    pub autofill_available: bool,
    pub collapsed: bool,
    pub expanded: bool,
    pub default: bool,
    pub editable: bool,
    pub focusable: bool,
    /// Grows horizontally, e.g. most toolbars and separators.
    pub horizontal: bool,
    /// Grows vertically, e.g. menu or combo box.
    pub vertical: bool,
    pub hovered: bool,
    /// Skip over this node in the accessibility tree, but keep its subtree.
    pub ignored: bool,
    pub invisible: bool,
    pub linked: bool,
    pub multiline: bool,
    pub multiselectable: bool,
    pub protected: bool,
    pub required: bool,
    pub richly_editable: bool,
    pub visited: bool,

    /// Use for a textbox that allows focus/selection but not input.
    pub read_only: bool,
    /// Use for a control or group of controls that disallows input.
    pub disabled: bool,
}

/// Indicates what functions can be performed when a dragged object
/// is released on the drop target.
/// Note: aria-dropeffect is deprecated in WAI-ARIA 1.1.
#[derive(Clone, PartialEq)]
pub struct DropEffects {
    pub copy: bool,
    pub execute: bool,
    pub link: bool,
    pub move_: bool,
    pub popup: bool,
}

#[derive(Clone, PartialEq)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub overline: Option<TextDecoration>,
    pub strikethrough: Option<TextDecoration>,
    pub underline: Option<TextDecoration>,
}

#[derive(Clone, PartialEq)]
pub enum NodeAttribute {
    Name(String),
    /// What information was used to compute the object's name.
    NameFrom(NameFrom),

    Description(String),
    /// What information was used to compute the object's description.
    DescriptionFrom(DescriptionFrom),

    Value(String),

    Busy(bool),

    /// The object functions as a text field which exposes its descendants.
    /// Use cases include the root of a content-editable region, an ARIA
    /// textbox which isn't currently editable and which has interactive
    /// descendants, and a <body> element that has "design-mode" set to "on".
    NonatomicTextFieldRoot(bool),

    // Live region attributes.
    ContainerLiveAtomic(bool),
    ContainerLiveBusy(bool),
    LiveAtomic(bool),

    /// If a dialog box is marked as explicitly modal
    Modal(bool),

    /// Set on a canvas element if it has fallback content.
    CanvasHasFallback(bool),

    /// Indicates this node is user-scrollable, e.g. overflow:scroll|auto, as
    /// opposed to only programmatically scrollable, like overflow:hidden, or
    /// not scrollable at all, e.g. overflow:visible.
    Scrollable(bool),

    /// A hint to clients that the node is clickable.
    Clickable(bool),

    /// Indicates that this node clips its children, i.e. may have
    /// overflow: hidden or clip children by default.
    ClipsChildren(bool),

    /// Indicates that this node is not selectable because the style has
    /// user-select: none. Note that there may be other reasons why a node is
    /// not selectable - for example, bullets in a list. However, this attribute
    /// is only set on user-select: none.
    NotUserSelectableStyle(bool),

    /// Indicates whether this node is selected or unselected.
    /// The absence of this flag (as opposed to a false setting)
    /// means that the concept of "selected" doesn't apply.
    /// When deciding whether to set the flag to false or omit it,
    /// consider whether it would be appropriate for a screen reader
    /// to announce "not selected". The ambiguity of this flag
    /// in platform accessibility APIs has made extraneous
    /// "not selected" announcements a common annoyance.
    Selected(bool),
    /// Indicates whether this node is selected due to selection follows focus.
    SelectedFromFocus(bool),

    /// Indicates whether this node can be grabbed for drag-and-drop operation.
    /// Setting this flag to false rather than omitting it means that
    /// this node is not currently grabbed but it can be.
    /// Note: aria-grabbed is deprecated in WAI-ARIA 1.1.
    Grabbed(bool),
    DropEffects(DropEffects),

    /// Indicates whether this node causes a hard line-break
    /// (e.g. block level elements, or <br>)
    IsLineBreakingObject(bool),
    /// Indicates whether this node causes a page break
    IsPageBreakingObject(bool),

    /// True if the node has any ARIA attributes set.
    HasAriaAttribute(bool),

    /// This element allows touches to be passed through when a screen reader
    /// is in touch exploration mode, e.g. a virtual keyboard normally
    /// behaves this way.
    TouchPassThrough(bool),

    /// Ids of nodes that are children of this node logically, but are
    /// not children of this node in the tree structure. As an example,
    /// a table cell is a child of a row, and an 'indirect' child of a
    /// column.
    IndirectChildIds(Vec<NodeId>),

    // Relationships between this node and other nodes.
    ActiveDescendantId(NodeId),
    ErrorMessageId(NodeId),
    InPageLinkTargetId(NodeId),
    MemberOfId(NodeId),
    NextOnLineId(NodeId),
    PreviousOnLineId(NodeId),
    PopupForId(NodeId),
    ControlsIds(Vec<NodeId>),
    DetailsIds(Vec<NodeId>),
    DescribedByIds(Vec<NodeId>),
    FlowToIds(Vec<NodeId>),
    LabelledByIds(Vec<NodeId>),
    RadioGroupIds(Vec<NodeId>),

    Markers(Vec<TextMarker>),

    TextDirection(TextDirection),
    /// For inline text. This is the pixel position of the end of each
    /// character within the bounding rectangle of this object, in the
    /// direction given by [`Node::text_direction`]. For example, for left-to-right
    /// text, the first offset is the right coordinate of the first
    /// character within the object's bounds, the second offset
    /// is the right coordinate of the second character, and so on.
    CharacterOffsets(Vec<f32>),

    /// For inline text. The UTF-8 code unit indices of each word.
    Words(Vec<Range<usize>>),

    CustomActions(Vec<CustomAction>),

    AccessKey(String),

    InvalidState(InvalidState),

    AutoComplete(String),

    CheckedState(CheckedState),
    CheckedStateDescription(String),

    ChildTreeId(String),

    ClassName(String),

    ContainerLiveRelevant(String),
    ContainerLiveStatus(String),

    CssDisplay(String),

    /// Only present when different from parent.
    FontFamily(String),

    HtmlTag(String),
    InnerHtml(String),

    InputType(String),

    KeyShortcuts(String),

    /// Only present when different from parent.
    Language(String),

    LiveRelevant(String),
    LiveStatus(String),

    /// Only if not already exposed in [`Node::name`] ([`NameFrom::Placeholder`]).
    Placeholder(String),

    CustomRole(String),
    RoleDescription(String),

    /// Only if not already exposed in [`Node::name`] ([`NameFrom::Title`]).
    Tooltip(String),

    Url(String),

    DefaultActionVerb(DefaultActionVerb),

    // Scrollable container attributes.
    ScrollX(f32),
    ScrollXMin(f32),
    ScrollXMax(f32),
    ScrollY(f32),
    ScrollYMin(f32),
    ScrollYMax(f32),

    /// The endpoints of a text selection, in UTF-8 code units.
    TextSelection(Range<usize>),

    AriaColumnCount(usize),
    AriaCellColumnIndex(usize),
    AriaCellColumnSpan(usize),
    AriaRowCount(usize),
    AriaCellRowIndex(usize),
    AriaCellRowSpan(usize),

    // Table attributes.
    TableRowCount(usize),
    TableColumnCount(usize),
    TableHeaderId(NodeId),

    // Table row attributes.
    TableRowIndex(usize),
    TableRowHeaderId(NodeId),

    // Table column attributes.
    TableColumnIndex(usize),
    TableColumnHeaderId(NodeId),

    // Table cell attributes.
    TableCellColumnIndex(usize),
    TableCellColumnSpan(usize),
    TableCellRowIndex(usize),
    TableCellRowSpan(usize),
    SortDirection(SortDirection),

    /// Tree control attributes.
    HierarchicalLevel(usize),

    // Position or Number of items in current set of listitems or treeitems
    SetSize(usize),
    PosInSet(usize),

    /// For [`Role::ColorWell`], specifies the selected color in RGBA.
    ColorValue(u32),

    AriaCurrent(AriaCurrent),

    /// Background color in RGBA.
    BackgroundColor(u32),
    /// Foreground color in RGBA.
    ForegroundColor(u32),

    HasPopup(HasPopup),

    /// The list style type. Only available on list items.
    ListStyle(ListStyle),

    TextAlign(TextAlign),
    VerticalOffset(VerticalOffset),

    TextStyle(TextStyle),

    // Focus traversal order.
    PreviousFocusId(NodeId),
    NextFocusId(NodeId),

    // Range attributes.
    ValueForRange(f32),
    MinValueForRange(f32),
    MaxValueForRange(f32),
    StepValueForRange(f32),

    // Text attributes.
    /// Font size is in pixels.
    FontSize(f32),
    /// Font weight can take on any arbitrary numeric value. Increments of 100 in
    /// range [0, 900] represent keywords such as light, normal, bold, etc.
    FontWeight(f32),
    /// The text indent of the text, in mm.
    TextIndent(f32),
}

/// A single accessible object. A complete UI is represented as a tree of these.
#[derive(Clone, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub role: Role,
    pub bounds: Option<RelativeBounds>,
    pub child_ids: Vec<NodeId>,
    pub state: NodeState,

    /// Unordered set of actions supported by this node.
    pub actions: Vec<Action>,

    /// Unordered list of the attributes of a node. Each type of attribute
    /// should appear at most once.
    pub attributes: Vec<NodeAttribute>,
}

/// The data associated with an accessibility tree that's global to the
/// tree and not associated with any particular node.
#[derive(Clone, PartialEq)]
pub struct Tree {
    /// The globally unique ID of this tree. The format of this ID
    /// is up to the implementer. A UUID v4 is a safe choice.
    pub id: String,

    /// The ID of the tree that this tree is contained in, if any.
    pub parent_tree_id: Option<String>,

    /// The ID of the tree that has focus, if it's not this tree
    /// but a descendant of it.
    pub focused_tree_id: Option<String>,
    /// The node with keyboard focus within this tree, if any.
    pub focused_node_id: Option<NodeId>,

    /// The node that's used as the root scroller, if any. On some platforms
    /// like Android we need to ignore accessibility scroll offsets for
    /// that node and get them from the viewport instead.
    pub root_scroller_id: Option<NodeId>,
}

/// A serializable representation of an atomic change to a tree.
/// The sender and receiver must be in sync; the update is only meant
/// to bring the tree from a specific previous state into its next state.
/// Trying to apply it to the wrong tree should immediately panic.
#[derive(Clone, PartialEq)]
pub struct TreeUpdate {
    /// The optional ID of a node to clear, before applying any updates.
    /// Clearing a node means deleting all of its children and their descendants,
    /// but leaving that node in the tree. It's an error to clear a node but not
    /// subsequently update it as part of the same `TreeUpdate`.
    pub node_id_to_clear: Option<NodeId>,

    /// An ordered list of zero or more node updates to apply to the tree.
    ///
    /// Suppose that the next [`Node`] to be applied is `node`. The following
    /// invariants must hold:
    ///
    /// * Either:
    ///     1. `node.id` is already in the tree, or
    ///     2. the tree is empty, and `node` is the new root of the tree.
    /// * Every child ID in `node.child_ids` must either be already a child
    ///   of this node, or a new ID not previously in the tree. It is not
    ///   allowed to "reparent" a child to this node without first removing
    ///   that child from its previous parent.
    /// * When a new ID appears in `node.child_ids`, the tree should create a
    ///   new uninitialized placeholder node for it immediately. That
    ///   placeholder must be updated within the same `TreeUpdate`, otherwise
    ///   it's a fatal error. This guarantees the tree is always complete
    ///   before or after a `TreeUpdate`.
    pub nodes: Vec<Node>,

    /// Updated information about the tree as a whole. This may be omitted
    /// if it has not changed since the previous update, but providing the same
    /// information again is also allowed. This is required when initializing
    /// a tree.
    pub tree: Option<Tree>,

    /// The ID of the tree's root node. This is required when the tree
    /// is being initialized or if the root is changing.
    pub root_id: Option<NodeId>,
}
