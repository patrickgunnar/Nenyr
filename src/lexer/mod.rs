use crate::tokens::NenyrTokens;

pub struct Lexer<'a> {
    raw_nenyr: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(raw_nenyr: &'a str) -> Self {
        Self {
            raw_nenyr,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    // TODO: Trace the current position
    pub fn trace_lexer_position() {
        let on_line = 0;
        let on_col = 0;
        let on_pos = 0;

        let line = String::new(); //
        let line_before = String::new(); // Option
        let line_after = String::new(); // Option
    }

    pub fn current_char(&self) -> Option<char> {
        self.raw_nenyr[self.position..].chars().next()
    }

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

    fn parse_number(&mut self) -> NenyrTokens {
        let start_pos = self.position;

        while let Some(char) = self.current_char() {
            if char.is_digit(10) {
                self.position += char.len_utf8();
                self.column += char.len_utf8();
            } else {
                break;
            }
        }

        let value = self.raw_nenyr[start_pos..self.position].parse().unwrap();

        NenyrTokens::Number(value)
    }

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

    fn match_identifier(&self, identifier: String) -> NenyrTokens {
        match identifier {
            _ => NenyrTokens::EndOfLine,
        }
    }
}
