use crate::tokens::NenyrTokens;

/// Trait responsible for converting Nenyr property tokens into corresponding CSS properties.
///
/// This trait provides a method to convert property tokens defined in Nenyr (the DSL used in Galadriel CSS)
/// into valid CSS property names. Each property token represents a specific CSS property, and the conversion
/// process is done via a match operation that returns the appropriate CSS property name.
///
/// ## Usage
///
/// This trait can be implemented by any struct that needs to handle the conversion of Nenyr property tokens
/// into CSS property names. The provided method `convert_nenyr_property_to_css_property` offers a
/// default implementation which maps various `NenyrTokens` property enum variants to their respective CSS
/// properties.
///
/// ## Provided Method
///
/// - `convert_nenyr_property_to_css_property`:
///   - Converts a given `NenyrTokens` property enum variant into its respective CSS property name.
///   - The method utilizes a match block to ensure that the correct CSS property string is returned
///     for each Nenyr token.
///   - The returned value is a `Some(String)` containing the corresponding CSS property name.
///
/// ## Performance Considerations
///
/// Since this method uses a simple match operation to convert property tokens, it operates in constant time
/// for each token lookup, making it highly efficient for real-time style conversions in large applications.
///
/// # Example of Property-Tokens-to-CSS-Properties Mapping:
///
/// | Nenyr property Token       | CSS Property               |
/// |----------------------------|----------------------------|
/// | `NenyrTokens::Padding`     | `Some("padding")`          |
/// | `NenyrTokens::Margin`      | `Some("margin")`           |
/// | `NenyrTokens::Border`      | `Some("border")`           |
/// | `NenyrTokens::FontSize`    | `Some("font-size")`        |
/// | `NenyrTokens::Background`  | `Some("background")`       |
pub trait NenyrPropertyConverter {
    /// Converts a Nenyr property token into a corresponding CSS property name.
    ///
    /// # Parameters
    ///
    /// - `nenyr_token`: The Nenyr property token that needs to be converted. This token is represented by
    ///   the `NenyrTokens` property enum, which defines a comprehensive set of tokens corresponding to various
    ///   CSS properties.
    ///
    /// # Returns
    ///
    /// A `Some(String)` containing the CSS property name that matches the given Nenyr token.
    fn convert_nenyr_property_to_css_property(&self, nenyr_token: &NenyrTokens) -> Option<String> {
        match nenyr_token {
            NenyrTokens::All => Some("all".to_string()),
            NenyrTokens::Hyphens => Some("hyphens".to_string()),
            NenyrTokens::FlexGrow => Some("flex-grow".to_string()),
            NenyrTokens::AspectRatio => Some("aspect-ratio".to_string()),
            NenyrTokens::AccentColor => Some("accent-color".to_string()),
            NenyrTokens::BackdropFilter => Some("backdrop-filter".to_string()),
            NenyrTokens::Content => Some("content".to_string()),
            NenyrTokens::Gap => Some("gap".to_string()),
            NenyrTokens::RowGap => Some("row-gap".to_string()),
            NenyrTokens::Scale => Some("scale".to_string()),
            NenyrTokens::Order => Some("order".to_string()),
            NenyrTokens::PointerEvents => Some("pointer-events".to_string()),
            NenyrTokens::Margin => Some("margin".to_string()),
            NenyrTokens::MarginBottom => Some("margin-bottom".to_string()),
            NenyrTokens::MarginLeft => Some("margin-left".to_string()),
            NenyrTokens::MarginRight => Some("margin-right".to_string()),
            NenyrTokens::MarginTop => Some("margin-top".to_string()),
            NenyrTokens::Padding => Some("padding".to_string()),
            NenyrTokens::PaddingBottom => Some("padding-bottom".to_string()),
            NenyrTokens::PaddingLeft => Some("padding-left".to_string()),
            NenyrTokens::PaddingRight => Some("padding-right".to_string()),
            NenyrTokens::PaddingTop => Some("padding-top".to_string()),
            NenyrTokens::Height => Some("height".to_string()),
            NenyrTokens::Width => Some("width".to_string()),
            NenyrTokens::Filter => Some("filter".to_string()),
            NenyrTokens::MaxHeight => Some("max-height".to_string()),
            NenyrTokens::MaxWidth => Some("max-width".to_string()),
            NenyrTokens::MinHeight => Some("min-height".to_string()),
            NenyrTokens::MinWidth => Some("min-width".to_string()),
            NenyrTokens::Border => Some("border".to_string()),
            NenyrTokens::BorderBottom => Some("border-bottom".to_string()),
            NenyrTokens::BorderBottomColor => Some("border-bottom-color".to_string()),
            NenyrTokens::BorderBottomStyle => Some("border-bottom-style".to_string()),
            NenyrTokens::BorderBottomWidth => Some("border-bottom-width".to_string()),
            NenyrTokens::BorderColor => Some("border-color".to_string()),
            NenyrTokens::BorderLeft => Some("border-left".to_string()),
            NenyrTokens::BorderLeftColor => Some("border-left-color".to_string()),
            NenyrTokens::BorderLeftStyle => Some("border-left-style".to_string()),
            NenyrTokens::BorderLeftWidth => Some("border-left-width".to_string()),
            NenyrTokens::BorderRight => Some("border-right".to_string()),
            NenyrTokens::BorderRightColor => Some("border-right-color".to_string()),
            NenyrTokens::BorderRightStyles => Some("border-right-styles".to_string()),
            NenyrTokens::BorderRightWidth => Some("border-right-width".to_string()),
            NenyrTokens::BorderStyle => Some("border-style".to_string()),
            NenyrTokens::BorderTop => Some("border-top".to_string()),
            NenyrTokens::BorderTopColor => Some("border-top-color".to_string()),
            NenyrTokens::BorderTopStyle => Some("border-top-style".to_string()),
            NenyrTokens::BorderTopWidth => Some("border-top-width".to_string()),
            NenyrTokens::BorderWidth => Some("border-width".to_string()),
            NenyrTokens::Outline => Some("outline".to_string()),
            NenyrTokens::OutlineColor => Some("outline-color".to_string()),
            NenyrTokens::OutlineStyle => Some("outline-style".to_string()),
            NenyrTokens::OutlineWidth => Some("outline-width".to_string()),
            NenyrTokens::BorderBottomLeftRadius => Some("border-bottom-left-radius".to_string()),
            NenyrTokens::BorderBottomRightRadius => Some("border-bottom-right-radius".to_string()),
            NenyrTokens::BorderImage => Some("border-image".to_string()),
            NenyrTokens::BorderImageOutset => Some("border-image-outset".to_string()),
            NenyrTokens::BorderImageRepeat => Some("border-image-repeat".to_string()),
            NenyrTokens::BorderImageSlice => Some("border-image-slice".to_string()),
            NenyrTokens::BorderImageSource => Some("border-image-source".to_string()),
            NenyrTokens::BorderImageWidth => Some("border-image-width".to_string()),
            NenyrTokens::BorderRadius => Some("border-radius".to_string()),
            NenyrTokens::BorderTopLeftRadius => Some("border-top-left-radius".to_string()),
            NenyrTokens::BorderTopRightRadius => Some("border-top-right-radius".to_string()),
            NenyrTokens::BoxDecorationBreak => Some("box-decoration-break".to_string()),
            NenyrTokens::BoxShadow => Some("box-shadow".to_string()),
            NenyrTokens::Background => Some("background".to_string()),
            NenyrTokens::BackgroundAttachment => Some("background-attachment".to_string()),
            NenyrTokens::BackgroundColor => Some("background-color".to_string()),
            NenyrTokens::BackgroundImage => Some("background-image".to_string()),
            NenyrTokens::BackgroundPosition => Some("background-position".to_string()),
            NenyrTokens::BackgroundPositionX => Some("background-position-x".to_string()),
            NenyrTokens::BackgroundPositionY => Some("background-position-y".to_string()),
            NenyrTokens::BackgroundRepeat => Some("background-repeat".to_string()),
            NenyrTokens::BackgroundClip => Some("background-clip".to_string()),
            NenyrTokens::BackgroundOrigin => Some("background-origin".to_string()),
            NenyrTokens::BackgroundSize => Some("background-size".to_string()),
            NenyrTokens::BackgroundBlendMode => Some("background-blend-mode".to_string()),
            NenyrTokens::ColorProfile => Some("color-profile".to_string()),
            NenyrTokens::Opacity => Some("opacity".to_string()),
            NenyrTokens::RenderingIntent => Some("rendering-intent".to_string()),
            NenyrTokens::Font => Some("font".to_string()),
            NenyrTokens::FontFamily => Some("font-family".to_string()),
            NenyrTokens::FontSize => Some("font-size".to_string()),
            NenyrTokens::FontStyle => Some("font-style".to_string()),
            NenyrTokens::FontVariant => Some("font-variant".to_string()),
            NenyrTokens::FontWeight => Some("font-weight".to_string()),
            NenyrTokens::FontSizeAdjust => Some("font-size-adjust".to_string()),
            NenyrTokens::FontStretch => Some("font-stretch".to_string()),
            NenyrTokens::Positioning => Some("positioning".to_string()),
            NenyrTokens::Bottom => Some("bottom".to_string()),
            NenyrTokens::Clear => Some("clear".to_string()),
            NenyrTokens::ClipPath => Some("clip-path".to_string()),
            NenyrTokens::Cursor => Some("cursor".to_string()),
            NenyrTokens::Display => Some("display".to_string()),
            NenyrTokens::Float => Some("float".to_string()),
            NenyrTokens::Left => Some("left".to_string()),
            NenyrTokens::Overflow => Some("overflow".to_string()),
            NenyrTokens::Position => Some("position".to_string()),
            NenyrTokens::Right => Some("right".to_string()),
            NenyrTokens::Top => Some("top".to_string()),
            NenyrTokens::Visibility => Some("visibility".to_string()),
            NenyrTokens::ZIndex => Some("z-index".to_string()),
            NenyrTokens::Color => Some("color".to_string()),
            NenyrTokens::Direction => Some("direction".to_string()),
            NenyrTokens::FlexDirection => Some("flex-direction".to_string()),
            NenyrTokens::FlexWrap => Some("flex-wrap".to_string()),
            NenyrTokens::LetterSpacing => Some("letter-spacing".to_string()),
            NenyrTokens::LineHeight => Some("line-height".to_string()),
            NenyrTokens::LineBreak => Some("line-break".to_string()),
            NenyrTokens::TextAlign => Some("text-align".to_string()),
            NenyrTokens::TextDecoration => Some("text-decoration".to_string()),
            NenyrTokens::TextIndent => Some("text-indent".to_string()),
            NenyrTokens::TextTransform => Some("text-transform".to_string()),
            NenyrTokens::UnicodeBidi => Some("unicode-bidi".to_string()),
            NenyrTokens::VerticalAlign => Some("vertical-align".to_string()),
            NenyrTokens::WhiteSpace => Some("white-space".to_string()),
            NenyrTokens::WordSpacing => Some("word-spacing".to_string()),
            NenyrTokens::TextOutline => Some("text-outline".to_string()),
            NenyrTokens::TextOverflow => Some("text-overflow".to_string()),
            NenyrTokens::TextShadow => Some("text-shadow".to_string()),
            NenyrTokens::TextWrap => Some("text-wrap".to_string()),
            NenyrTokens::WordBreak => Some("word-break".to_string()),
            NenyrTokens::WordWrap => Some("word-wrap".to_string()),
            NenyrTokens::ListStyle => Some("list-style".to_string()),
            NenyrTokens::ListStyleImage => Some("list-style-image".to_string()),
            NenyrTokens::ListStylePosition => Some("list-style-position".to_string()),
            NenyrTokens::ListStyleType => Some("list-style-type".to_string()),
            NenyrTokens::BorderCollapse => Some("border-collapse".to_string()),
            NenyrTokens::BorderSpacing => Some("border-spacing".to_string()),
            NenyrTokens::CaptionSide => Some("caption-side".to_string()),
            NenyrTokens::EmptyCells => Some("empty-cells".to_string()),
            NenyrTokens::TableLayout => Some("table-layout".to_string()),
            NenyrTokens::MarqueeDirection => Some("marquee-direction".to_string()),
            NenyrTokens::MarqueePlayCount => Some("marquee-play-count".to_string()),
            NenyrTokens::MarqueeSpeed => Some("marquee-speed".to_string()),
            NenyrTokens::MarqueeStyle => Some("marquee-style".to_string()),
            NenyrTokens::OverflowX => Some("overflow-x".to_string()),
            NenyrTokens::OverflowY => Some("overflow-y".to_string()),
            NenyrTokens::OverflowStyle => Some("overflow-style".to_string()),
            NenyrTokens::Rotation => Some("rotation".to_string()),
            NenyrTokens::BoxAlign => Some("box-align".to_string()),
            NenyrTokens::BoxDirection => Some("box-direction".to_string()),
            NenyrTokens::BoxFlex => Some("box-flex".to_string()),
            NenyrTokens::BoxFlexGroup => Some("box-flex-group".to_string()),
            NenyrTokens::BoxLines => Some("box-lines".to_string()),
            NenyrTokens::BoxOrdinalGroup => Some("box-ordinal-group".to_string()),
            NenyrTokens::BoxOrient => Some("box-orient".to_string()),
            NenyrTokens::BoxPack => Some("box-pack".to_string()),
            NenyrTokens::AlignmentAdjust => Some("alignment-adjust".to_string()),
            NenyrTokens::AlignmentBaseline => Some("alignment-baseline".to_string()),
            NenyrTokens::BaselineShift => Some("baseline-shift".to_string()),
            NenyrTokens::DominantBaseline => Some("dominant-baseline".to_string()),
            NenyrTokens::DropInitialAfterAdjust => Some("drop-initial-after-adjust".to_string()),
            NenyrTokens::DropInitialAfterAlign => Some("drop-initial-after-align".to_string()),
            NenyrTokens::DropInitialBeforeAdjust => Some("drop-initial-before-adjust".to_string()),
            NenyrTokens::DropInitialBeforeAlign => Some("drop-initial-before-align".to_string()),
            NenyrTokens::DropInitialSize => Some("drop-initial-size".to_string()),
            NenyrTokens::DropInitialValue => Some("drop-initial-value".to_string()),
            NenyrTokens::InlineBoxAlign => Some("inline-box-align".to_string()),
            NenyrTokens::LineStacking => Some("line-stacking".to_string()),
            NenyrTokens::LineStackingRuby => Some("line-stacking-ruby".to_string()),
            NenyrTokens::LineStackingShift => Some("line-stacking-shift".to_string()),
            NenyrTokens::LineStackingStrategy => Some("line-stacking-strategy".to_string()),
            NenyrTokens::TextHeight => Some("text-height".to_string()),
            NenyrTokens::ColumnCount => Some("column-count".to_string()),
            NenyrTokens::ColumnFill => Some("column-fill".to_string()),
            NenyrTokens::ColumnGap => Some("column-gap".to_string()),
            NenyrTokens::ColumnRule => Some("column-rule".to_string()),
            NenyrTokens::ColumnRuleColor => Some("column-rule-color".to_string()),
            NenyrTokens::ColumnRuleStyle => Some("column-rule-style".to_string()),
            NenyrTokens::ColumnRuleWidth => Some("column-rule-width".to_string()),
            NenyrTokens::ColumnSpan => Some("column-span".to_string()),
            NenyrTokens::ColumnWidth => Some("column-width".to_string()),
            NenyrTokens::Columns => Some("columns".to_string()),
            NenyrTokens::Animation => Some("animation".to_string()),
            NenyrTokens::AnimationName => Some("animation-name".to_string()),
            NenyrTokens::AnimationDuration => Some("animation-duration".to_string()),
            NenyrTokens::AnimationTimingFunction => Some("animation-timing-function".to_string()),
            NenyrTokens::AnimationDelay => Some("animation-delay".to_string()),
            NenyrTokens::AnimationFillMode => Some("animation-fill-mode".to_string()),
            NenyrTokens::AnimationIterationCount => Some("animation-iteration-count".to_string()),
            NenyrTokens::AnimationDirection => Some("animation-direction".to_string()),
            NenyrTokens::AnimationPlayState => Some("animation-play-state".to_string()),
            NenyrTokens::Transform => Some("transform".to_string()),
            NenyrTokens::TransformOrigin => Some("transform-origin".to_string()),
            NenyrTokens::TransformStyle => Some("transform-style".to_string()),
            NenyrTokens::Perspective => Some("perspective".to_string()),
            NenyrTokens::PerspectiveOrigin => Some("perspective-origin".to_string()),
            NenyrTokens::BackfaceVisibility => Some("backface-visibility".to_string()),
            NenyrTokens::Transition => Some("transition".to_string()),
            NenyrTokens::TransitionProperty => Some("transition-property".to_string()),
            NenyrTokens::TransitionDuration => Some("transition-duration".to_string()),
            NenyrTokens::TransitionTimingFunction => Some("transition-timing-function".to_string()),
            NenyrTokens::TransitionDelay => Some("transition-delay".to_string()),
            NenyrTokens::Orphans => Some("orphans".to_string()),
            NenyrTokens::PageBreakAfter => Some("page-break-after".to_string()),
            NenyrTokens::PageBreakBefore => Some("page-break-before".to_string()),
            NenyrTokens::PageBreakInside => Some("page-break-inside".to_string()),
            NenyrTokens::Widows => Some("widows".to_string()),
            NenyrTokens::Mark => Some("mark".to_string()),
            NenyrTokens::MarkAfter => Some("mark-after".to_string()),
            NenyrTokens::MarkBefore => Some("mark-before".to_string()),
            NenyrTokens::Phonemes => Some("phonemes".to_string()),
            NenyrTokens::Rest => Some("rest".to_string()),
            NenyrTokens::RestAfter => Some("rest-after".to_string()),
            NenyrTokens::RestBefore => Some("rest-before".to_string()),
            NenyrTokens::VoiceBalance => Some("voice-balance".to_string()),
            NenyrTokens::VoiceDuration => Some("voice-duration".to_string()),
            NenyrTokens::VoicePitch => Some("voice-pitch".to_string()),
            NenyrTokens::VoicePitchRange => Some("voice-pitch-range".to_string()),
            NenyrTokens::VoiceRate => Some("voice-rate".to_string()),
            NenyrTokens::VoiceStress => Some("voice-stress".to_string()),
            NenyrTokens::VoiceVolume => Some("voice-volume".to_string()),
            NenyrTokens::Appearance => Some("appearance".to_string()),
            NenyrTokens::BoxSizing => Some("box-sizing".to_string()),
            NenyrTokens::Icon => Some("icon".to_string()),
            NenyrTokens::NavDown => Some("nav-down".to_string()),
            NenyrTokens::NavIndex => Some("nav-index".to_string()),
            NenyrTokens::NavLeft => Some("nav-left".to_string()),
            NenyrTokens::NavRight => Some("nav-right".to_string()),
            NenyrTokens::NavUp => Some("nav-up".to_string()),
            NenyrTokens::OutlineOffset => Some("outline-offset".to_string()),
            NenyrTokens::Resize => Some("resize".to_string()),
            NenyrTokens::Quotes => Some("quotes".to_string()),
            NenyrTokens::Rotate => Some("rotate".to_string()),
            NenyrTokens::Translate => Some("translate".to_string()),
            NenyrTokens::UserSelect => Some("user-select".to_string()),
            NenyrTokens::WritingMode => Some("writing-mode".to_string()),
            NenyrTokens::ObjectPosition => Some("object-position".to_string()),
            NenyrTokens::ObjectFit => Some("object-fit".to_string()),
            NenyrTokens::JustifySelf => Some("justify-self".to_string()),
            NenyrTokens::JustifyContent => Some("justify-content".to_string()),
            NenyrTokens::JustifyItems => Some("justify-items".to_string()),
            NenyrTokens::AlignSelf => Some("align-self".to_string()),
            NenyrTokens::AlignContent => Some("align-content".to_string()),
            NenyrTokens::AlignItems => Some("align-items".to_string()),
            NenyrTokens::Grid => Some("grid".to_string()),
            NenyrTokens::GridArea => Some("grid-area".to_string()),
            NenyrTokens::GridAutoColumns => Some("grid-auto-columns".to_string()),
            NenyrTokens::GridAutoFlow => Some("grid-auto-flow".to_string()),
            NenyrTokens::GridAutoRows => Some("grid-auto-rows".to_string()),
            NenyrTokens::GridColumn => Some("grid-column".to_string()),
            NenyrTokens::GridColumnEnd => Some("grid-column-end".to_string()),
            NenyrTokens::GridColumnStart => Some("grid-column-start".to_string()),
            NenyrTokens::GridRow => Some("grid-row".to_string()),
            NenyrTokens::GridRowEnd => Some("grid-row-end".to_string()),
            NenyrTokens::GridRowStart => Some("grid-row-start".to_string()),
            NenyrTokens::GridTemplate => Some("grid-template".to_string()),
            NenyrTokens::GridTemplateAreas => Some("grid-template-areas".to_string()),
            NenyrTokens::GridTemplateColumns => Some("grid-template-columns".to_string()),
            NenyrTokens::GridTemplateRows => Some("grid-template-rows".to_string()),
            NenyrTokens::ScrollbarColor => Some("scrollbar-color".to_string()),
            NenyrTokens::ScrollbarWidth => Some("scrollbar-width".to_string()),
            NenyrTokens::ScrollbarGutter => Some("scrollbar-gutter".to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::NenyrTokens;

    use super::NenyrPropertyConverter;

    struct NenyrToken {}

    impl NenyrToken {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrPropertyConverter for NenyrToken {}

    #[test]
    fn all_tokens_are_valid() {
        let nenyr_token = NenyrToken::new();

        assert_eq!(
            Some("aspect-ratio".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AspectRatio)
        );
        assert_eq!(
            Some("accent-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AccentColor)
        );
        assert_eq!(
            Some("backdrop-filter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackdropFilter)
        );
        assert_eq!(
            Some("content".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Content)
        );
        assert_eq!(
            Some("gap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Gap)
        );
        assert_eq!(
            Some("row-gap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::RowGap)
        );
        assert_eq!(
            Some("scale".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Scale)
        );
        assert_eq!(
            Some("order".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Order)
        );
        assert_eq!(
            Some("pointer-events".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PointerEvents)
        );
        assert_eq!(
            Some("margin".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Margin)
        );
        assert_eq!(
            Some("margin-bottom".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarginBottom)
        );
        assert_eq!(
            Some("margin-left".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarginLeft)
        );
        assert_eq!(
            Some("margin-right".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarginRight)
        );
        assert_eq!(
            Some("margin-top".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarginTop)
        );
        assert_eq!(
            Some("padding".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Padding)
        );
        assert_eq!(
            Some("padding-bottom".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PaddingBottom)
        );
        assert_eq!(
            Some("padding-left".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PaddingLeft)
        );
        assert_eq!(
            Some("padding-right".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PaddingRight)
        );
        assert_eq!(
            Some("padding-top".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PaddingTop)
        );
        assert_eq!(
            Some("height".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Height)
        );
        assert_eq!(
            Some("width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Width)
        );
        assert_eq!(
            Some("filter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Filter)
        );
        assert_eq!(
            Some("max-height".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MaxHeight)
        );
        assert_eq!(
            Some("max-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MaxWidth)
        );
        assert_eq!(
            Some("min-height".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MinHeight)
        );
        assert_eq!(
            Some("min-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MinWidth)
        );
        assert_eq!(
            Some("border".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Border)
        );
        assert_eq!(
            Some("border-bottom".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottom)
        );
        assert_eq!(
            Some("border-bottom-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomColor)
        );
        assert_eq!(
            Some("border-bottom-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomStyle)
        );
        assert_eq!(
            Some("border-bottom-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomWidth)
        );
        assert_eq!(
            Some("border-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderColor)
        );
        assert_eq!(
            Some("border-left".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderLeft)
        );
        assert_eq!(
            Some("border-left-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderLeftColor)
        );
        assert_eq!(
            Some("border-left-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderLeftStyle)
        );
        assert_eq!(
            Some("border-left-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderLeftWidth)
        );
        assert_eq!(
            Some("border-right".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRight)
        );
        assert_eq!(
            Some("border-right-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRightColor)
        );
        assert_eq!(
            Some("border-right-styles".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRightStyles)
        );
        assert_eq!(
            Some("border-right-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRightWidth)
        );
        assert_eq!(
            Some("border-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderStyle)
        );
        assert_eq!(
            Some("border-top".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTop)
        );
        assert_eq!(
            Some("border-top-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopColor)
        );
        assert_eq!(
            Some("border-top-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopStyle)
        );
        assert_eq!(
            Some("border-top-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopWidth)
        );
        assert_eq!(
            Some("border-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderWidth)
        );
        assert_eq!(
            Some("outline".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Outline)
        );
        assert_eq!(
            Some("outline-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OutlineColor)
        );
        assert_eq!(
            Some("outline-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OutlineStyle)
        );
        assert_eq!(
            Some("outline-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OutlineWidth)
        );
        assert_eq!(
            Some("border-bottom-left-radius".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomLeftRadius)
        );
        assert_eq!(
            Some("border-bottom-right-radius".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomRightRadius)
        );
        assert_eq!(
            Some("border-image".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImage)
        );
        assert_eq!(
            Some("border-image-outset".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageOutset)
        );
        assert_eq!(
            Some("border-image-repeat".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageRepeat)
        );
        assert_eq!(
            Some("border-image-slice".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageSlice)
        );
        assert_eq!(
            Some("border-image-source".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageSource)
        );
        assert_eq!(
            Some("border-image-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageWidth)
        );
        assert_eq!(
            Some("border-radius".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRadius)
        );
        assert_eq!(
            Some("border-top-left-radius".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopLeftRadius)
        );
        assert_eq!(
            Some("border-top-right-radius".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopRightRadius)
        );
        assert_eq!(
            Some("box-decoration-break".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxDecorationBreak)
        );
        assert_eq!(
            Some("box-shadow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxShadow)
        );
        assert_eq!(
            Some("background".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Background)
        );
        assert_eq!(
            Some("background-attachment".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundAttachment)
        );
        assert_eq!(
            Some("background-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundColor)
        );
        assert_eq!(
            Some("background-image".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundImage)
        );
        assert_eq!(
            Some("background-position".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundPosition)
        );
        assert_eq!(
            Some("background-position-x".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundPositionX)
        );
        assert_eq!(
            Some("background-position-y".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundPositionY)
        );
        assert_eq!(
            Some("background-repeat".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundRepeat)
        );
        assert_eq!(
            Some("background-clip".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundClip)
        );
        assert_eq!(
            Some("background-origin".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundOrigin)
        );
        assert_eq!(
            Some("background-size".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundSize)
        );
        assert_eq!(
            Some("background-blend-mode".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundBlendMode)
        );
        assert_eq!(
            Some("color-profile".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColorProfile)
        );
        assert_eq!(
            Some("opacity".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Opacity)
        );
        assert_eq!(
            Some("rendering-intent".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::RenderingIntent)
        );
        assert_eq!(
            Some("font".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Font)
        );
        assert_eq!(
            Some("font-family".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontFamily)
        );
        assert_eq!(
            Some("font-size".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontSize)
        );
        assert_eq!(
            Some("font-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontStyle)
        );
        assert_eq!(
            Some("font-variant".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontVariant)
        );
        assert_eq!(
            Some("font-weight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontWeight)
        );
        assert_eq!(
            Some("font-size-adjust".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontSizeAdjust)
        );
        assert_eq!(
            Some("font-stretch".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontStretch)
        );
        assert_eq!(
            Some("positioning".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Positioning)
        );
        assert_eq!(
            Some("bottom".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Bottom)
        );
        assert_eq!(
            Some("clear".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Clear)
        );
        assert_eq!(
            Some("clip-path".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ClipPath)
        );
        assert_eq!(
            Some("cursor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Cursor)
        );
        assert_eq!(
            Some("display".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Display)
        );
        assert_eq!(
            Some("float".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Float)
        );
        assert_eq!(
            Some("left".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Left)
        );
        assert_eq!(
            Some("overflow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Overflow)
        );
        assert_eq!(
            Some("position".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Position)
        );
        assert_eq!(
            Some("right".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Right)
        );
        assert_eq!(
            Some("top".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Top)
        );
        assert_eq!(
            Some("visibility".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Visibility)
        );
        assert_eq!(
            Some("z-index".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ZIndex)
        );
        assert_eq!(
            Some("color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Color)
        );
        assert_eq!(
            Some("direction".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Direction)
        );
        assert_eq!(
            Some("flex-direction".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FlexDirection)
        );
        assert_eq!(
            Some("flex-wrap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FlexWrap)
        );
        assert_eq!(
            Some("letter-spacing".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LetterSpacing)
        );
        assert_eq!(
            Some("line-height".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineHeight)
        );
        assert_eq!(
            Some("line-break".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineBreak)
        );
        assert_eq!(
            Some("text-align".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextAlign)
        );
        assert_eq!(
            Some("text-decoration".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextDecoration)
        );
        assert_eq!(
            Some("text-indent".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextIndent)
        );
        assert_eq!(
            Some("text-transform".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextTransform)
        );
        assert_eq!(
            Some("unicode-bidi".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::UnicodeBidi)
        );
        assert_eq!(
            Some("vertical-align".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VerticalAlign)
        );
        assert_eq!(
            Some("white-space".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WhiteSpace)
        );
        assert_eq!(
            Some("word-spacing".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WordSpacing)
        );
        assert_eq!(
            Some("text-outline".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextOutline)
        );
        assert_eq!(
            Some("text-overflow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextOverflow)
        );
        assert_eq!(
            Some("text-shadow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextShadow)
        );
        assert_eq!(
            Some("text-wrap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextWrap)
        );
        assert_eq!(
            Some("word-break".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WordBreak)
        );
        assert_eq!(
            Some("word-wrap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WordWrap)
        );
        assert_eq!(
            Some("list-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ListStyle)
        );
        assert_eq!(
            Some("list-style-image".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ListStyleImage)
        );
        assert_eq!(
            Some("list-style-position".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ListStylePosition)
        );
        assert_eq!(
            Some("list-style-type".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ListStyleType)
        );
        assert_eq!(
            Some("border-collapse".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderCollapse)
        );
        assert_eq!(
            Some("border-spacing".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderSpacing)
        );
        assert_eq!(
            Some("caption-side".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::CaptionSide)
        );
        assert_eq!(
            Some("empty-cells".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::EmptyCells)
        );
        assert_eq!(
            Some("table-layout".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TableLayout)
        );
        assert_eq!(
            Some("marquee-direction".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarqueeDirection)
        );
        assert_eq!(
            Some("marquee-play-count".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarqueePlayCount)
        );
        assert_eq!(
            Some("marquee-speed".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarqueeSpeed)
        );
        assert_eq!(
            Some("marquee-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarqueeStyle)
        );
        assert_eq!(
            Some("overflow-x".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OverflowX)
        );
        assert_eq!(
            Some("overflow-y".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OverflowY)
        );
        assert_eq!(
            Some("overflow-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OverflowStyle)
        );
        assert_eq!(
            Some("rotation".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Rotation)
        );
        assert_eq!(
            Some("box-align".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxAlign)
        );
        assert_eq!(
            Some("box-direction".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxDirection)
        );
        assert_eq!(
            Some("box-flex".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxFlex)
        );
        assert_eq!(
            Some("box-flex-group".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxFlexGroup)
        );
        assert_eq!(
            Some("box-lines".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxLines)
        );
        assert_eq!(
            Some("box-ordinal-group".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxOrdinalGroup)
        );
        assert_eq!(
            Some("box-orient".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxOrient)
        );
        assert_eq!(
            Some("box-pack".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxPack)
        );
        assert_eq!(
            Some("alignment-adjust".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignmentAdjust)
        );
        assert_eq!(
            Some("alignment-baseline".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignmentBaseline)
        );
        assert_eq!(
            Some("baseline-shift".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BaselineShift)
        );
        assert_eq!(
            Some("dominant-baseline".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::DominantBaseline)
        );
        assert_eq!(
            Some("drop-initial-after-adjust".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialAfterAdjust)
        );
        assert_eq!(
            Some("drop-initial-after-align".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialAfterAlign)
        );
        assert_eq!(
            Some("drop-initial-before-adjust".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialBeforeAdjust)
        );
        assert_eq!(
            Some("drop-initial-before-align".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialBeforeAlign)
        );
        assert_eq!(
            Some("drop-initial-size".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialSize)
        );
        assert_eq!(
            Some("drop-initial-value".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialValue)
        );
        assert_eq!(
            Some("inline-box-align".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::InlineBoxAlign)
        );
        assert_eq!(
            Some("line-stacking".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineStacking)
        );
        assert_eq!(
            Some("line-stacking-ruby".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineStackingRuby)
        );
        assert_eq!(
            Some("line-stacking-shift".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineStackingShift)
        );
        assert_eq!(
            Some("line-stacking-strategy".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineStackingStrategy)
        );
        assert_eq!(
            Some("text-height".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextHeight)
        );
        assert_eq!(
            Some("column-count".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnCount)
        );
        assert_eq!(
            Some("column-fill".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnFill)
        );
        assert_eq!(
            Some("column-gap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnGap)
        );
        assert_eq!(
            Some("column-rule".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnRule)
        );
        assert_eq!(
            Some("column-rule-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnRuleColor)
        );
        assert_eq!(
            Some("column-rule-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnRuleStyle)
        );
        assert_eq!(
            Some("column-rule-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnRuleWidth)
        );
        assert_eq!(
            Some("column-span".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnSpan)
        );
        assert_eq!(
            Some("column-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnWidth)
        );
        assert_eq!(
            Some("columns".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Columns)
        );
        assert_eq!(
            Some("animation".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Animation)
        );
        assert_eq!(
            Some("animation-name".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationName)
        );
        assert_eq!(
            Some("animation-duration".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationDuration)
        );
        assert_eq!(
            Some("animation-timing-function".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::AnimationTimingFunction)
        );
        assert_eq!(
            Some("animation-delay".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationDelay)
        );
        assert_eq!(
            Some("animation-fill-mode".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationFillMode)
        );
        assert_eq!(
            Some("animation-iteration-count".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::AnimationIterationCount)
        );
        assert_eq!(
            Some("animation-direction".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationDirection)
        );
        assert_eq!(
            Some("animation-play-state".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationPlayState)
        );
        assert_eq!(
            Some("transform".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Transform)
        );
        assert_eq!(
            Some("transform-origin".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransformOrigin)
        );
        assert_eq!(
            Some("transform-style".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransformStyle)
        );
        assert_eq!(
            Some("perspective".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Perspective)
        );
        assert_eq!(
            Some("perspective-origin".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PerspectiveOrigin)
        );
        assert_eq!(
            Some("backface-visibility".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackfaceVisibility)
        );
        assert_eq!(
            Some("transition".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Transition)
        );
        assert_eq!(
            Some("transition-property".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransitionProperty)
        );
        assert_eq!(
            Some("transition-duration".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransitionDuration)
        );
        assert_eq!(
            Some("transition-timing-function".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::TransitionTimingFunction)
        );
        assert_eq!(
            Some("transition-delay".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransitionDelay)
        );
        assert_eq!(
            Some("orphans".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Orphans)
        );
        assert_eq!(
            Some("page-break-after".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PageBreakAfter)
        );
        assert_eq!(
            Some("page-break-before".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PageBreakBefore)
        );
        assert_eq!(
            Some("page-break-inside".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PageBreakInside)
        );
        assert_eq!(
            Some("widows".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Widows)
        );
        assert_eq!(
            Some("mark".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Mark)
        );
        assert_eq!(
            Some("mark-after".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarkAfter)
        );
        assert_eq!(
            Some("mark-before".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarkBefore)
        );
        assert_eq!(
            Some("phonemes".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Phonemes)
        );
        assert_eq!(
            Some("rest".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Rest)
        );
        assert_eq!(
            Some("rest-after".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::RestAfter)
        );
        assert_eq!(
            Some("rest-before".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::RestBefore)
        );
        assert_eq!(
            Some("voice-balance".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceBalance)
        );
        assert_eq!(
            Some("voice-duration".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceDuration)
        );
        assert_eq!(
            Some("voice-pitch".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoicePitch)
        );
        assert_eq!(
            Some("voice-pitch-range".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoicePitchRange)
        );
        assert_eq!(
            Some("voice-rate".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceRate)
        );
        assert_eq!(
            Some("voice-stress".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceStress)
        );
        assert_eq!(
            Some("voice-volume".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceVolume)
        );
        assert_eq!(
            Some("appearance".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Appearance)
        );
        assert_eq!(
            Some("box-sizing".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxSizing)
        );
        assert_eq!(
            Some("icon".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Icon)
        );
        assert_eq!(
            Some("nav-down".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavDown)
        );
        assert_eq!(
            Some("nav-index".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavIndex)
        );
        assert_eq!(
            Some("nav-left".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavLeft)
        );
        assert_eq!(
            Some("nav-right".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavRight)
        );
        assert_eq!(
            Some("nav-up".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavUp)
        );
        assert_eq!(
            Some("outline-offset".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OutlineOffset)
        );
        assert_eq!(
            Some("resize".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Resize)
        );
        assert_eq!(
            Some("quotes".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Quotes)
        );
        assert_eq!(
            Some("rotate".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Rotate)
        );
        assert_eq!(
            Some("translate".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Translate)
        );
        assert_eq!(
            Some("user-select".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::UserSelect)
        );
        assert_eq!(
            Some("writing-mode".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WritingMode)
        );
        assert_eq!(
            Some("object-position".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ObjectPosition)
        );
        assert_eq!(
            Some("object-fit".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ObjectFit)
        );
        assert_eq!(
            Some("justify-self".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::JustifySelf)
        );
        assert_eq!(
            Some("justify-content".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::JustifyContent)
        );
        assert_eq!(
            Some("justify-items".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::JustifyItems)
        );
        assert_eq!(
            Some("align-self".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignSelf)
        );
        assert_eq!(
            Some("align-content".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignContent)
        );
        assert_eq!(
            Some("align-items".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignItems)
        );
        assert_eq!(
            Some("grid".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Grid)
        );
        assert_eq!(
            Some("grid-area".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridArea)
        );
        assert_eq!(
            Some("grid-auto-columns".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridAutoColumns)
        );
        assert_eq!(
            Some("grid-auto-flow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridAutoFlow)
        );
        assert_eq!(
            Some("grid-auto-rows".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridAutoRows)
        );
        assert_eq!(
            Some("grid-column".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridColumn)
        );
        assert_eq!(
            Some("grid-column-end".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridColumnEnd)
        );
        assert_eq!(
            Some("grid-column-start".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridColumnStart)
        );
        assert_eq!(
            Some("grid-row".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridRow)
        );
        assert_eq!(
            Some("grid-row-end".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridRowEnd)
        );
        assert_eq!(
            Some("grid-row-start".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridRowStart)
        );
        assert_eq!(
            Some("grid-template".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridTemplate)
        );
        assert_eq!(
            Some("grid-template-areas".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridTemplateAreas)
        );
        assert_eq!(
            Some("grid-template-columns".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridTemplateColumns)
        );
        assert_eq!(
            Some("grid-template-rows".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridTemplateRows)
        );
        assert_eq!(
            Some("scrollbar-color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ScrollbarColor)
        );
        assert_eq!(
            Some("scrollbar-width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ScrollbarWidth)
        );
        assert_eq!(
            Some("scrollbar-gutter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ScrollbarGutter)
        );
    }

    #[test]
    fn all_tokens_are_not_valid() {
        let nenyr_token = NenyrToken::new();

        assert_ne!(
            Some("AspectRatio".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AspectRatio)
        );
        assert_ne!(
            Some("AccentColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AccentColor)
        );
        assert_ne!(
            Some("BackdropFilter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackdropFilter)
        );
        assert_ne!(
            Some("Content".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Content)
        );
        assert_ne!(
            Some("Gap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Gap)
        );
        assert_ne!(
            Some("RowGap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::RowGap)
        );
        assert_ne!(
            Some("Scale".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Scale)
        );
        assert_ne!(
            Some("Order".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Order)
        );
        assert_ne!(
            Some("PointerEvents".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PointerEvents)
        );
        assert_ne!(
            Some("Margin".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Margin)
        );
        assert_ne!(
            Some("MarginBottom".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarginBottom)
        );
        assert_ne!(
            Some("MarginLeft".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarginLeft)
        );
        assert_ne!(
            Some("MarginRight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarginRight)
        );
        assert_ne!(
            Some("MarginTop".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarginTop)
        );
        assert_ne!(
            Some("Padding".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Padding)
        );
        assert_ne!(
            Some("PaddingBottom".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PaddingBottom)
        );
        assert_ne!(
            Some("PaddingLeft".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PaddingLeft)
        );
        assert_ne!(
            Some("PaddingRight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PaddingRight)
        );
        assert_ne!(
            Some("PaddingTop".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PaddingTop)
        );
        assert_ne!(
            Some("Height".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Height)
        );
        assert_ne!(
            Some("Width".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Width)
        );
        assert_ne!(
            Some("Filter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Filter)
        );
        assert_ne!(
            Some("MaxHeight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MaxHeight)
        );
        assert_ne!(
            Some("MaxWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MaxWidth)
        );
        assert_ne!(
            Some("MinHeight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MinHeight)
        );
        assert_ne!(
            Some("MinWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MinWidth)
        );
        assert_ne!(
            Some("Border".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Border)
        );
        assert_ne!(
            Some("BorderBottom".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottom)
        );
        assert_ne!(
            Some("BorderBottomColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomColor)
        );
        assert_ne!(
            Some("BorderBottomStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomStyle)
        );
        assert_ne!(
            Some("BorderBottomWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomWidth)
        );
        assert_ne!(
            Some("BorderColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderColor)
        );
        assert_ne!(
            Some("BorderLeft".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderLeft)
        );
        assert_ne!(
            Some("BorderLeftColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderLeftColor)
        );
        assert_ne!(
            Some("BorderLeftStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderLeftStyle)
        );
        assert_ne!(
            Some("BorderLeftWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderLeftWidth)
        );
        assert_ne!(
            Some("BorderRight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRight)
        );
        assert_ne!(
            Some("BorderRightColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRightColor)
        );
        assert_ne!(
            Some("BorderRightStyles".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRightStyles)
        );
        assert_ne!(
            Some("BorderRightWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRightWidth)
        );
        assert_ne!(
            Some("BorderStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderStyle)
        );
        assert_ne!(
            Some("BorderTop".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTop)
        );
        assert_ne!(
            Some("BorderTopColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopColor)
        );
        assert_ne!(
            Some("BorderTopStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopStyle)
        );
        assert_ne!(
            Some("BorderTopWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopWidth)
        );
        assert_ne!(
            Some("BorderWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderWidth)
        );
        assert_ne!(
            Some("Outline".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Outline)
        );
        assert_ne!(
            Some("OutlineColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OutlineColor)
        );
        assert_ne!(
            Some("OutlineStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OutlineStyle)
        );
        assert_ne!(
            Some("OutlineWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OutlineWidth)
        );
        assert_ne!(
            Some("BorderBottomLeftRadius".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomLeftRadius)
        );
        assert_ne!(
            Some("BorderBottomRightRadius".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::BorderBottomRightRadius)
        );
        assert_ne!(
            Some("BorderImage".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImage)
        );
        assert_ne!(
            Some("BorderImageOutset".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageOutset)
        );
        assert_ne!(
            Some("BorderImageRepeat".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageRepeat)
        );
        assert_ne!(
            Some("BorderImageSlice".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageSlice)
        );
        assert_ne!(
            Some("BorderImageSource".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageSource)
        );
        assert_ne!(
            Some("BorderImageWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderImageWidth)
        );
        assert_ne!(
            Some("BorderRadius".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderRadius)
        );
        assert_ne!(
            Some("BorderTopLeftRadius".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopLeftRadius)
        );
        assert_ne!(
            Some("BorderTopRightRadius".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderTopRightRadius)
        );
        assert_ne!(
            Some("BoxDecorationBreak".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxDecorationBreak)
        );
        assert_ne!(
            Some("BoxShadow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxShadow)
        );
        assert_ne!(
            Some("Background".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Background)
        );
        assert_ne!(
            Some("BackgroundAttachment".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundAttachment)
        );
        assert_ne!(
            Some("BackgroundColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundColor)
        );
        assert_ne!(
            Some("BackgroundImage".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundImage)
        );
        assert_ne!(
            Some("BackgroundPosition".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundPosition)
        );
        assert_ne!(
            Some("BackgroundPositionX".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundPositionX)
        );
        assert_ne!(
            Some("BackgroundPositionY".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundPositionY)
        );
        assert_ne!(
            Some("BackgroundRepeat".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundRepeat)
        );
        assert_ne!(
            Some("BackgroundClip".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundClip)
        );
        assert_ne!(
            Some("BackgroundOrigin".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundOrigin)
        );
        assert_ne!(
            Some("BackgroundSize".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundSize)
        );
        assert_ne!(
            Some("BackgroundBlendMode".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackgroundBlendMode)
        );
        assert_ne!(
            Some("ColorProfile".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColorProfile)
        );
        assert_ne!(
            Some("Opacity".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Opacity)
        );
        assert_ne!(
            Some("RenderingIntent".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::RenderingIntent)
        );
        assert_ne!(
            Some("Font".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Font)
        );
        assert_ne!(
            Some("FontFamily".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontFamily)
        );
        assert_ne!(
            Some("FontSize".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontSize)
        );
        assert_ne!(
            Some("FontStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontStyle)
        );
        assert_ne!(
            Some("FontVariant".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontVariant)
        );
        assert_ne!(
            Some("FontWeight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontWeight)
        );
        assert_ne!(
            Some("FontSizeAdjust".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontSizeAdjust)
        );
        assert_ne!(
            Some("FontStretch".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FontStretch)
        );
        assert_ne!(
            Some("Positioning".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Positioning)
        );
        assert_ne!(
            Some("Bottom".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Bottom)
        );
        assert_ne!(
            Some("Clear".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Clear)
        );
        assert_ne!(
            Some("ClipPath".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ClipPath)
        );
        assert_ne!(
            Some("Cursor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Cursor)
        );
        assert_ne!(
            Some("Display".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Display)
        );
        assert_ne!(
            Some("Float".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Float)
        );
        assert_ne!(
            Some("Left".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Left)
        );
        assert_ne!(
            Some("Overflow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Overflow)
        );
        assert_ne!(
            Some("Position".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Position)
        );
        assert_ne!(
            Some("Right".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Right)
        );
        assert_ne!(
            Some("Top".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Top)
        );
        assert_ne!(
            Some("Visibility".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Visibility)
        );
        assert_ne!(
            Some("ZIndex".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ZIndex)
        );
        assert_ne!(
            Some("Color".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Color)
        );
        assert_ne!(
            Some("Direction".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Direction)
        );
        assert_ne!(
            Some("FlexDirection".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FlexDirection)
        );
        assert_ne!(
            Some("FlexWrap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::FlexWrap)
        );
        assert_ne!(
            Some("LetterSpacing".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LetterSpacing)
        );
        assert_ne!(
            Some("LineHeight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineHeight)
        );
        assert_ne!(
            Some("LineBreak".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineBreak)
        );
        assert_ne!(
            Some("TextAlign".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextAlign)
        );
        assert_ne!(
            Some("TextDecoration".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextDecoration)
        );
        assert_ne!(
            Some("TextIndent".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextIndent)
        );
        assert_ne!(
            Some("TextTransform".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextTransform)
        );
        assert_ne!(
            Some("UnicodeBidi".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::UnicodeBidi)
        );
        assert_ne!(
            Some("VerticalAlign".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VerticalAlign)
        );
        assert_ne!(
            Some("WhiteSpace".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WhiteSpace)
        );
        assert_ne!(
            Some("WordSpacing".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WordSpacing)
        );
        assert_ne!(
            Some("TextOutline".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextOutline)
        );
        assert_ne!(
            Some("TextOverflow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextOverflow)
        );
        assert_ne!(
            Some("TextShadow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextShadow)
        );
        assert_ne!(
            Some("TextWrap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextWrap)
        );
        assert_ne!(
            Some("WordBreak".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WordBreak)
        );
        assert_ne!(
            Some("WordWrap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WordWrap)
        );
        assert_ne!(
            Some("ListStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ListStyle)
        );
        assert_ne!(
            Some("ListStyleImage".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ListStyleImage)
        );
        assert_ne!(
            Some("ListStylePosition".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ListStylePosition)
        );
        assert_ne!(
            Some("ListStyleType".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ListStyleType)
        );
        assert_ne!(
            Some("BorderCollapse".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderCollapse)
        );
        assert_ne!(
            Some("BorderSpacing".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BorderSpacing)
        );
        assert_ne!(
            Some("CaptionSide".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::CaptionSide)
        );
        assert_ne!(
            Some("EmptyCells".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::EmptyCells)
        );
        assert_ne!(
            Some("TableLayout".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TableLayout)
        );
        assert_ne!(
            Some("MarqueeDirection".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarqueeDirection)
        );
        assert_ne!(
            Some("MarqueePlayCount".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarqueePlayCount)
        );
        assert_ne!(
            Some("MarqueeSpeed".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarqueeSpeed)
        );
        assert_ne!(
            Some("MarqueeStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarqueeStyle)
        );
        assert_ne!(
            Some("OverflowX".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OverflowX)
        );
        assert_ne!(
            Some("OverflowY".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OverflowY)
        );
        assert_ne!(
            Some("OverflowStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OverflowStyle)
        );
        assert_ne!(
            Some("Rotation".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Rotation)
        );
        assert_ne!(
            Some("BoxAlign".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxAlign)
        );
        assert_ne!(
            Some("BoxDirection".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxDirection)
        );
        assert_ne!(
            Some("BoxFlex".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxFlex)
        );
        assert_ne!(
            Some("BoxFlexGroup".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxFlexGroup)
        );
        assert_ne!(
            Some("BoxLines".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxLines)
        );
        assert_ne!(
            Some("BoxOrdinalGroup".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxOrdinalGroup)
        );
        assert_ne!(
            Some("BoxOrient".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxOrient)
        );
        assert_ne!(
            Some("BoxPack".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxPack)
        );
        assert_ne!(
            Some("AlignmentAdjust".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignmentAdjust)
        );
        assert_ne!(
            Some("AlignmentBaseline".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignmentBaseline)
        );
        assert_ne!(
            Some("BaselineShift".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BaselineShift)
        );
        assert_ne!(
            Some("DominantBaseline".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::DominantBaseline)
        );
        assert_ne!(
            Some("DropInitialAfterAdjust".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialAfterAdjust)
        );
        assert_ne!(
            Some("DropInitialAfterAlign".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialAfterAlign)
        );
        assert_ne!(
            Some("DropInitialBeforeAdjust".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialBeforeAdjust)
        );
        assert_ne!(
            Some("DropInitialBeforeAlign".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialBeforeAlign)
        );
        assert_ne!(
            Some("DropInitialSize".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialSize)
        );
        assert_ne!(
            Some("DropInitialValue".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::DropInitialValue)
        );
        assert_ne!(
            Some("InlineBoxAlign".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::InlineBoxAlign)
        );
        assert_ne!(
            Some("LineStacking".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineStacking)
        );
        assert_ne!(
            Some("LineStackingRuby".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineStackingRuby)
        );
        assert_ne!(
            Some("LineStackingShift".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineStackingShift)
        );
        assert_ne!(
            Some("LineStackingStrategy".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::LineStackingStrategy)
        );
        assert_ne!(
            Some("TextHeight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TextHeight)
        );
        assert_ne!(
            Some("ColumnCount".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnCount)
        );
        assert_ne!(
            Some("ColumnFill".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnFill)
        );
        assert_ne!(
            Some("ColumnGap".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnGap)
        );
        assert_ne!(
            Some("ColumnRule".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnRule)
        );
        assert_ne!(
            Some("ColumnRuleColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnRuleColor)
        );
        assert_ne!(
            Some("ColumnRuleStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnRuleStyle)
        );
        assert_ne!(
            Some("ColumnRuleWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnRuleWidth)
        );
        assert_ne!(
            Some("ColumnSpan".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnSpan)
        );
        assert_ne!(
            Some("ColumnWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ColumnWidth)
        );
        assert_ne!(
            Some("Columns".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Columns)
        );
        assert_ne!(
            Some("Animation".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Animation)
        );
        assert_ne!(
            Some("AnimationName".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationName)
        );
        assert_ne!(
            Some("AnimationDuration".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationDuration)
        );
        assert_ne!(
            Some("AnimationTimingFunction".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::AnimationTimingFunction)
        );
        assert_ne!(
            Some("AnimationDelay".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationDelay)
        );
        assert_ne!(
            Some("AnimationFillMode".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationFillMode)
        );
        assert_ne!(
            Some("AnimationIterationCount".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::AnimationIterationCount)
        );
        assert_ne!(
            Some("AnimationDirection".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationDirection)
        );
        assert_ne!(
            Some("AnimationPlayState".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AnimationPlayState)
        );
        assert_ne!(
            Some("Transform".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Transform)
        );
        assert_ne!(
            Some("TransformOrigin".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransformOrigin)
        );
        assert_ne!(
            Some("TransformStyle".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransformStyle)
        );
        assert_ne!(
            Some("Perspective".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Perspective)
        );
        assert_ne!(
            Some("PerspectiveOrigin".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PerspectiveOrigin)
        );
        assert_ne!(
            Some("BackfaceVisibility".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BackfaceVisibility)
        );
        assert_ne!(
            Some("Transition".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Transition)
        );
        assert_ne!(
            Some("TransitionProperty".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransitionProperty)
        );
        assert_ne!(
            Some("TransitionDuration".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransitionDuration)
        );
        assert_ne!(
            Some("TransitionTimingFunction".to_string()),
            nenyr_token
                .convert_nenyr_property_to_css_property(&NenyrTokens::TransitionTimingFunction)
        );
        assert_ne!(
            Some("TransitionDelay".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::TransitionDelay)
        );
        assert_ne!(
            Some("Orphans".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Orphans)
        );
        assert_ne!(
            Some("PageBreakAfter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PageBreakAfter)
        );
        assert_ne!(
            Some("PageBreakBefore".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PageBreakBefore)
        );
        assert_ne!(
            Some("PageBreakInside".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::PageBreakInside)
        );
        assert_ne!(
            Some("Widows".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Widows)
        );
        assert_ne!(
            Some("Mark".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Mark)
        );
        assert_ne!(
            Some("MarkAfter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarkAfter)
        );
        assert_ne!(
            Some("MarkBefore".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::MarkBefore)
        );
        assert_ne!(
            Some("Phonemes".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Phonemes)
        );
        assert_ne!(
            Some("Rest".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Rest)
        );
        assert_ne!(
            Some("RestAfter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::RestAfter)
        );
        assert_ne!(
            Some("RestBefore".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::RestBefore)
        );
        assert_ne!(
            Some("VoiceBalance".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceBalance)
        );
        assert_ne!(
            Some("VoiceDuration".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceDuration)
        );
        assert_ne!(
            Some("VoicePitch".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoicePitch)
        );
        assert_ne!(
            Some("VoicePitchRange".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoicePitchRange)
        );
        assert_ne!(
            Some("VoiceRate".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceRate)
        );
        assert_ne!(
            Some("VoiceStress".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceStress)
        );
        assert_ne!(
            Some("VoiceVolume".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::VoiceVolume)
        );
        assert_ne!(
            Some("Appearance".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Appearance)
        );
        assert_ne!(
            Some("BoxSizing".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::BoxSizing)
        );
        assert_ne!(
            Some("Icon".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Icon)
        );
        assert_ne!(
            Some("NavDown".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavDown)
        );
        assert_ne!(
            Some("NavIndex".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavIndex)
        );
        assert_ne!(
            Some("NavLeft".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavLeft)
        );
        assert_ne!(
            Some("NavRight".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavRight)
        );
        assert_ne!(
            Some("NavUp".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::NavUp)
        );
        assert_ne!(
            Some("OutlineOffset".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::OutlineOffset)
        );
        assert_ne!(
            Some("Resize".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Resize)
        );
        assert_ne!(
            Some("Quotes".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Quotes)
        );
        assert_ne!(
            Some("Rotate".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Rotate)
        );
        assert_ne!(
            Some("Translate".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Translate)
        );
        assert_ne!(
            Some("UserSelect".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::UserSelect)
        );
        assert_ne!(
            Some("WritingMode".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::WritingMode)
        );
        assert_ne!(
            Some("ObjectPosition".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ObjectPosition)
        );
        assert_ne!(
            Some("ObjectFit".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ObjectFit)
        );
        assert_ne!(
            Some("JustifySelf".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::JustifySelf)
        );
        assert_ne!(
            Some("JustifyContent".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::JustifyContent)
        );
        assert_ne!(
            Some("JustifyItems".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::JustifyItems)
        );
        assert_ne!(
            Some("AlignSelf".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignSelf)
        );
        assert_ne!(
            Some("AlignContent".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignContent)
        );
        assert_ne!(
            Some("AlignItems".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::AlignItems)
        );
        assert_ne!(
            Some("Grid".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::Grid)
        );
        assert_ne!(
            Some("GridArea".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridArea)
        );
        assert_ne!(
            Some("GridAutoColumns".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridAutoColumns)
        );
        assert_ne!(
            Some("GridAutoFlow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridAutoFlow)
        );
        assert_ne!(
            Some("GridAutoRows".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridAutoRows)
        );
        assert_ne!(
            Some("GridColumn".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridColumn)
        );
        assert_ne!(
            Some("GridColumnEnd".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridColumnEnd)
        );
        assert_ne!(
            Some("GridColumnStart".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridColumnStart)
        );
        assert_ne!(
            Some("GridRow".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridRow)
        );
        assert_ne!(
            Some("GridRowEnd".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridRowEnd)
        );
        assert_ne!(
            Some("GridRowStart".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridRowStart)
        );
        assert_ne!(
            Some("GridTemplate".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridTemplate)
        );
        assert_ne!(
            Some("GridTemplateAreas".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridTemplateAreas)
        );
        assert_ne!(
            Some("GridTemplateColumns".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridTemplateColumns)
        );
        assert_ne!(
            Some("GridTemplateRows".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::GridTemplateRows)
        );
        assert_ne!(
            Some("ScrollbarColor".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ScrollbarColor)
        );
        assert_ne!(
            Some("ScrollbarWidth".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ScrollbarWidth)
        );
        assert_ne!(
            Some("ScrollbarGutter".to_string()),
            nenyr_token.convert_nenyr_property_to_css_property(&NenyrTokens::ScrollbarGutter)
        );
    }
}
