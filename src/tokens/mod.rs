#[derive(Debug, PartialEq, Clone)]
pub enum NenyrTokens {
    // Nenyr keywords
    Construct,
    Central,
    Layout,
    Module,
    Declare,
    Extending,
    Deriving,

    // Nenyr methods
    Imports,
    Typefaces,
    Breakpoints,
    Themes,
    Aliases,
    Variables,
    Class,

    // Import pattern
    Import,

    // Breakpoints pattern
    MobileFirst,
    DesktopFirst,

    // Themes pattern
    Light,
    Dark,

    // Animation pattern
    Fraction,
    Progressive,
    From,
    Halfway,
    To,

    // Value collectors
    //Unknown(char),
    StringLiteral(String),
    Number(f64),
    Identifier(String),

    // Syntax tokens
    ParenthesisOpen,
    ParenthesisClose,
    CurlyBracketOpen,
    CurlyBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    Comma,
    Colon,
    EndOfLine,
    StartOfFile,
    True,
    False,

    // Style Patterns
    Important,
    Stylesheet,
    Hover,
    Active,
    Focus,
    FirstChild,
    LastChild,
    FirstOfType,
    LastOfType,
    OnlyChild,
    OnlyOfType,
    Target,
    Visited,
    Checked,
    Disabled,
    Enabled,
    ReadOnly,
    ReadWrite,
    PlaceholderShown,
    Valid,
    Invalid,
    Required,
    Optional,
    Fullscreen,
    FocusWithin,
    FirstLine,
    FirstLetter,
    Before,
    After,
    OutOfRange,
    Root,
    Empty,
    PanoramicViewer,

    // Nenyr Properties
    Hyphens,
    FlexGrow,
    AspectRatio,
    AccentColor,
    BackdropFilter,
    Content,
    Gap,
    RowGap,
    Scale,
    Order,
    PointerEvents,
    Margin,
    MarginBottom,
    MarginLeft,
    MarginRight,
    MarginTop,
    Padding,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    PaddingTop,
    Height,
    Width,
    Filter,
    MaxHeight,
    MaxWidth,
    MinHeight,
    MinWidth,
    Border,
    BorderBottom,
    BorderBottomColor,
    BorderBottomStyle,
    BorderBottomWidth,
    BorderColor,
    BorderLeft,
    BorderLeftColor,
    BorderLeftStyle,
    BorderLeftWidth,
    BorderRight,
    BorderRightColor,
    BorderRightStyles,
    BorderRightWidth,
    BorderStyle,
    BorderTop,
    BorderTopColor,
    BorderTopStyle,
    BorderTopWidth,
    BorderWidth,
    Outline,
    OutlineColor,
    OutlineStyle,
    OutlineWidth,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    BorderImage,
    BorderImageOutset,
    BorderImageRepeat,
    BorderImageSlice,
    BorderImageSource,
    BorderImageWidth,
    BorderRadius,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BoxDecorationBreak,
    BoxShadow,
    Background,
    BackgroundAttachment,
    BackgroundColor,
    BackgroundImage,
    BackgroundPosition,
    BackgroundPositionX,
    BackgroundPositionY,
    BackgroundRepeat,
    BackgroundClip,
    BackgroundOrigin,
    BackgroundSize,
    BackgroundBlendMode,
    ColorProfile,
    Opacity,
    RenderingIntent,
    Font,
    FontFamily,
    FontSize,
    FontStyle,
    FontVariant,
    FontWeight,
    FontSizeAdjust,
    FontStretch,
    Positioning,
    Bottom,
    Clear,
    ClipPath,
    Cursor,
    Display,
    Float,
    Left,
    Overflow,
    Position,
    Right,
    Top,
    Visibility,
    ZIndex,
    Color,
    Direction,
    FlexDirection,
    FlexWrap,
    LetterSpacing,
    LineHeight,
    LineBreak,
    TextAlign,
    TextDecoration,
    TextIndent,
    TextTransform,
    UnicodeBidi,
    VerticalAlign,
    WhiteSpace,
    WordSpacing,
    TextOutline,
    TextOverflow,
    TextShadow,
    TextWrap,
    WordBreak,
    WordWrap,
    ListStyle,
    ListStyleImage,
    ListStylePosition,
    ListStyleType,
    BorderCollapse,
    BorderSpacing,
    CaptionSide,
    EmptyCells,
    TableLayout,
    MarqueeDirection,
    MarqueePlayCount,
    MarqueeSpeed,
    MarqueeStyle,
    OverflowX,
    OverflowY,
    OverflowStyle,
    Rotation,
    BoxAlign,
    BoxDirection,
    BoxFlex,
    BoxFlexGroup,
    BoxLines,
    BoxOrdinalGroup,
    BoxOrient,
    BoxPack,
    AlignmentAdjust,
    AlignmentBaseline,
    BaselineShift,
    DominantBaseline,
    DropInitialAfterAdjust,
    DropInitialAfterAlign,
    DropInitialBeforeAdjust,
    DropInitialBeforeAlign,
    DropInitialSize,
    DropInitialValue,
    InlineBoxAlign,
    LineStacking,
    LineStackingRuby,
    LineStackingShift,
    LineStackingStrategy,
    TextHeight,
    ColumnCount,
    ColumnFill,
    ColumnGap,
    ColumnRule,
    ColumnRuleColor,
    ColumnRuleStyle,
    ColumnRuleWidth,
    ColumnSpan,
    ColumnWidth,
    Columns,
    Animation,
    AnimationName,
    AnimationDuration,
    AnimationTimingFunction,
    AnimationDelay,
    AnimationFillMode,
    AnimationIterationCount,
    AnimationDirection,
    AnimationPlayState,
    Transform,
    TransformOrigin,
    TransformStyle,
    Perspective,
    PerspectiveOrigin,
    BackfaceVisibility,
    Transition,
    TransitionProperty,
    TransitionDuration,
    TransitionTimingFunction,
    TransitionDelay,
    Orphans,
    PageBreakAfter,
    PageBreakBefore,
    PageBreakInside,
    Widows,
    Mark,
    MarkAfter,
    MarkBefore,
    Phonemes,
    Rest,
    RestAfter,
    RestBefore,
    VoiceBalance,
    VoiceDuration,
    VoicePitch,
    VoicePitchRange,
    VoiceRate,
    VoiceStress,
    VoiceVolume,
    Appearance,
    BoxSizing,
    Icon,
    NavDown,
    NavIndex,
    NavLeft,
    NavRight,
    NavUp,
    OutlineOffset,
    Resize,
    Quotes,
    Rotate,
    Translate,
    UserSelect,
    WritingMode,
    ObjectPosition,
    ObjectFit,
    JustifySelf,
    JustifyContent,
    JustifyItems,
    AlignSelf,
    AlignContent,
    AlignItems,
    Grid,
    GridArea,
    GridAutoColumns,
    GridAutoFlow,
    GridAutoRows,
    GridColumn,
    GridColumnEnd,
    GridColumnStart,
    GridRow,
    GridRowEnd,
    GridRowStart,
    GridTemplate,
    GridTemplateAreas,
    GridTemplateColumns,
    GridTemplateRows,
    ScrollbarColor,
    ScrollbarWidth,
    ScrollbarGutter,
}
