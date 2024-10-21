use crate::{
    error::{NenyrError, NenyrErrorKind, NenyrErrorTracing},
    tokens::NenyrTokens,
    NenyrResult,
};

/// The `Lexer` struct is responsible for tokenizing input written in the Nenyr language.
/// It takes raw source code and breaks it into tokens, which are the smallest units of the language,
/// such as keywords, identifiers, symbols, numbers, and strings.
///
/// The `Lexer` maintains its current position in the input while tracking the current line and column
/// for improved error reporting and debugging. It can handle both single-line and block comments,
/// skips whitespace, and identifies various language constructs.
///
/// For each valid token identified in the input, the lexer returns a `NenyrTokens` instance.
/// If it encounters an unrecognized character or sequence, it returns an `Unknown` token,
/// with plans to implement more detailed error handling for invalid inputs in the future.
///
/// # Fields
///
/// * `raw_nenyr`: A reference to the raw source code written in the Nenyr language.
///   This is a borrowed string slice (`&'a str`), enabling the lexer to operate directly
///   on the input without making a copy, which ensures efficient performance even with large inputs.
/// * `position`: The current byte position in the input string, used to track progress
///   through the input and retrieve the next character or token.
/// * `line`: The current line number in the input, starting at 1 and incremented with each newline character (`'\n'`).
/// * `column`: The current column number in the current line, used for error reporting.
///   It increments for each character processed and resets to 1 when a newline is encountered.
/// * `context_path`: A reference to a string representing the Nenyr context file path,
///   which may provide additional information about the source code's origin.
/// * `context_name`: An optional `String` representing the name of the Nenyr context,
///   which can be useful for distinguishing between different scopes or modules within the Nenyr document.
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
    /// The context path for the Nenyr context, providing additional information about the source's origin.
    context_path: &'a str,
    /// An optional name of the context, useful for distinguishing between different scopes or modules in the Nenyr document.
    context_name: Option<String>,
}

impl<'a> Lexer<'a> {
    /// Constructs a new `Lexer` instance from the provided raw input string in the Nenyr language.
    ///
    /// The lexer initializes its state at the beginning of the input, setting the position to 0,
    /// and initializing the line and column counters to represent the first line and first column.
    ///
    /// # Parameters
    ///
    /// * `raw_nenyr`: A string slice (`&'a str`) representing the raw input source in the Nenyr language.
    /// * `context_path`: A string slice (`&'a str`) indicating the context path, which may provide additional
    ///   information regarding the source's origin.
    ///
    /// # Returns
    ///
    /// Returns a `Lexer` struct that is ready to tokenize the provided input string.
    pub fn new(raw_nenyr: &'a str, context_path: &'a str) -> Self {
        Self {
            raw_nenyr,
            context_path,
            position: 0,
            line: 1,
            column: 1,
            context_name: None,
        }
    }

    /// Sets the name of the Nenyr context.
    ///
    /// This method allows updating the `context_name` field with a new value, which can be useful for
    /// distinguishing between different scopes or modules within the Nenyr source code.
    ///
    /// # Parameters
    ///
    /// * `context_name`: An `Option<String>` that represents the name of the context. If `None` is provided,
    ///   the context name will be cleared.
    pub fn set_context_name(&mut self, context_name: Option<String>) {
        self.context_name = context_name;
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
        let line_before = if let Some(idx) = self.line.checked_sub(2) {
            self.trace_lexer_line(idx)
        } else {
            None
        };

        let error_line = if let Some(idx) = self.line.checked_sub(1) {
            self.trace_lexer_line(idx)
        } else {
            None
        };

        NenyrErrorTracing::new(
            line_before,
            self.trace_lexer_line(self.line),
            error_line,
            self.line,
            self.column,
            self.position,
        )
    }

    /// Raises an error when an unknown or invalid token is encountered during lexing.
    ///
    /// This method generates a `NenyrError` when the lexer detects an unknown token
    /// or an invalid character that doesn't match any expected patterns. The error
    /// contains contextual information, such as the name of the current context,
    /// the file path being processed, the type of error, and a trace of the lexer's
    /// position to help pinpoint where the error occurred.
    ///
    /// This method is generally called when the lexer is unable to process certain
    /// characters in the input stream, indicating a syntax issue in the source code.
    fn raise_unknown_token_error(&self, unknown_token: char) -> NenyrError {
        NenyrError::new(
            Some(format!("To resolve the error, please remove the unsupported token `{}` from your Nenyr code and revalidate. Ensure all tokens comply with Nenyr syntax to avoid future issues.", unknown_token)),
            self.context_name.to_owned(),
            self.context_path.to_string(),
            format!("The current token `{}` is not supported within Nenyr syntax. Please verify the token and ensure it adheres to the Nenyr language rules.", unknown_token),
            NenyrErrorKind::SyntaxError,
            self.trace_lexer_position(),
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
    /// whitespace, comments, delimiters, symbols, and string literals, returning
    /// the appropriate `NenyrTokens` for each type of token. If an unknown token
    /// is encountered, the function returns a `NenyrError`.
    ///
    /// The function also handles line comments (`//`) and block comments (`/* */`).
    /// For unknown tokens, the lexer raises an error containing information about
    /// the token's location, allowing for more precise debugging.
    ///
    /// # Parameters
    ///
    /// - `context_name`: An optional `String` representing the name of the context
    ///    in which this lexer is operating (useful for error reporting).
    /// - `context_path`: A `String` representing the current path or file being lexed.
    ///
    /// # Returns
    ///
    /// - `Ok(NenyrTokens)`: A `NenyrTokens` enum representing the next valid token in the input stream.
    /// - `Err(NenyrError)`: An error if an unknown or invalid token is encountered.
    ///
    /// This could be a keyword, identifier, symbol, string literal, number, or any
    /// other valid token. When the end of the input is reached, an `EndOfLine` token
    /// is returned.
    ///
    /// # Errors
    ///
    /// - Returns `Err(NenyrError)` if an unknown token is encountered, containing
    ///   details such as the line, column, and the problematic character.
    pub fn next_token(&mut self) -> NenyrResult<NenyrTokens> {
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

                    return Err(self.raise_unknown_token_error('/'));
                }
                // Handle delimiters and symbols
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' => {
                    self.position += char.len_utf8();
                    self.column += char.len_utf8();

                    match char {
                        '(' => return Ok(NenyrTokens::ParenthesisOpen),
                        ')' => return Ok(NenyrTokens::ParenthesisClose),
                        '{' => return Ok(NenyrTokens::CurlyBracketOpen),
                        '}' => return Ok(NenyrTokens::CurlyBracketClose),
                        '[' => return Ok(NenyrTokens::SquareBracketOpen),
                        ']' => return Ok(NenyrTokens::SquareBracketClose),
                        ',' => return Ok(NenyrTokens::Comma),
                        ':' => return Ok(NenyrTokens::Colon),
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

                    return Ok(self.parse_string_literal(char));
                }
                // Handle identifiers
                'a'..='z' | 'A'..='Z' => {
                    return Ok(self.parse_identifier());
                }
                // Handle numbers
                '0'..='9' => {
                    return Ok(self.parse_number());
                }
                // Handle unknown characters
                _ => {
                    self.position += char.len_utf8();
                    self.column += char.len_utf8();

                    return Err(self.raise_unknown_token_error(char));
                }
            }
        }

        // Return EndOfFile token when the input is exhausted
        Ok(NenyrTokens::EndOfLine)
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
            "Important" => NenyrTokens::Important,
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
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::EndOfLine));
    }

    #[test]
    fn test_single_token() {
        let input = "(";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::ParenthesisOpen));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::EndOfLine));
    }

    #[test]
    fn test_multiple_tokens() {
        let input = "( ) { }";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::ParenthesisOpen));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::ParenthesisClose));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::CurlyBracketOpen));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::CurlyBracketClose));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::EndOfLine));
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "   ( )   ";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::ParenthesisOpen));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::ParenthesisClose));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::EndOfLine));
    }

    #[test]
    fn test_string_literal() {
        let input = "\"hello\"";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(
            lexer.next_token(),
            Ok(NenyrTokens::StringLiteral("hello".to_string()))
        );
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::EndOfLine));
    }

    #[test]
    fn test_comments() {
        let input = "// this is a comment\n( )";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::ParenthesisOpen));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::ParenthesisClose));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::EndOfLine));
    }

    #[test]
    fn test_identifier() {
        let input = "Construct";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::Construct));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::EndOfLine));
    }

    #[test]
    fn test_number() {
        let input = "123";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::Number(123)));
        assert_eq!(lexer.next_token(), Ok(NenyrTokens::EndOfLine));
    }

    #[test]
    fn test_unknown_token() {
        let input = "@";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(
            lexer.next_token(),
            Err(NenyrError {
                suggestion: Some("To resolve the error, please remove the unsupported token `@` from your Nenyr code and revalidate. Ensure all tokens comply with Nenyr syntax to avoid future issues.".to_string()),
                context_name: None,
                context_path: "".to_string(),
                error_message: "The current token `@` is not supported within Nenyr syntax. Please verify the token and ensure it adheres to the Nenyr language rules.".to_string(),
                error_kind: NenyrErrorKind::SyntaxError,
                error_tracing: NenyrErrorTracing {
                    line_before: None,
                    line_after: None,
                    error_line: Some("@".to_string()),
                    error_on_line: 1,
                    error_on_col: 2,
                    error_on_pos: 1
                }
            })
        );
    }

    #[test]
    fn test_unknown_token_before_success() {
        let input = "@ Declare Aliases({}),\nDeclare";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(
            lexer.next_token(),
            Err(NenyrError {
                suggestion: Some("To resolve the error, please remove the unsupported token `@` from your Nenyr code and revalidate. Ensure all tokens comply with Nenyr syntax to avoid future issues.".to_string()),
                context_name: None,
                context_path: "".to_string(),
                error_message: "The current token `@` is not supported within Nenyr syntax. Please verify the token and ensure it adheres to the Nenyr language rules.".to_string(),
                error_kind: NenyrErrorKind::SyntaxError,
                error_tracing: NenyrErrorTracing {
                    line_before: None,
                    line_after: Some("Declare".to_string()),
                    error_line: Some("@ Declare Aliases({}),".to_string()),
                    error_on_line: 1,
                    error_on_col: 2,
                    error_on_pos: 1
                }
            })
        );

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::Declare));
    }

    #[test]
    fn test_unknown_token_after_success() {
        let input = "Declare\n@ Declare Aliases({})";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::Declare));

        assert_eq!(
            lexer.next_token(),
            Err(NenyrError {
                suggestion: Some("To resolve the error, please remove the unsupported token `@` from your Nenyr code and revalidate. Ensure all tokens comply with Nenyr syntax to avoid future issues.".to_string()),
                context_name: None,
                context_path: "".to_string(),
                error_message: "The current token `@` is not supported within Nenyr syntax. Please verify the token and ensure it adheres to the Nenyr language rules.".to_string(),
                error_kind: NenyrErrorKind::SyntaxError,
                error_tracing: NenyrErrorTracing {
                    line_before: Some("Declare".to_string()),
                    line_after: None,
                    error_line: Some("@ Declare Aliases({})".to_string()),
                    error_on_line: 2,
                    error_on_col: 2,
                    error_on_pos: 9
                }
            })
        );
    }

    #[test]
    fn test_unknown_token_between_success() {
        let input = "Declare\n@\nDeclare Aliases({})";
        let mut lexer = Lexer::new(input, "");

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::Declare));

        assert_eq!(
            lexer.next_token(),
            Err(NenyrError {
                suggestion: Some("To resolve the error, please remove the unsupported token `@` from your Nenyr code and revalidate. Ensure all tokens comply with Nenyr syntax to avoid future issues.".to_string()),
                context_name: None,
                context_path: "".to_string(),
                error_message: "The current token `@` is not supported within Nenyr syntax. Please verify the token and ensure it adheres to the Nenyr language rules.".to_string(),
                error_kind: NenyrErrorKind::SyntaxError,
                error_tracing: NenyrErrorTracing {
                    line_before: Some("Declare".to_string()),
                    line_after: Some("Declare Aliases({})".to_string()),
                    error_line: Some("@".to_string()),
                    error_on_line: 2,
                    error_on_col: 2,
                    error_on_pos: 9
                }
            })
        );

        assert_eq!(lexer.next_token(), Ok(NenyrTokens::Declare));
    }

    #[test]
    fn performance_test_large_valid_nenyr_vector() {
        let large_nenyr_vector: Vec<_> = (0..1_000_000).map(|_| "Construct").collect();

        for input in large_nenyr_vector {
            let mut lexer = Lexer::new(input, "");

            assert_eq!(lexer.next_token(), Ok(NenyrTokens::Construct));
        }
    }

    #[test]
    fn performance_test_large_not_valid_nenyr_vector() {
        let large_nenyr_vector: Vec<_> = (0..1_000_000).map(|_| "aConstruct").collect();

        for input in large_nenyr_vector {
            let mut lexer = Lexer::new(input, "");

            assert_eq!(
                lexer.next_token(),
                Ok(NenyrTokens::Identifier("aConstruct".to_string()))
            );
        }
    }

    #[test]
    fn performance_test_large_error_nenyr_vector() {
        let large_nenyr_vector: Vec<_> = (0..1_000_000).map(|_| "@Construct").collect();

        for input in large_nenyr_vector {
            let mut lexer = Lexer::new(input, "");

            assert_eq!(
                lexer.next_token(),
                Err(NenyrError {
                    suggestion: Some("To resolve the error, please remove the unsupported token `@` from your Nenyr code and revalidate. Ensure all tokens comply with Nenyr syntax to avoid future issues.".to_string()),
                    context_name: None,
                    context_path: "".to_string(),
                    error_message: "The current token `@` is not supported within Nenyr syntax. Please verify the token and ensure it adheres to the Nenyr language rules.".to_string(),
                    error_kind: NenyrErrorKind::SyntaxError,
                    error_tracing: NenyrErrorTracing {
                        line_before: None,
                        line_after: None,
                        error_line: Some("@Construct".to_string()),
                        error_on_line: 1,
                        error_on_col: 2,
                        error_on_pos: 1
                    }
                })
            );
        }
    }
}
