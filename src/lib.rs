use error::{NenyrError, NenyrErrorKind};

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

mod types {
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
    pub mod property;
    pub mod style_pattern;
    pub mod style_syntax;
    pub mod typeface;
    pub mod variable_value;
}

mod error;

pub fn throw_error() -> NenyrError {
    NenyrError::new(
        Some("suggestion".to_string()),
        Some("line before".to_string()),
        Some("line after".to_string()),
        Some("context name".to_string()),
        "context path".to_string(),
        "error line".to_string(),
        "error message".to_string(),
        NenyrErrorKind::SyntaxError,
        10,
        5,
        20,
    )
}
