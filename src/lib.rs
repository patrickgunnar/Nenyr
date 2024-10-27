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

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn central_context_is_valid() {
        let raw_nenyr = "Construct Central {
    Declare Imports([
        Import('https://fonts.googleapis.com/css2?family=Matemasie&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap'),
        Import('../mocks/imports/another_external.css'),
        Import('../mocks/imports/external_styles.css'),
        Import('../mocks/imports/styles.css'),
    ]),
    Declare Typefaces({
        roseMartin: '../mocks/typefaces/rosemartin.regular.otf',
        regularEot: '../mocks/typefaces/showa-source-curry.regular-webfont.eot',
        regularSvg: '../mocks/typefaces/showa-source-curry.regular-webfont.svg',
        regularTtf: '../mocks/typefaces/showa-source-curry.regular-webfont.ttf',
        regularWoff: '../mocks/typefaces/showa-source-curry.regular-webfont.woff',
        regularWoff2: '../mocks/typefaces/showa-source-curry.regular-webfont.woff2'
    }),
    Declare Breakpoints({
        MobileFirst({
            onMobTablet: '780px',
            onMobDesktop: '1240px',
            onMobXl: '1440px',
            onMobXXl: '2240px'
        }),
        DesktopFirst({
            onDeskTablet: '780px',
            onDeskDesktop: '1240px',
            onDeskXl: '1440px',
            onDeskXXl: '2240px'
        })
    }),
    // Este é um comentário de linha.
    Declare Themes({
        Light({
            Variables({
                primaryColor: '#FFFFFF',
                secondaryColor: '#CCCCCC',
                accentColorVar: '#FF5733'
            })
        }),
        Dark({
            Variables({
                primaryColor: '#333333',
                secondaryColor: '#666666',
                accentColorVar: '#FF5733'
            })
        })
    }),
    Declare Aliases({
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    }),
    Declare Variables({
        myColor: '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    }),
    Declare Animation('giddyRespond') {
        Fraction(30, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Fraction(40, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction(4.0, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction([50, 70], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([5.0, 7.0], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([70, 80, 100], { // Este é um comentário de linha.
            transform: 'translate(50%, 50%)'
        })
    },
    Declare Animation('spiritedSavings') {
        Progressive({
            width: '${myVar}'
        }),
        Progressive({
            border: '1px solid red',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Progressive({
            backgroundColor: 'pink'
        }),
    },
    /* Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    */
    Declare Animation('grotesquePtarmigan') {
        From({
            width: '${myVar}'
        }),
        Halfway({
            border: '1px solid red'
        }),
        To({
            backgroundColor: 'pink',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        })
    },
    Declare Class('miniatureTrogon') Deriving('discreteAudio') {
        Important(true),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        Hover({
            background: '${secondaryColor}',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        PanoramicViewer({
            onMobTablet({
                Stylesheet({
                    // Este é um comentário de linha.
                    display: 'block', // Este é um comentário de linha.
                })
            }),
            onDeskDesktop({
                Hover({
                    bgd: '${secondaryColor}', // Este é um comentário de linha.
                    pdg: '${m15px}'
                })
            })
        })
    },
    Declare Class('myTestingClass') {
        Stylesheet({
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        PanoramicViewer({
            myBreakpoint({
                Stylesheet({
                    backgroundColor: 'blue',
                    border: '10px solid red',
                    height: '100px',
                    width: '200px'
                })
            })
        })
    }
}";
        let mut parser = NenyrParser::new(raw_nenyr, "src/central.nyr");

        assert_eq!(
            format!("{:?}", parser.parse()),
            "Ok(CentralContext(CentralContext { imports: Some(NenyrImports { values: {\"https://fonts.googleapis.com/css2?family=Matemasie&display=swap\": (), \"https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap\": (), \"https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap\": (), \"../mocks/imports/another_external.css\": (), \"../mocks/imports/external_styles.css\": (), \"../mocks/imports/styles.css\": ()} }), typefaces: Some(NenyrTypefaces { values: {\"roseMartin\": \"../mocks/typefaces/rosemartin.regular.otf\", \"regularEot\": \"../mocks/typefaces/showa-source-curry.regular-webfont.eot\", \"regularSvg\": \"../mocks/typefaces/showa-source-curry.regular-webfont.svg\", \"regularTtf\": \"../mocks/typefaces/showa-source-curry.regular-webfont.ttf\", \"regularWoff\": \"../mocks/typefaces/showa-source-curry.regular-webfont.woff\", \"regularWoff2\": \"../mocks/typefaces/showa-source-curry.regular-webfont.woff2\"} }), breakpoints: Some(NenyrBreakpoints { mobile_first: Some({\"onMobTablet\": \"780px\", \"onMobDesktop\": \"1240px\", \"onMobXl\": \"1440px\", \"onMobXXl\": \"2240px\"}), desktop_first: Some({\"onDeskTablet\": \"780px\", \"onDeskDesktop\": \"1240px\", \"onDeskXl\": \"1440px\", \"onDeskXXl\": \"2240px\"}) }), aliases: Some(NenyrAliases { values: {\"bgd\": \"background-color\", \"pdg\": \"padding\", \"dp\": \"display\", \"wd\": \"width\", \"hgt\": \"height\"} }), variables: Some(NenyrVariables { values: {\"myColor\": \"#FF6677\", \"grayColor\": \"gray\", \"blueColor\": \"blue\", \"redColor\": \"red\", \"primaryColor\": \"yellow\", \"secondaryColor\": \"white\"} }), themes: Some(NenyrThemes { light_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#FFFFFF\", \"secondaryColor\": \"#CCCCCC\", \"accentColorVar\": \"#FF5733\"} }), dark_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#333333\", \"secondaryColor\": \"#666666\", \"accentColorVar\": \"#FF5733\"} }) }), animations: Some({\"giddyRespond\": NenyrAnimation { animation_name: \"giddyRespond\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [30.0], properties: {\"bgd\": \"${accentColorVar}\", \"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"} }, Fraction { stops: [40.0], properties: {\"bgd\": \"${accentColorVar}\"} }, Fraction { stops: [4.0], properties: {\"bgd\": \"${accentColorVar}\"} }, Fraction { stops: [50.0, 70.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [5.0, 7.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [70.0, 80.0, 100.0], properties: {\"transform\": \"translate(50%, 50%)\"} }] }, \"spiritedSavings\": NenyrAnimation { animation_name: \"spiritedSavings\", kind: Some(Progressive), progressive_count: Some(3), keyframe: [Progressive({\"width\": \"${myVar}\"}), Progressive({\"border\": \"10px solid red\", \"background-color\": \"blue\", \"height\": \"100px\", \"width\": \"200px\"}), Progressive({\"background-color\": \"pink\"})] }, \"grotesquePtarmigan\": NenyrAnimation { animation_name: \"grotesquePtarmigan\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"width\": \"${myVar}\"}), Halfway({\"border\": \"1px solid red\"}), To({\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"})] }}), classes: Some({\"miniatureTrogon\": NenyrStyleClass { class_name: \"miniatureTrogon\", deriving_from: Some(\"discreteAudio\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"background-color\": \"#0000FF\", \"background\": \"#00FF00\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}, \":hover\": {\"background\": \"${secondaryColor}\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"display\": \"block\"}}, \"onDeskDesktop\": {\":hover\": {\"bgd\": \"${secondaryColor}\", \"pdg\": \"${m15px}\"}}}) }, \"myTestingClass\": NenyrStyleClass { class_name: \"myTestingClass\", deriving_from: None, is_important: None, style_patterns: Some({\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}), responsive_patterns: Some({\"myBreakpoint\": {\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}}) }}) }))".to_string()
        );
    }

    #[test]
    fn central_context_is_not_valid() {
        let raw_nenyr = "Construct Central {
    Declare Imports([
        Import('https://fonts.googleapis.com/css2?family=Matemasie&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap'),
        Import('../mocks/imports/another_external.css'),
        Import('../mocks/imports/external_styles.css'),
        Import('../mocks/imports/styles.css'),
    ]),
    Declare Typefaces({
        roseMartin: '../mocks/typefaces/rosemartin.regular.otf',
        regularEot: '../mocks/typefaces/showa-source-curry.regular-webfont.eot',
        regularSvg: '../mocks/typefaces/showa-source-curry.regular-webfont.svg',
        regularTtf: '../mocks/typefaces/showa-source-curry.regular-webfont.ttf',
        regularWoff: '../mocks/typefaces/showa-source-curry.regular-webfont.woff',
        regularWoff2: '../mocks/typefaces/showa-source-curry.regular-webfont.woff2'
    }),
    Declare Breakpoints({
        MobileFirst({
            onMobTablet: '780px',
            onMobDesktop: '1240px',
            onMobXl: '1440px',
            onMobXXl: '2240px'
        }),
        DesktopFirst({
            onDeskTablet: '780px',
            onDeskDesktop: '1240px',
            onDeskXl: '1440px',
            onDeskXXl: '2240px'
        })
    }),
    // Este é um comentário de linha.
    Declare Themes({
        Light({
            Variables({
                primaryColor: '#FFFFFF',
                secondaryColor: '#CCCCCC',
                accentColorVar: '#FF5733'
            })
        }),
        Dark({
            Variables({
                primaryColor: '#333333',
                secondaryColor: '#666666',
                accentColorVar: '#FF5733'
            })
        })
    }),
    Declare Aliases({
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    }),
    Declare Variables({
        myColor: '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    }),
    Declare Animation('giddyRespond') {
        Fraction(30, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Fraction(40, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction(4.0, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction([50, 70], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([5.0, 7.0], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([70, 80, 100], { // Este é um comentário de linha.
            transform: 'translate(50%, 50%)'
        })
    },
    Declare Animation('spiritedSavings') {
        Progressive({
            width: '${myVar}'
        }),
        Progressive({
            border: '1px solid red',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Progressive({
            backgroundColor: 'pink'
        }),
    },
    /* Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    */
    Declare Animation('grotesquePtarmigan') {
        From({
            width: '${myVar}'
        }),
        Halfway({
            border: '1px solid red'
        }),
        To({
            backgroundColor: 'pink',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        })
    },
    Declare Class('miniatureTrogon') Deriving('discreteAudio') 
        Important(true),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        Hover({
            background: '${secondaryColor}',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        PanoramicViewer({
            onMobTablet({
                Stylesheet({
                    // Este é um comentário de linha.
                    display: 'block', // Este é um comentário de linha.
                })
            }),
            onDeskDesktop({
                Hover({
                    bgd: '${secondaryColor}', // Este é um comentário de linha.
                    pdg: '${m15px}'
                })
            })
        })
    },
    Declare Class('myTestingClass') {
        Stylesheet({
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        PanoramicViewer({
            myBreakpoint({
                Stylesheet({
                    backgroundColor: 'blue',
                    border: '10px solid red',
                    height: '100px',
                    width: '200px'
                })
            })
        })
    }
}";
        let mut parser = NenyrParser::new(raw_nenyr, "src/central.nyr");

        assert_eq!(
            format!("{:?}", parser.parse()),
            "Err(NenyrError { suggestion: Some(\"Ensure that the `miniatureTrogon` class or deriving name declaration is followed by an opening curly bracket `{` to properly define the class block. The correct syntax is: `Declare Class('miniatureTrogon') { ... }` or `Declare Class('miniatureTrogon') Deriving('layoutName') { ... }`.\"), context_name: Some(\"Central\"), context_path: \"src/central.nyr\", error_message: \"An opening curly bracket `{` was expected after the `miniatureTrogon` class or deriving name declaration to start the class block, but it was not found. However, found `Important` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"    Declare Class('miniatureTrogon') Deriving('discreteAudio') \"), line_after: Some(\"        Stylesheet({\"), error_line: Some(\"        Important(true),\"), error_on_line: 129, error_on_col: 18, error_on_pos: 4164 } })".to_string()
        );
    }

    #[test]
    fn layout_context_is_valid() {
        let raw_nenyr = "Construct Layout('hellishAdobe') {
    // Este é um comentário de linha.
    Declare Themes({
        Light({
            Variables({
                primaryColor: '#FFFFFF',
                secondaryColor: '#CCCCCC',
                accentColorVar: '#FF5733'
            })
        }),
        Dark({
            Variables({
                primaryColor: '#333333',
                secondaryColor: '#666666',
                accentColorVar: '#FF5733'
            })
        })
    }),
    Declare Aliases({
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    }),
    Declare Variables({
        myColor: '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    }),
    Declare Animation('giddyRespond') {
        Fraction(30, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Fraction(40, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction(4.0, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction([50, 70], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([5.0, 7.0], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([70, 80, 100], { // Este é um comentário de linha.
            transform: 'translate(50%, 50%)'
        })
    },
    Declare Animation('spiritedSavings') {
        Progressive({
            width: '${myVar}'
        }),
        Progressive({
            border: '1px solid red',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Progressive({
            backgroundColor: 'pink'
        }),
    },
    /* Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    */
    Declare Animation('grotesquePtarmigan') {
        From({
            width: '${myVar}'
        }),
        Halfway({
            border: '1px solid red'
        }),
        To({
            backgroundColor: 'pink',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        })
    },
    Declare Class('miniatureTrogon') Deriving('discreteAudio') {
        Important(true),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        Hover({
            background: '${secondaryColor}',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        PanoramicViewer({
            onMobTablet({
                Stylesheet({
                    // Este é um comentário de linha.
                    display: 'block', // Este é um comentário de linha.
                })
            }),
            onDeskDesktop({
                Hover({
                    bgd: '${secondaryColor}', // Este é um comentário de linha.
                    pdg: '${m15px}'
                })
            })
        })
    },
    Declare Class('myTestingClass') {
        Stylesheet({
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        PanoramicViewer({
            myBreakpoint({
                Stylesheet({
                    backgroundColor: 'blue',
                    border: '10px solid red',
                    height: '100px',
                    width: '200px'
                })
            })
        })
    }
}
";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(
            format!("{:?}", parser.parse()),
            "Ok(LayoutContext(LayoutContext { layout_name: \"hellishAdobe\", aliases: Some(NenyrAliases { values: {\"bgd\": \"background-color\", \"pdg\": \"padding\", \"dp\": \"display\", \"wd\": \"width\", \"hgt\": \"height\"} }), variables: Some(NenyrVariables { values: {\"myColor\": \"#FF6677\", \"grayColor\": \"gray\", \"blueColor\": \"blue\", \"redColor\": \"red\", \"primaryColor\": \"yellow\", \"secondaryColor\": \"white\"} }), themes: Some(NenyrThemes { light_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#FFFFFF\", \"secondaryColor\": \"#CCCCCC\", \"accentColorVar\": \"#FF5733\"} }), dark_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#333333\", \"secondaryColor\": \"#666666\", \"accentColorVar\": \"#FF5733\"} }) }), animations: Some({\"giddyRespond\": NenyrAnimation { animation_name: \"giddyRespond\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [30.0], properties: {\"bgd\": \"${accentColorVar}\", \"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"} }, Fraction { stops: [40.0], properties: {\"bgd\": \"${accentColorVar}\"} }, Fraction { stops: [4.0], properties: {\"bgd\": \"${accentColorVar}\"} }, Fraction { stops: [50.0, 70.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [5.0, 7.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [70.0, 80.0, 100.0], properties: {\"transform\": \"translate(50%, 50%)\"} }] }, \"spiritedSavings\": NenyrAnimation { animation_name: \"spiritedSavings\", kind: Some(Progressive), progressive_count: Some(3), keyframe: [Progressive({\"width\": \"${myVar}\"}), Progressive({\"border\": \"10px solid red\", \"background-color\": \"blue\", \"height\": \"100px\", \"width\": \"200px\"}), Progressive({\"background-color\": \"pink\"})] }, \"grotesquePtarmigan\": NenyrAnimation { animation_name: \"grotesquePtarmigan\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"width\": \"${myVar}\"}), Halfway({\"border\": \"1px solid red\"}), To({\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"})] }}), classes: Some({\"miniatureTrogon\": NenyrStyleClass { class_name: \"miniatureTrogon\", deriving_from: Some(\"discreteAudio\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"background-color\": \"#0000FF\", \"background\": \"#00FF00\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}, \":hover\": {\"background\": \"${secondaryColor}\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"display\": \"block\"}}, \"onDeskDesktop\": {\":hover\": {\"bgd\": \"${secondaryColor}\", \"pdg\": \"${m15px}\"}}}) }, \"myTestingClass\": NenyrStyleClass { class_name: \"myTestingClass\", deriving_from: None, is_important: None, style_patterns: Some({\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}), responsive_patterns: Some({\"myBreakpoint\": {\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}}) }}) }))".to_string()
        );
    }

    #[test]
    fn layout_context_is_not_valid() {
        let raw_nenyr = "Construct Layout('hellishAdobe') {
    // Este é um comentário de linha.
    Declare Themes({
        Light({
            Variables({
                primaryColor: '#FFFFFF',
                secondaryColor: '#CCCCCC',
                accentColorVar: '#FF5733'
            })
        }),
        Dark({
            Variables({
                primaryColor: '#333333',
                secondaryColor: '#666666',
                accentColorVar: '#FF5733'
            })
        })
    }),
    Declare Aliases({
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    }),
    Declare Variables({
        myColor: '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    }),
    Declare Animation('giddyRespond') 
        Fraction(30, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Fraction(40, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction(4.0, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction([50, 70], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([5.0, 7.0], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([70, 80, 100], { // Este é um comentário de linha.
            transform: 'translate(50%, 50%)'
        })
    },
    Declare Animation('spiritedSavings') {
        Progressive({
            width: '${myVar}'
        }),
        Progressive({
            border: '1px solid red',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Progressive({
            backgroundColor: 'pink'
        }),
    },
    /* Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    */
    Declare Animation('grotesquePtarmigan') {
        From({
            width: '${myVar}'
        }),
        Halfway({
            border: '1px solid red'
        }),
        To({
            backgroundColor: 'pink',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        })
    },
    Declare Class('miniatureTrogon') Deriving('discreteAudio') {
        Important(true),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        Hover({
            background: '${secondaryColor}',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        PanoramicViewer({
            onMobTablet({
                Stylesheet({
                    // Este é um comentário de linha.
                    display: 'block', // Este é um comentário de linha.
                })
            }),
            onDeskDesktop({
                Hover({
                    bgd: '${secondaryColor}', // Este é um comentário de linha.
                    pdg: '${m15px}'
                })
            })
        })
    },
    Declare Class('myTestingClass') {
        Stylesheet({
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        PanoramicViewer({
            myBreakpoint({
                Stylesheet({
                    backgroundColor: 'blue',
                    border: '10px solid red',
                    height: '100px',
                    width: '200px'
                })
            })
        })
    }
}
";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(
            format!("{:?}", parser.parse()),
            "Err(NenyrError { suggestion: Some(\"Ensure that the `giddyRespond` animation name declaration is followed by an opening curly bracket `{` to properly define the animation block. The correct syntax is: `Declare Animation('giddyRespond') { ... }`.\"), context_name: Some(\"hellishAdobe\"), context_path: \"\", error_message: \"An opening curly bracket `{` was expected after the `giddyRespond` animation name declaration to start the animation block, but it was not found. However, found `Fraction` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"    Declare Animation('giddyRespond') \"), line_after: Some(\"            // Este é um comentário de linha.\"), error_line: Some(\"        Fraction(30, {\"), error_on_line: 37, error_on_col: 17, error_on_pos: 942 } })".to_string()
        );
    }

    #[test]
    fn module_context_is_valid() {
        let raw_nenyr = "Construct Module('ultimateFeel') Extending('hellishAdobe') {
    Declare Aliases({
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    }),
    Declare Variables({
        myColor: '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    }),
    Declare Animation('giddyRespond') {
        Fraction(30, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Fraction(40, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction(4.0, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction([50, 70], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([5.0, 7.0], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([70, 80, 100], { // Este é um comentário de linha.
            transform: 'translate(50%, 50%)'
        })
    },
    Declare Animation('spiritedSavings') {
        Progressive({
            width: '${myVar}'
        }),
        Progressive({
            border: '1px solid red',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Progressive({
            backgroundColor: 'pink'
        }),
    },
    /* Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    */
    Declare Animation('grotesquePtarmigan') {
        From({
            width: '${myVar}'
        }),
        Halfway({
            border: '1px solid red'
        }),
        To({
            backgroundColor: 'pink',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        })
    },
    Declare Class('miniatureTrogon') Deriving('discreteAudio') {
        Important(true),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        Hover({
            background: '${secondaryColor}',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        PanoramicViewer({
            onMobTablet({
                Stylesheet({
                    // Este é um comentário de linha.
                    display: 'block', // Este é um comentário de linha.
                })
            }),
            onDeskDesktop({
                Hover({
                    bgd: '${secondaryColor}', // Este é um comentário de linha.
                    pdg: '${m15px}'
                })
            })
        })
    },
    Declare Class('myTestingClass') {
        Stylesheet({
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        PanoramicViewer({
            myBreakpoint({
                Stylesheet({
                    backgroundColor: 'blue',
                    border: '10px solid red',
                    height: '100px',
                    width: '200px'
                })
            })
        })
    }
}
";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(
            format!("{:?}", parser.parse()),
            "Ok(ModuleContext(ModuleContext { module_name: \"ultimateFeel\", extending_from: Some(\"hellishAdobe\"), aliases: Some(NenyrAliases { values: {\"bgd\": \"background-color\", \"pdg\": \"padding\", \"dp\": \"display\", \"wd\": \"width\", \"hgt\": \"height\"} }), variables: Some(NenyrVariables { values: {\"myColor\": \"#FF6677\", \"grayColor\": \"gray\", \"blueColor\": \"blue\", \"redColor\": \"red\", \"primaryColor\": \"yellow\", \"secondaryColor\": \"white\"} }), animations: Some({\"giddyRespond\": NenyrAnimation { animation_name: \"giddyRespond\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [30.0], properties: {\"bgd\": \"${accentColorVar}\", \"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"} }, Fraction { stops: [40.0], properties: {\"bgd\": \"${accentColorVar}\"} }, Fraction { stops: [4.0], properties: {\"bgd\": \"${accentColorVar}\"} }, Fraction { stops: [50.0, 70.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [5.0, 7.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [70.0, 80.0, 100.0], properties: {\"transform\": \"translate(50%, 50%)\"} }] }, \"spiritedSavings\": NenyrAnimation { animation_name: \"spiritedSavings\", kind: Some(Progressive), progressive_count: Some(3), keyframe: [Progressive({\"width\": \"${myVar}\"}), Progressive({\"border\": \"10px solid red\", \"background-color\": \"blue\", \"height\": \"100px\", \"width\": \"200px\"}), Progressive({\"background-color\": \"pink\"})] }, \"grotesquePtarmigan\": NenyrAnimation { animation_name: \"grotesquePtarmigan\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"width\": \"${myVar}\"}), Halfway({\"border\": \"1px solid red\"}), To({\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"})] }}), classes: Some({\"miniatureTrogon\": NenyrStyleClass { class_name: \"miniatureTrogon\", deriving_from: Some(\"discreteAudio\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"background-color\": \"#0000FF\", \"background\": \"#00FF00\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}, \":hover\": {\"background\": \"${secondaryColor}\", \"padding\": \"${m15px21}\", \"bdr\": \"5px\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"display\": \"block\"}}, \"onDeskDesktop\": {\":hover\": {\"bgd\": \"${secondaryColor}\", \"pdg\": \"${m15px}\"}}}) }, \"myTestingClass\": NenyrStyleClass { class_name: \"myTestingClass\", deriving_from: None, is_important: None, style_patterns: Some({\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}), responsive_patterns: Some({\"myBreakpoint\": {\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}}) }}) }))".to_string()
        );
    }

    #[test]
    fn module_context_is_not_valid() {
        let raw_nenyr = "Construct Module('ultimateFeel') Extending('hellishAdobe') {
    Declare Aliases({
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    }),
    Declare Variables(
        myColor: '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    }),
    Declare Animation('giddyRespond') {
        Fraction(30, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Fraction(40, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction(4.0, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
        }),
        Fraction([50, 70], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([5.0, 7.0], { // Este é um comentário de linha.
            backgroundColor: 'blue'
        }),
        Fraction([70, 80, 100], { // Este é um comentário de linha.
            transform: 'translate(50%, 50%)'
        })
    },
    Declare Animation('spiritedSavings') {
        Progressive({
            width: '${myVar}'
        }),
        Progressive({
            border: '1px solid red',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Progressive({
            backgroundColor: 'pink'
        }),
    },
    /* Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    - Este é um commentário de bloco
    */
    Declare Animation('grotesquePtarmigan') {
        From({
            width: '${myVar}'
        }),
        Halfway({
            border: '1px solid red'
        }),
        To({
            backgroundColor: 'pink',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        })
    },
    Declare Class('miniatureTrogon') Deriving('discreteAudio') {
        Important(true),
        Stylesheet({
            backgroundColor: '${accentColorVar}',
            backgroundColor: '#0000FF',
            background: '#00FF00',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        Hover({
            background: '${secondaryColor}',
            padding: '${m15px21}',
            bdr: '5px'
        }),
        PanoramicViewer({
            onMobTablet({
                Stylesheet({
                    // Este é um comentário de linha.
                    display: 'block', // Este é um comentário de linha.
                })
            }),
            onDeskDesktop({
                Hover({
                    bgd: '${secondaryColor}', // Este é um comentário de linha.
                    pdg: '${m15px}'
                })
            })
        })
    },
    Declare Class('myTestingClass') {
        Stylesheet({
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        PanoramicViewer({
            myBreakpoint({
                Stylesheet({
                    backgroundColor: 'blue',
                    border: '10px solid red',
                    height: '100px',
                    width: '200px'
                })
            })
        })
    }
}
";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        assert_eq!(
            format!("{:?}", parser.parse()),
            "Err(NenyrError { suggestion: Some(\"After the opening parenthesis, an opening curly bracket `{` is required to properly define the properties block in the `Variables` declaration. Ensure the pattern follows correct Nenyr syntax, like `Variables({ key: 'value', ... })`.\"), context_name: Some(\"ultimateFeel\"), context_path: \"\", error_message: \"The `Variables` declaration block was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis. However, found `myColor` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"    Declare Variables(\"), line_after: Some(\"        grayColor: 'gray',\"), error_line: Some(\"        myColor: '#FF6677',\"), error_on_line: 11, error_on_col: 16, error_on_pos: 266 } })".to_string()
        );
    }
}
