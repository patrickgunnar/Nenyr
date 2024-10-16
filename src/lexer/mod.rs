use crate::{error::NenyrErrorTracing, tokens::NenyrTokens};

/// The `Lexer` struct is responsible for tokenizing the Nenyr language input.
/// It takes raw source code written in Nenyr and breaks it into tokens,
/// which are the smallest units of the language, such as keywords, identifiers,
/// symbols, numbers, and strings.
///
/// The `Lexer` operates by maintaining its current position in the input,
/// as well as tracking the current line and column for better error reporting
/// and debugging purposes. It handles both single-line and block comments,
/// skips over whitespace, and identifies various language constructs.
///
/// The lexer returns `NenyrTokens` for each valid token it identifies in the input.
/// If an unrecognized character or sequence is encountered, it returns
/// an `Unknown` token, and will eventually support more detailed error handling
/// for invalid input.
///
/// # Fields
///
/// * `raw_nenyr`: A reference to the raw source code in the Nenyr language.
///   This is a borrowed string slice (`&'a str`), and the lexer operates directly
///   on this input without making a copy, ensuring efficient performance on large inputs.
/// * `position`: The current byte position in the input string. This is used to
///   track progress through the input and retrieve the next character or token.
/// * `line`: The current line number in the input, used for error reporting and debugging.
///   This is incremented every time the lexer encounters a newline character (`'\n'`).
/// * `column`: The current column number in the current line, used for error
///   reporting and tracking. This is incremented for each character and reset when
///   a new line is encountered.
#[derive(Debug, PartialEq, Clone)]
pub struct Lexer<'a> {
    /// The raw input source written in Nenyr language, borrowed for the lifetime of the lexer.
    raw_nenyr: &'a str,
    /// The current position in the input string (in bytes), used to track which
    /// character the lexer is processing.
    position: usize,
    /// The current line number in the input, starts at 1 and increments with each newline.
    line: usize,
    /// The current column number within the current line, resets to 1 after each newline.
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Constructs a new `Lexer` from the provided raw input string in the Nenyr language.
    ///
    /// The lexer starts at the beginning of the input, with the position set to 0, and
    /// the line and column counters initialized to the first line and first column.
    ///
    /// # Parameters
    ///
    /// * `raw_nenyr`: A string slice representing the raw input source in the Nenyr language.
    ///
    /// # Returns
    ///
    /// A `Lexer` struct ready to tokenize the input string.
    pub fn new(raw_nenyr: &'a str) -> Self {
        Self {
            raw_nenyr,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Traces the lexer’s current line by retrieving the text of the line at the
    /// specified index in the input. This is useful for error reporting and debugging,
    /// providing the user with context about where an error occurred.
    ///
    /// # Parameters
    ///
    /// * `idx`: The index of the line to retrieve. This should be a 0-based index.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the line content if it exists, or `None`
    /// if the index is out of bounds.
    fn trace_lexer_line(&self, idx: usize) -> Option<String> {
        for (index, line) in self.raw_nenyr.lines().enumerate() {
            if index == idx {
                return Some(line.to_string());
            }
        }

        None
    }

    /// Provides detailed information about the lexer's current position in the input,
    /// including the lines of context around the current position, the current line,
    /// column, and overall byte position. This is used for constructing detailed
    /// error messages when issues arise during tokenization.
    ///
    /// # Returns
    ///
    /// A `NenyrErrorTracing` struct that contains the context around the current
    /// lexer position for debugging purposes.
    pub fn trace_lexer_position(&self) -> NenyrErrorTracing {
        NenyrErrorTracing::new(
            self.trace_lexer_line(self.position - 2),
            self.trace_lexer_line(self.position),
            self.trace_lexer_line(self.position - 1),
            self.line,
            self.column,
            self.position,
        )
    }

    /// Retrieves the current character in the input string without advancing
    /// the position of the lexer. This is useful for peeking at the next character
    /// to decide the appropriate action, such as tokenizing an identifier or operator.
    ///
    /// # Returns
    ///
    /// An `Option<char>` representing the current character. If the end of the input
    /// has been reached, this function returns `None`.
    pub fn current_char(&self) -> Option<char> {
        self.raw_nenyr[self.position..].chars().next()
    }

    /// Advances the lexer to the next token in the input. This function processes
    /// whitespace, comments, and symbols, returning the appropriate `NenyrTokens`
    /// for each type of token. When the end of the input is reached, an
    /// `EndOfLine` token is returned.
    ///
    /// # Returns
    ///
    /// A `NenyrTokens` enum representing the next token in the input stream. This could
    /// be a keyword, identifier, symbol, string literal, number, or other valid token.
    pub fn next_token(&mut self) -> NenyrTokens {
        while let Some(char) = self.current_char() {
            match char {
                // Skip whitespace and update position and column
                ' ' | '\t' => {
                    self.position += char.len_utf8();
                    self.column += char.len_utf8();
                }
                // Handle newlines
                '\n' => {
                    self.position += char.len_utf8();
                    self.line += 1;
                    self.column = 1;
                }
                // Handle carriage returns
                '\r' => {
                    // Check if followed by newline
                    if self.raw_nenyr[self.position + char.len_utf8()..].starts_with('\n') {
                        self.position += 2;
                    } else {
                        self.position += char.len_utf8();
                    }

                    self.line += 1;
                    self.column = 1;
                }
                // Handle comments
                '/' => {
                    self.position += char.len_utf8();
                    self.column += char.len_utf8();

                    // Check for line comment
                    if self.current_char() == Some('/') {
                        let slash_len = '/'.len_utf8();

                        self.position += slash_len;
                        self.column += slash_len;

                        self.skip_line_comment();

                        continue;

                    // Check for block comment
                    } else if self.current_char() == Some('*') {
                        let asterisk_len = '*'.len_utf8();

                        self.position += asterisk_len;
                        self.column += asterisk_len;

                        self.skip_block_comment();

                        continue;
                    }

                    return NenyrTokens::Unknown('/');
                }
                // Handle delimiters and symbols
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' => {
                    self.position += char.len_utf8();
                    self.column += char.len_utf8();

                    match char {
                        '(' => return NenyrTokens::ParenthesisOpen,
                        ')' => return NenyrTokens::ParenthesisClose,
                        '{' => return NenyrTokens::CurlyBracketOpen,
                        '}' => return NenyrTokens::CurlyBracketClose,
                        '[' => return NenyrTokens::SquareBracketOpen,
                        ']' => return NenyrTokens::SquareBracketClose,
                        ',' => return NenyrTokens::Comma,
                        ':' => return NenyrTokens::Colon,
                        _ => {
                            // TODO: Replace this with a NenyrError
                            unreachable!()
                        }
                    }
                }
                // Handle string literals
                '"' | '\'' => {
                    self.position += char.len_utf8();
                    self.column += char.len_utf8();

                    return self.parse_string_literal(char);
                }
                // Handle identifiers
                'a'..='z' | 'A'..='Z' => {
                    return self.parse_identifier();
                }
                // Handle numbers
                '0'..='9' => {
                    return self.parse_number();
                }
                // Handle unknown characters
                _ => {
                    self.position += char.len_utf8();
                    self.column += char.len_utf8();

                    return NenyrTokens::Unknown(char);
                }
            }
        }

        // Return EndOfFile token when the input is exhausted
        NenyrTokens::EndOfLine
    }

    /// Skips over a line comment in the raw input.
    ///
    /// A line comment starts with a specific marker (like `//`) and ends at the next newline (`\n`).
    /// This method updates the internal state of the parser to move past the entire comment, advancing
    /// both the position and the column, and adjusting the line number when a newline is encountered.
    fn skip_line_comment(&mut self) {
        while let Some(char) = self.current_char() {
            if char == '\n' {
                self.position += char.len_utf8();
                self.line += 1;
                self.column = 1;

                break;
            }

            self.position += char.len_utf8();
            self.column += char.len_utf8();
        }
    }

    /// Skips over a block comment in the raw input.
    ///
    /// Block comments are typically enclosed between a start marker (e.g., `/*`) and an end marker
    /// (e.g., `*/`). This method reads characters until it finds the closing marker, while updating
    /// the position, line, and column counters. It also correctly handles newlines within the comment.
    fn skip_block_comment(&mut self) {
        while let Some(char) = self.current_char() {
            if char == '*' && self.raw_nenyr[self.position + char.len_utf8()..].starts_with('/') {
                let current_char_plus_slash_len = char.len_utf8() + '/'.len_utf8();

                self.position += current_char_plus_slash_len;
                self.column += current_char_plus_slash_len;

                break;
            }

            if char == '\n' {
                self.position += char.len_utf8();
                self.line += 1;
                self.column = 1;
            } else {
                self.position += char.len_utf8();
                self.column += char.len_utf8();
            }
        }
    }

    /// Parses an identifier from the input and returns the corresponding token.
    ///
    /// An identifier is a sequence of alphanumeric characters. This method extracts such a sequence
    /// and then matches it against known Nenyr keywords (like "Construct" or "Central"). The position
    /// and column counters are advanced accordingly.
    ///
    /// # Returns
    ///
    /// * `NenyrTokens::Construct` for a recognized keyword.
    /// * A token representing the identifier if it's not a keyword.
    fn parse_identifier(&mut self) -> NenyrTokens {
        let start_pos = self.position;

        while let Some(char) = self.current_char() {
            if char.is_ascii_alphanumeric() {
                self.position += char.len_utf8();
                self.column += char.len_utf8();
            } else {
                break;
            }
        }

        let identifier = self.raw_nenyr[start_pos..self.position].to_string();

        self.match_identifier(identifier)
    }

    /// Parses a numeric literal from the input and returns a token representing the number.
    ///
    /// This method consumes a sequence of digits (0-9) or '.' from the input, interprets them as a float,
    /// and returns a `NenyrTokens::Number` token. The position and column counters are updated as
    /// characters are processed.
    ///
    /// # Panics
    ///
    /// This method will panic if the slice of digits cannot be parsed into a valid number.
    fn parse_number(&mut self) -> NenyrTokens {
        let start_pos = self.position;

        while let Some(char) = self.current_char() {
            if char.is_digit(10) || char == '.' {
                self.position += char.len_utf8();
                self.column += char.len_utf8();
            } else {
                break;
            }
        }

        let value = self.raw_nenyr[start_pos..self.position].parse().unwrap();

        NenyrTokens::Number(value)
    }

    /// Parses a string literal from the input, delimited by a given character.
    ///
    /// This method starts at the current position and consumes characters until it encounters the same
    /// delimiter character (like `"` or `'`) that opened the string. It returns a `NenyrTokens::StringLiteral`
    /// token containing the extracted string (excluding the delimiters). The position and column are
    /// updated accordingly.
    ///
    /// # Parameters
    ///
    /// * `entered_char` - The character that opened the string literal (e.g., `"` or `'`).
    ///
    /// # Returns
    ///
    /// A `NenyrTokens::StringLiteral` token containing the parsed string.
    fn parse_string_literal(&mut self, entered_char: char) -> NenyrTokens {
        let start_pos = self.position;

        while let Some(char) = self.current_char() {
            self.position += char.len_utf8();
            self.column += char.len_utf8();

            if char == entered_char {
                break;
            }
        }

        let value = self.raw_nenyr[start_pos..(self.position - 1)].to_string();

        NenyrTokens::StringLiteral(value)
    }

    /// Matches a given identifier against predefined Nenyr keywords and returns the corresponding token.
    ///
    /// This method attempts to match an identifier string to a set of known keywords used within the Nenyr DSL
    /// (e.g., "Construct", "Central"). If the identifier matches one of these keywords, a corresponding
    /// `NenyrTokens` variant (e.g., `NenyrTokens::Construct`) is returned. Otherwise, if the identifier is not
    /// recognized as a keyword, it is treated as a generic identifier and returned as `NenyrTokens::Identifier(String)`.
    ///
    /// This enables the lexer to differentiate between reserved words and user-defined identifiers during parsing.
    ///
    /// # Parameters
    ///
    /// * `identifier` - A `String` representing the identifier to be matched against known Nenyr keywords.
    ///
    /// # Returns
    ///
    /// * `NenyrTokens::Construct` if the identifier matches the keyword "Construct".
    /// * `NenyrTokens::Central` if the identifier matches the keyword "Central".
    /// * `NenyrTokens::Identifier(String)` if the identifier does not match any predefined keywords, where `String` contains the original identifier.
    fn match_identifier(&self, identifier: String) -> NenyrTokens {
        match identifier.as_str() {
            // Nenyr keywords
            "Construct" => NenyrTokens::Construct,
            "Central" => NenyrTokens::Central,
            "Layout" => NenyrTokens::Layout,
            "Module" => NenyrTokens::Module,
            "Declare" => NenyrTokens::Declare,
            "Extending" => NenyrTokens::Extending,
            "Deriving" => NenyrTokens::Deriving,

            // Nenyr methods
            "Imports" => NenyrTokens::Imports,
            "Typefaces" => NenyrTokens::Typefaces,
            "Breakpoints" => NenyrTokens::Breakpoints,
            "Themes" => NenyrTokens::Themes,
            "Aliases" => NenyrTokens::Aliases,
            "Variables" => NenyrTokens::Variables,
            "Class" => NenyrTokens::Class,

            // Import pattern
            "Import" => NenyrTokens::Import,

            // Breakpoints pattern
            "MobileFirst" => NenyrTokens::MobileFirst,
            "DesktopFirst" => NenyrTokens::DesktopFirst,

            // Themes pattern
            "Light" => NenyrTokens::Light,
            "Dark" => NenyrTokens::Dark,

            // Animation pattern
            "Fraction" => NenyrTokens::Fraction,
            "Progressive" => NenyrTokens::Progressive,
            "From" => NenyrTokens::From,
            "Halfway" => NenyrTokens::Halfway,
            "To" => NenyrTokens::To,

            // Syntax tokens
            "true" => NenyrTokens::True,
            "false" => NenyrTokens::False,

            // Nenyr style patterns
            "Stylesheet" => NenyrTokens::Stylesheet,
            "PanoramicViewer" => NenyrTokens::PanoramicViewer,
            "Hover" => NenyrTokens::Hover,
            "Active" => NenyrTokens::Active,
            "Focus" => NenyrTokens::Focus,
            "FirstChild" => NenyrTokens::FirstChild,
            "LastChild" => NenyrTokens::LastChild,
            "FirstOfType" => NenyrTokens::FirstOfType,
            "LastOfType" => NenyrTokens::LastOfType,
            "OnlyChild" => NenyrTokens::OnlyChild,
            "OnlyOfType" => NenyrTokens::OnlyOfType,
            "Target" => NenyrTokens::Target,
            "Visited" => NenyrTokens::Visited,
            "Checked" => NenyrTokens::Checked,
            "Disabled" => NenyrTokens::Disabled,
            "Enabled" => NenyrTokens::Enabled,
            "ReadOnly" => NenyrTokens::ReadOnly,
            "ReadWrite" => NenyrTokens::ReadWrite,
            "PlaceholderShown" => NenyrTokens::PlaceholderShown,
            "Valid" => NenyrTokens::Valid,
            "Invalid" => NenyrTokens::Invalid,
            "Required" => NenyrTokens::Required,
            "Optional" => NenyrTokens::Optional,
            "Fullscreen" => NenyrTokens::Fullscreen,
            "FocusWithin" => NenyrTokens::FocusWithin,
            "FirstLine" => NenyrTokens::FirstLine,
            "FirstLetter" => NenyrTokens::FirstLetter,
            "Before" => NenyrTokens::Before,
            "After" => NenyrTokens::After,
            "OutOfRange" => NenyrTokens::OutOfRange,
            "Root" => NenyrTokens::Root,
            "Empty" => NenyrTokens::Empty,

            // Nenyr properties group
            "aspectRatio" => NenyrTokens::AspectRatio,
            "accentColor" => NenyrTokens::AccentColor,
            "backdropFilter" => NenyrTokens::BackdropFilter,
            "content" => NenyrTokens::Content,
            "gap" => NenyrTokens::Gap,
            "rowGap" => NenyrTokens::RowGap,
            "scale" => NenyrTokens::Scale,
            "order" => NenyrTokens::Order,
            "pointerEvents" => NenyrTokens::PointerEvents,
            "margin" => NenyrTokens::Margin,
            "marginBottom" => NenyrTokens::MarginBottom,
            "marginLeft" => NenyrTokens::MarginLeft,
            "marginRight" => NenyrTokens::MarginRight,
            "marginTop" => NenyrTokens::MarginTop,
            "padding" => NenyrTokens::Padding,
            "paddingBottom" => NenyrTokens::PaddingBottom,
            "paddingLeft" => NenyrTokens::PaddingLeft,
            "paddingRight" => NenyrTokens::PaddingRight,
            "paddingTop" => NenyrTokens::PaddingTop,
            "height" => NenyrTokens::Height,
            "width" => NenyrTokens::Width,
            "filter" => NenyrTokens::Filter,
            "maxHeight" => NenyrTokens::MaxHeight,
            "maxWidth" => NenyrTokens::MaxWidth,
            "minHeight" => NenyrTokens::MinHeight,
            "minWidth" => NenyrTokens::MinWidth,
            "border" => NenyrTokens::Border,
            "borderBottom" => NenyrTokens::BorderBottom,
            "borderBottomColor" => NenyrTokens::BorderBottomColor,
            "borderBottomStyle" => NenyrTokens::BorderBottomStyle,
            "borderBottomWidth" => NenyrTokens::BorderBottomWidth,
            "borderColor" => NenyrTokens::BorderColor,
            "borderLeft" => NenyrTokens::BorderLeft,
            "borderLeftColor" => NenyrTokens::BorderLeftColor,
            "borderLeftStyle" => NenyrTokens::BorderLeftStyle,
            "borderLeftWidth" => NenyrTokens::BorderLeftWidth,
            "borderRight" => NenyrTokens::BorderRight,
            "borderRightColor" => NenyrTokens::BorderRightColor,
            "borderRightStyles" => NenyrTokens::BorderRightStyles,
            "borderRightWidth" => NenyrTokens::BorderRightWidth,
            "borderStyle" => NenyrTokens::BorderStyle,
            "borderTop" => NenyrTokens::BorderTop,
            "borderTopColor" => NenyrTokens::BorderTopColor,
            "borderTopStyle" => NenyrTokens::BorderTopStyle,
            "borderTopWidth" => NenyrTokens::BorderTopWidth,
            "borderWidth" => NenyrTokens::BorderWidth,
            "outline" => NenyrTokens::Outline,
            "outlineColor" => NenyrTokens::OutlineColor,
            "outlineStyle" => NenyrTokens::OutlineStyle,
            "outlineWidth" => NenyrTokens::OutlineWidth,
            "borderBottomLeftRadius" => NenyrTokens::BorderBottomLeftRadius,
            "borderBottomRightRadius" => NenyrTokens::BorderBottomRightRadius,
            "borderImage" => NenyrTokens::BorderImage,
            "borderImageOutset" => NenyrTokens::BorderImageOutset,
            "borderImageRepeat" => NenyrTokens::BorderImageRepeat,
            "borderImageSlice" => NenyrTokens::BorderImageSlice,
            "borderImageSource" => NenyrTokens::BorderImageSource,
            "borderImageWidth" => NenyrTokens::BorderImageWidth,
            "borderRadius" => NenyrTokens::BorderRadius,
            "borderTopLeftRadius" => NenyrTokens::BorderTopLeftRadius,
            "borderTopRightRadius" => NenyrTokens::BorderTopRightRadius,
            "boxDecorationBreak" => NenyrTokens::BoxDecorationBreak,
            "boxShadow" => NenyrTokens::BoxShadow,
            "background" => NenyrTokens::Background,
            "backgroundAttachment" => NenyrTokens::BackgroundAttachment,
            "backgroundColor" => NenyrTokens::BackgroundColor,
            "backgroundImage" => NenyrTokens::BackgroundImage,
            "backgroundPosition" => NenyrTokens::BackgroundPosition,
            "backgroundPositionX" => NenyrTokens::BackgroundPositionX,
            "backgroundPositionY" => NenyrTokens::BackgroundPositionY,
            "backgroundRepeat" => NenyrTokens::BackgroundRepeat,
            "backgroundClip" => NenyrTokens::BackgroundClip,
            "backgroundOrigin" => NenyrTokens::BackgroundOrigin,
            "backgroundSize" => NenyrTokens::BackgroundSize,
            "backgroundBlendMode" => NenyrTokens::BackgroundBlendMode,
            "colorProfile" => NenyrTokens::ColorProfile,
            "opacity" => NenyrTokens::Opacity,
            "renderingIntent" => NenyrTokens::RenderingIntent,
            "font" => NenyrTokens::Font,
            "fontFamily" => NenyrTokens::FontFamily,
            "fontSize" => NenyrTokens::FontSize,
            "fontStyle" => NenyrTokens::FontStyle,
            "fontVariant" => NenyrTokens::FontVariant,
            "fontWeight" => NenyrTokens::FontWeight,
            "fontSizeAdjust" => NenyrTokens::FontSizeAdjust,
            "fontStretch" => NenyrTokens::FontStretch,
            "positioning" => NenyrTokens::Positioning,
            "bottom" => NenyrTokens::Bottom,
            "clear" => NenyrTokens::Clear,
            "clipPath" => NenyrTokens::ClipPath,
            "cursor" => NenyrTokens::Cursor,
            "display" => NenyrTokens::Display,
            "float" => NenyrTokens::Float,
            "left" => NenyrTokens::Left,
            "overflow" => NenyrTokens::Overflow,
            "position" => NenyrTokens::Position,
            "right" => NenyrTokens::Right,
            "top" => NenyrTokens::Top,
            "visibility" => NenyrTokens::Visibility,
            "zIndex" => NenyrTokens::ZIndex,
            "color" => NenyrTokens::Color,
            "direction" => NenyrTokens::Direction,
            "flexDirection" => NenyrTokens::FlexDirection,
            "flexWrap" => NenyrTokens::FlexWrap,
            "letterSpacing" => NenyrTokens::LetterSpacing,
            "lineHeight" => NenyrTokens::LineHeight,
            "lineBreak" => NenyrTokens::LineBreak,
            "textAlign" => NenyrTokens::TextAlign,
            "textDecoration" => NenyrTokens::TextDecoration,
            "textIndent" => NenyrTokens::TextIndent,
            "textTransform" => NenyrTokens::TextTransform,
            "unicodeBidi" => NenyrTokens::UnicodeBidi,
            "verticalAlign" => NenyrTokens::VerticalAlign,
            "whiteSpace" => NenyrTokens::WhiteSpace,
            "wordSpacing" => NenyrTokens::WordSpacing,
            "textOutline" => NenyrTokens::TextOutline,
            "textOverflow" => NenyrTokens::TextOverflow,
            "textShadow" => NenyrTokens::TextShadow,
            "textWrap" => NenyrTokens::TextWrap,
            "wordBreak" => NenyrTokens::WordBreak,
            "wordWrap" => NenyrTokens::WordWrap,
            "listStyle" => NenyrTokens::ListStyle,
            "listStyleImage" => NenyrTokens::ListStyleImage,
            "listStylePosition" => NenyrTokens::ListStylePosition,
            "listStyleType" => NenyrTokens::ListStyleType,
            "borderCollapse" => NenyrTokens::BorderCollapse,
            "borderSpacing" => NenyrTokens::BorderSpacing,
            "captionSide" => NenyrTokens::CaptionSide,
            "emptyCells" => NenyrTokens::EmptyCells,
            "tableLayout" => NenyrTokens::TableLayout,
            "marqueeDirection" => NenyrTokens::MarqueeDirection,
            "marqueePlayCount" => NenyrTokens::MarqueePlayCount,
            "marqueeSpeed" => NenyrTokens::MarqueeSpeed,
            "marqueeStyle" => NenyrTokens::MarqueeStyle,
            "overflowX" => NenyrTokens::OverflowX,
            "overflowY" => NenyrTokens::OverflowY,
            "overflowStyle" => NenyrTokens::OverflowStyle,
            "rotation" => NenyrTokens::Rotation,
            "boxAlign" => NenyrTokens::BoxAlign,
            "boxDirection" => NenyrTokens::BoxDirection,
            "boxFlex" => NenyrTokens::BoxFlex,
            "boxFlexGroup" => NenyrTokens::BoxFlexGroup,
            "boxLines" => NenyrTokens::BoxLines,
            "boxOrdinalGroup" => NenyrTokens::BoxOrdinalGroup,
            "boxOrient" => NenyrTokens::BoxOrient,
            "boxPack" => NenyrTokens::BoxPack,
            "alignmentAdjust" => NenyrTokens::AlignmentAdjust,
            "alignmentBaseline" => NenyrTokens::AlignmentBaseline,
            "baselineShift" => NenyrTokens::BaselineShift,
            "dominantBaseline" => NenyrTokens::DominantBaseline,
            "dropInitialAfterAdjust" => NenyrTokens::DropInitialAfterAdjust,
            "dropInitialAfterAlign" => NenyrTokens::DropInitialAfterAlign,
            "dropInitialBeforeAdjust" => NenyrTokens::DropInitialBeforeAdjust,
            "dropInitialBeforeAlign" => NenyrTokens::DropInitialBeforeAlign,
            "dropInitialSize" => NenyrTokens::DropInitialSize,
            "dropInitialValue" => NenyrTokens::DropInitialValue,
            "inlineBoxAlign" => NenyrTokens::InlineBoxAlign,
            "lineStacking" => NenyrTokens::LineStacking,
            "lineStackingRuby" => NenyrTokens::LineStackingRuby,
            "lineStackingShift" => NenyrTokens::LineStackingShift,
            "lineStackingStrategy" => NenyrTokens::LineStackingStrategy,
            "textHeight" => NenyrTokens::TextHeight,
            "columnCount" => NenyrTokens::ColumnCount,
            "columnFill" => NenyrTokens::ColumnFill,
            "columnGap" => NenyrTokens::ColumnGap,
            "columnRule" => NenyrTokens::ColumnRule,
            "columnRuleColor" => NenyrTokens::ColumnRuleColor,
            "columnRuleStyle" => NenyrTokens::ColumnRuleStyle,
            "columnRuleWidth" => NenyrTokens::ColumnRuleWidth,
            "columnSpan" => NenyrTokens::ColumnSpan,
            "columnWidth" => NenyrTokens::ColumnWidth,
            "columns" => NenyrTokens::Columns,
            "animation" => NenyrTokens::Animation,
            "animationName" => NenyrTokens::AnimationName,
            "animationDuration" => NenyrTokens::AnimationDuration,
            "animationTimingFunction" => NenyrTokens::AnimationTimingFunction,
            "animationDelay" => NenyrTokens::AnimationDelay,
            "animationFillMode" => NenyrTokens::AnimationFillMode,
            "animationIterationCount" => NenyrTokens::AnimationIterationCount,
            "animationDirection" => NenyrTokens::AnimationDirection,
            "animationPlayState" => NenyrTokens::AnimationPlayState,
            "transform" => NenyrTokens::Transform,
            "transformOrigin" => NenyrTokens::TransformOrigin,
            "transformStyle" => NenyrTokens::TransformStyle,
            "perspective" => NenyrTokens::Perspective,
            "perspectiveOrigin" => NenyrTokens::PerspectiveOrigin,
            "backfaceVisibility" => NenyrTokens::BackfaceVisibility,
            "transition" => NenyrTokens::Transition,
            "transitionProperty" => NenyrTokens::TransitionProperty,
            "transitionDuration" => NenyrTokens::TransitionDuration,
            "transitionTimingFunction" => NenyrTokens::TransitionTimingFunction,
            "transitionDelay" => NenyrTokens::TransitionDelay,
            "orphans" => NenyrTokens::Orphans,
            "pageBreakAfter" => NenyrTokens::PageBreakAfter,
            "pageBreakBefore" => NenyrTokens::PageBreakBefore,
            "pageBreakInside" => NenyrTokens::PageBreakInside,
            "widows" => NenyrTokens::Widows,
            "mark" => NenyrTokens::Mark,
            "markAfter" => NenyrTokens::MarkAfter,
            "markBefore" => NenyrTokens::MarkBefore,
            "phonemes" => NenyrTokens::Phonemes,
            "rest" => NenyrTokens::Rest,
            "restAfter" => NenyrTokens::RestAfter,
            "restBefore" => NenyrTokens::RestBefore,
            "voiceBalance" => NenyrTokens::VoiceBalance,
            "voiceDuration" => NenyrTokens::VoiceDuration,
            "voicePitch" => NenyrTokens::VoicePitch,
            "voicePitchRange" => NenyrTokens::VoicePitchRange,
            "voiceRate" => NenyrTokens::VoiceRate,
            "voiceStress" => NenyrTokens::VoiceStress,
            "voiceVolume" => NenyrTokens::VoiceVolume,
            "appearance" => NenyrTokens::Appearance,
            "boxSizing" => NenyrTokens::BoxSizing,
            "icon" => NenyrTokens::Icon,
            "navDown" => NenyrTokens::NavDown,
            "navIndex" => NenyrTokens::NavIndex,
            "navLeft" => NenyrTokens::NavLeft,
            "navRight" => NenyrTokens::NavRight,
            "navUp" => NenyrTokens::NavUp,
            "outlineOffset" => NenyrTokens::OutlineOffset,
            "resize" => NenyrTokens::Resize,
            "quotes" => NenyrTokens::Quotes,
            "rotate" => NenyrTokens::Rotate,
            "translate" => NenyrTokens::Translate,
            "userSelect" => NenyrTokens::UserSelect,
            "writingMode" => NenyrTokens::WritingMode,
            "objectPosition" => NenyrTokens::ObjectPosition,
            "objectFit" => NenyrTokens::ObjectFit,
            "justifySelf" => NenyrTokens::JustifySelf,
            "justifyContent" => NenyrTokens::JustifyContent,
            "justifyItems" => NenyrTokens::JustifyItems,
            "alignSelf" => NenyrTokens::AlignSelf,
            "alignContent" => NenyrTokens::AlignContent,
            "alignItems" => NenyrTokens::AlignItems,
            "grid" => NenyrTokens::Grid,
            "gridArea" => NenyrTokens::GridArea,
            "gridAutoColumns" => NenyrTokens::GridAutoColumns,
            "gridAutoFlow" => NenyrTokens::GridAutoFlow,
            "gridAutoRows" => NenyrTokens::GridAutoRows,
            "gridColumn" => NenyrTokens::GridColumn,
            "gridColumnEnd" => NenyrTokens::GridColumnEnd,
            "gridColumnStart" => NenyrTokens::GridColumnStart,
            "gridRow" => NenyrTokens::GridRow,
            "gridRowEnd" => NenyrTokens::GridRowEnd,
            "gridRowStart" => NenyrTokens::GridRowStart,
            "gridTemplate" => NenyrTokens::GridTemplate,
            "gridTemplateAreas" => NenyrTokens::GridTemplateAreas,
            "gridTemplateColumns" => NenyrTokens::GridTemplateColumns,
            "gridTemplateRows" => NenyrTokens::GridTemplateRows,
            "scrollbarColor" => NenyrTokens::ScrollbarColor,
            "scrollbarWidth" => NenyrTokens::ScrollbarWidth,
            "scrollbarGutter" => NenyrTokens::ScrollbarGutter,

            // That's means that the received identifier is not a token,
            // then return it as an Identifier.
            _ => NenyrTokens::Identifier(identifier),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = "";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), NenyrTokens::EndOfLine);
    }

    #[test]
    fn test_single_token() {
        let input = "(";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), NenyrTokens::ParenthesisOpen);
        assert_eq!(lexer.next_token(), NenyrTokens::EndOfLine);
    }

    #[test]
    fn test_multiple_tokens() {
        let input = "( ) { }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), NenyrTokens::ParenthesisOpen);
        assert_eq!(lexer.next_token(), NenyrTokens::ParenthesisClose);
        assert_eq!(lexer.next_token(), NenyrTokens::CurlyBracketOpen);
        assert_eq!(lexer.next_token(), NenyrTokens::CurlyBracketClose);
        assert_eq!(lexer.next_token(), NenyrTokens::EndOfLine);
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "   ( )   ";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), NenyrTokens::ParenthesisOpen);
        assert_eq!(lexer.next_token(), NenyrTokens::ParenthesisClose);
        assert_eq!(lexer.next_token(), NenyrTokens::EndOfLine);
    }

    #[test]
    fn test_string_literal() {
        let input = "\"hello\"";
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next_token(),
            NenyrTokens::StringLiteral("hello".to_string())
        );
        assert_eq!(lexer.next_token(), NenyrTokens::EndOfLine);
    }

    #[test]
    fn test_comments() {
        let input = "// this is a comment\n( )";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), NenyrTokens::ParenthesisOpen);
        assert_eq!(lexer.next_token(), NenyrTokens::ParenthesisClose);
        assert_eq!(lexer.next_token(), NenyrTokens::EndOfLine);
    }

    #[test]
    fn test_identifier() {
        let input = "Construct";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), NenyrTokens::Construct);
        assert_eq!(lexer.next_token(), NenyrTokens::EndOfLine);
    }

    #[test]
    fn test_number() {
        let input = "123";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), NenyrTokens::Number(123));
        assert_eq!(lexer.next_token(), NenyrTokens::EndOfLine);
    }

    #[test]
    fn test_unknown_token() {
        let input = "@";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), NenyrTokens::Unknown('@'));
    }
}
