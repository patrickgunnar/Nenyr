use error::{NenyrError, NenyrErrorKind};
use lexer::Lexer;
use tokens::NenyrTokens;
use types::ast::NenyrAst;

mod converters {
    pub mod property;
    pub mod style_pattern;
}

mod creators {
    pub mod aliases;
    pub mod animation;
    pub mod breakpoints;
    pub mod class;
    pub mod imports;
    pub mod themes;
    pub mod typefaces;
    pub mod variables;
}

mod interfaces {
    pub mod aliases;
    pub mod animations;
    pub mod breakpoints;
    pub mod class;
    pub mod delimiters;
    pub mod handlers;
    pub mod imports;
    pub mod keywords;
    pub mod literals;
    pub mod patterns;
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
    pub mod style_syntax;
    pub mod typeface;
    pub mod variable_value;
}

mod error;
mod lexer;
mod tokens;

pub type NenyrResult<T> = Result<T, NenyrError>;

pub struct NenyrParser<'a> {
    lexer: Lexer<'a>,
    context_path: &'a str,
    current_token: NenyrTokens,
}

impl<'a> NenyrParser<'a> {
    pub fn new(raw_nenyr: &'a str, context_path: &'a str) -> Self {
        let lexer = Lexer::new(raw_nenyr, &context_path);

        Self {
            lexer,
            context_path,
            current_token: NenyrTokens::StartOfFile,
        }
    }

    pub fn parse(&mut self) -> NenyrResult<NenyrAst> {
        self.process_next_token()?;

        self.parse_construct_keyword(
            Some("Ensure that every Nenyr context starts with the `Construct` keyword at the root level to properly define the scope and structure of your context.".to_string()),
            "Expected the Nenyr context to begin with the `Construct` keyword at the root.".to_string(),
            Self::parse_current_context,
        )
    }

    fn parse_current_context(&mut self) -> NenyrResult<NenyrAst> {
        match self.current_token {
            NenyrTokens::Central => {}
            NenyrTokens::Layout => {}
            NenyrTokens::Module => {}
            _ => {
                return Err(NenyrError::new(
                    Some("To define a Nenyr Context, please use one of the following keywords: `Central`, `Layout`, or `Module`.".to_string()),
                    None,
                    self.context_path.to_string(),
                    format!("The token `{:?}` is not recognized as a valid Nenyr context keyword. Please check your syntax.", self.current_token),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }

        Ok(NenyrAst {
            central_context: None,
            layout_context: None,
            module_context: None,
        })
    }
}
