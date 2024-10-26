use converters::{property::NenyrPropertyConverter, style_pattern::NenyrStylePatternConverter};
use error::{NenyrError, NenyrErrorKind};
use lexer::Lexer;
use store::NenyrProcessStore;
use tokens::NenyrTokens;
use types::ast::NenyrAst;
use validators::{
    breakpoint::NenyrBreakpointValidator, identifier::NenyrIdentifierValidator,
    import::NenyrImportValidator, style_syntax::NenyrStyleSyntaxValidator,
    typeface::NenyrTypefaceValidator, variable_value::NenyrVariableValueValidator,
};

mod converters {
    pub mod property;
    pub mod style_pattern;
}

mod interfaces {
    pub mod aliases;
    pub mod animations;
    pub mod breakpoints;
    pub mod central;
    pub mod class;
    pub mod delimiters;
    pub mod handlers;
    pub mod imports;
    pub mod keywords;
    pub mod layout;
    pub mod literals;
    pub mod module;
    pub mod panoramic;
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
    pub mod identifier;
    pub mod import;
    pub mod style_syntax;
    pub mod typeface;
    pub mod variable_value;
}

mod error;
mod lexer;
mod macros;
mod store;
mod tokens;

/// A type alias for results returned by Nenyr operations.
///
/// The `NenyrResult` type is a specialized `Result` type that returns a value of type `T` on
/// success, or a `NenyrError` on failure. This allows for convenient error handling
/// specific to the Nenyr framework's parsing and processing operations.
pub type NenyrResult<T> = Result<T, NenyrError>;

/// A parser for processing Nenyr syntax.
///
/// The `NenyrParser` struct is responsible for parsing Nenyr code and constructing an
/// Abstract Syntax Tree (AST). It utilizes a lexer to tokenize the input and a processing
/// state to manage parsing operations. This struct implements various validation traits
/// to ensure that the syntax adheres to the expected format.
///
/// # Fields
/// - `lexer`: An instance of the `Lexer` used for tokenizing the raw Nenyr input.
/// - `context_path`: A string slice representing the path to the current context being parsed.
/// - `context_name`: An optional name for the current context, if defined.
/// - `current_token`: The token currently being processed, represented as a `NenyrTokens`.
/// - `processing_state`: An instance of `NenyrProcessStore` that maintains the state
///   during parsing operations.
pub struct NenyrParser<'a> {
    lexer: Lexer<'a>,
    context_path: &'a str,
    context_name: Option<String>,
    current_token: NenyrTokens,
    processing_state: NenyrProcessStore,
}

impl<'a> NenyrIdentifierValidator for NenyrParser<'a> {}
impl<'a> NenyrStyleSyntaxValidator for NenyrParser<'a> {}
impl<'a> NenyrPropertyConverter for NenyrParser<'a> {}
impl<'a> NenyrStylePatternConverter for NenyrParser<'a> {}
impl<'a> NenyrVariableValueValidator for NenyrParser<'a> {}
impl<'a> NenyrTypefaceValidator for NenyrParser<'a> {}
impl<'a> NenyrImportValidator for NenyrParser<'a> {}
impl<'a> NenyrBreakpointValidator for NenyrParser<'a> {}

impl<'a> NenyrParser<'a> {
    /// Creates a new instance of `NenyrParser`.
    ///
    /// This method initializes the parser with the raw Nenyr input and the context path.
    /// A new lexer is created based on the provided raw Nenyr string, and the initial
    /// token is set to `NenyrTokens::StartOfFile`.
    ///
    /// # Parameters
    /// - `raw_nenyr`: A string slice containing the raw Nenyr code to be parsed.
    /// - `context_path`: A string slice representing the path to the context being parsed.
    ///
    /// # Returns
    /// A new instance of `NenyrParser` ready to parse the given input.
    pub fn new(raw_nenyr: &'a str, context_path: &'a str) -> Self {
        let lexer = Lexer::new(raw_nenyr, &context_path);

        Self {
            lexer,
            context_path,
            context_name: None,
            current_token: NenyrTokens::StartOfFile,
            processing_state: NenyrProcessStore::new(),
        }
    }

    /// Parses the raw Nenyr input and constructs an AST.
    ///
    /// This method initiates the parsing process by processing the next token and
    /// verifying that the context begins with the `Construct` keyword. It then calls
    /// the appropriate method to parse the current context based on the token type.
    ///
    /// # Returns
    /// A `NenyrResult<NenyrAst>`, which is either the constructed AST or a `NenyrError`
    /// indicating a failure in parsing.
    pub fn parse(&mut self) -> NenyrResult<NenyrAst> {
        self.process_next_token()?;

        self.parse_construct_keyword(
            Some("Ensure that every Nenyr context starts with the `Construct` keyword at the root level to properly define the scope and structure of your context.".to_string()),
            "Expected the Nenyr context to begin with the `Construct` keyword at the root.",
            Self::parse_current_context,
        )
    }

    /// Parses the current context based on the token type.
    ///
    /// This method checks the current token and determines which context to parse:
    /// `Central`, `Layout`, or `Module`. If the token is recognized, it processes
    /// the corresponding context and returns the appropriate AST variant.
    ///
    /// # Returns
    /// A `NenyrResult<NenyrAst>` containing the parsed context or a `NenyrError`
    /// if the token does not match any valid context keyword.
    fn parse_current_context(&mut self) -> NenyrResult<NenyrAst> {
        match self.current_token {
            NenyrTokens::Central => {
                let central_context = self.process_central_context()?;

                Ok(NenyrAst::CentralContext(central_context))
            }
            NenyrTokens::Layout => {
                let layout_context = self.process_layout_context()?;

                Ok(NenyrAst::LayoutContext(layout_context))
            }
            NenyrTokens::Module => {
                let module_context = self.process_module_context()?;

                Ok(NenyrAst::ModuleContext(module_context))
            }
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
    }
}
