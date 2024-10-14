use error::{NenyrError, NenyrErrorKind};
use lexer::Lexer;

mod converters {
    pub mod property;
    pub mod style_pattern;
}

mod interfaces {
    pub mod aliases;
    pub mod animation;
    pub mod breakpoints;
    pub mod class;
    pub mod imports;
    pub mod themes;
    pub mod typefaces;
    pub mod variables;
}

mod modules {
    pub mod aliases;
    pub mod animations;
    pub mod ast;
    pub mod breakpoints;
    pub mod central;
    pub mod class;
    pub mod imports;
    pub mod layout;
    pub mod module;
    pub mod themes;
    pub mod typefaces;
    pub mod variables;
}

mod validators {
    pub mod breakpoint;
    pub mod i64_vector;
    pub mod identifier;
    pub mod import;
    pub mod style_syntax;
    pub mod typeface;
    pub mod variable_value;
}

mod error;
mod lexer;
mod tokens;

pub struct NenyrParser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> NenyrParser<'a> {
    pub fn new(raw_nenyr: &'a str) -> Self {
        let lexer = Lexer::new(raw_nenyr);

        Self { lexer }
    }
}
