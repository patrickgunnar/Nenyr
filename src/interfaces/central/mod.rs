use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::central::CentralContext,
    NenyrParser, NenyrResult,
};

/// # NenyrParser Central Context Parsing Methods
///
/// This implementation of the `NenyrParser` provides methods specifically designed
/// to parse blocks of code that define the central context within the Nenyr language.
/// The central context is crucial for the overall structure and function of the Nenyr
/// program, and these methods ensure that the syntax adheres to the expected rules
/// and conventions for central context declarations.
///
/// The methods in this group validate the syntax, process specific tokens, and
/// generate meaningful error messages if the input does not conform to the expected
/// structure. These methods work together to build a complete representation of
/// the central context, encapsulated in the `CentralContext` type.
impl NenyrParser {
    /// Processes the central context declaration.
    ///
    /// This method initiates the parsing of a central context block by moving to the
    /// next token after the `Central` keyword. It expects an opening curly bracket
    /// `{` to signify the start of the block. If the expected tokens are not found,
    /// it generates appropriate error messages to guide the user in correcting the syntax.
    ///
    /// The method then calls `retrieve_central_context_block` to handle the contents
    /// of the block. Upon successful parsing, it returns an instance of `CentralContext`.
    ///
    /// # Errors
    /// Returns a `NenyrError` if the expected opening or closing curly brackets are
    /// not found, or if there are syntax issues within the central context block.
    pub(crate) fn process_central_context(&mut self) -> NenyrResult<CentralContext> {
        self.process_next_token()?;
        self.set_context_name(Some("Central".to_string()));

        self.parse_curly_bracketed_delimiter(
            Some("Ensure the construction of the central context includes an opening curly bracket `{` to define the start of the context block. The correct syntax is `Construct Central { ... }`.".to_string()),
            "Expected an opening curly bracket `{` after the `Central` keyword to start the central context block, but none was found.",
            Some("Ensure the central context block is properly closed with a curly bracket `}`. The correct syntax is `Construct Central { ... }`.".to_string()),
            "Expected a closing curly bracket `}` to end the central context block, but none was found.",
            Self::retrieve_central_context_block,
        )
    }

    /// Retrieves the contents of the central context block.
    ///
    /// This method processes the individual declarations within the central context block
    /// and validates them using the `loop_while_not!` macro to ensure correct syntax.
    /// It handles multiple declarations that are separated by commas, and it specifically
    /// looks for the `Declare` keyword as part of the declarations.
    ///
    /// If a syntax error, such as a missing comma or a duplicated comma, is detected,
    /// an appropriate error message is generated to inform the user of the issue.
    ///
    /// # Errors
    /// Returns a `NenyrError` if the expected syntax is not followed within the block,
    /// including issues like duplicated commas or missing declarations.
    fn retrieve_central_context_block(&mut self) -> NenyrResult<CentralContext> {
        let mut central_context = CentralContext::new();

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the Nenyr central context to ensure proper syntax. The parser expects the `Declare` keyword to follow valid delimiters.".to_string()),
            "A duplicated comma was found inside the Nenyr central context. The parser expected to find the `Declare` keyword but did not.",
            Some("Ensure that a comma is placed after each block definition to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Construct Central { Declare Typefaces({ ... }), Declare Imports([ ... ]), ... }`.".to_string()),
            "All methods inside the central context block must be separated by commas. A comma is missing after the block definition. The parser expected a comma to separate elements but did not find one.",
            || self.processing_state.is_context_active(),
            |is_active| self.processing_state.set_context_active(is_active),
            {
                self.handle_central_declare_keyword(&mut central_context)?;
            }
        );

        self.processing_state.set_context_active(false);

        Ok(central_context)
    }

    /// Handles the parsing of declarations within the central context.
    ///
    /// This method specifically looks for the `Declare` keyword and validates that
    /// each declaration adheres to the expected syntax rules. If the `Declare` keyword
    /// is missing or malformed, an error is raised to guide the user in correcting the
    /// syntax.
    ///
    /// The method then calls `process_central_methods` to handle the actual methods
    /// declared within the context.
    ///
    /// # Parameters
    /// - `central_context`: A mutable reference to the `CentralContext` where parsed
    ///   declarations will be stored.
    ///
    /// # Errors
    /// Returns a `NenyrError` if the declaration does not start with the `Declare`
    /// keyword or if any other syntax issues are encountered.
    fn handle_central_declare_keyword(
        &mut self,
        central_context: &mut CentralContext,
    ) -> NenyrResult<()> {
        self.parse_declare_keyword(
            Some("Ensure that each method inside the central context is correctly declared using the `Declare` keyword. Example: `Construct Central { Declare Breakpoints({ ... }), Declare Class({ ... }), ... }`.".to_string()),
            "All methods declaration inside the central context must begin with the `Declare` keyword, but one of the methods is missing this declaration."
        )?;

        self.process_central_methods(central_context)
    }

    /// Processes individual method declarations within the central context.
    ///
    /// This method validates the current token to ensure it corresponds to one of the
    /// valid methods that can be declared within the central context. It checks for
    /// specific tokens like `Imports`, `Typefaces`, `Breakpoints`, and others.
    ///
    /// If an invalid method is encountered, an error is generated with a suggestion
    /// for correcting the issue based on the expected methods allowed in the central
    /// context.
    ///
    /// # Parameters
    /// - `central_context`: A mutable reference to the `CentralContext` instance to
    ///   which methods will be added.
    ///
    /// # Errors
    /// Returns a `NenyrError` if an unsupported method is declared or if the current
    /// token does not match any valid method.
    fn process_central_methods(&mut self, central_context: &mut CentralContext) -> NenyrResult<()> {
        self.processing_state.set_context_active(true);

        match self.current_token {
            NenyrTokens::Imports => {
                let imports = self.process_imports_method()?;

                central_context.add_imports_to_context(imports);
            }
            NenyrTokens::Typefaces => {
                let typefaces = self.process_typefaces_method()?;

                central_context.add_typefaces_to_context(typefaces);
            }
            NenyrTokens::Breakpoints => {
                let breakpoints = self.process_breakpoints_method()?;

                central_context.add_breakpoints_to_context(breakpoints);
            }
            NenyrTokens::Aliases => {
                let aliases = self.process_aliases_method()?;

                central_context.add_aliases_to_context(aliases);
            }
            NenyrTokens::Variables => {
                let variables = self.process_variables_method(false)?;

                central_context.add_variables_to_context(variables);
            }
            NenyrTokens::Themes => {
                let themes = self.process_themes_method()?;

                central_context.add_themes_to_context(themes);
            }
            NenyrTokens::Animation => {
                let (animation_name, animation) = self.process_animation_method()?;

                central_context.add_animation_to_context(animation_name, animation);
            }
            NenyrTokens::Class => {
                let (class_name, style_class) = self.process_class_method()?;

                central_context.add_style_class_to_context(class_name, style_class);
            }
            _ => {
                return Err(NenyrError::new(
                    Some("Ensure that only valid methods supported by the central context are declared. Review the documentation for methods allowed within `Construct Central { ... }`.".to_string()),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error("One of the methods in the central context is either not a valid Nenyr method or is not supported within the central context."),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn central_context_is_valid() {
        let raw_nenyr = "Central {
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
    },
}";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: None, typefaces: None, breakpoints: None, aliases: None, variables: None, themes: None, animations: None, classes: Some({\"miniatureTrogon\": NenyrStyleClass { class_name: \"miniatureTrogon\", deriving_from: Some(\"discreteAudio\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"background-color\": \"#0000FF\", \"background\": \"#00FF00\", \"padding\": \"${m15px21}\", \"nickname;bdr\": \"5px\"}, \":hover\": {\"background\": \"${secondaryColor}\", \"padding\": \"${m15px21}\", \"nickname;bdr\": \"5px\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"display\": \"block\"}}, \"onDeskDesktop\": {\":hover\": {\"nickname;bgd\": \"${secondaryColor}\", \"nickname;pdg\": \"${m15px}\"}}}) }, \"myTestingClass\": NenyrStyleClass { class_name: \"myTestingClass\", deriving_from: None, is_important: None, style_patterns: Some({\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}), responsive_patterns: Some({\"myBreakpoint\": {\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}}) }}) })".to_string()
        );
    }

    #[test]
    fn central_context_is_not_valid() {
        let raw_nenyr = "Central {
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
        PanoramicViewer(
            myBreakpoint({
                Stylesheet({
                    backgroundColor: 'blue',
                    border: '10px solid red',
                    height: '100px',
                    width: '200px'
                })
            })
        })
    },
}";

        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Err(NenyrError { suggestion: Some(\"After the open parenthesis, an opening curly bracket `{` is required to properly define the panoramic block in `myTestingClass` class. Ensure the panoramic pattern follows the correct Nenyr syntax, such as `Class('myTestingClass') { PanoramicViewer({ ... }) }`.\"), context_name: Some(\"Central\"), context_path: \"\", error_message: \"The panoramic pattern in the `myTestingClass` class was expected to receive an object as a value, but an opening curly bracket `{` was not found after the open parenthesis. However, found `myBreakpoint` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        PanoramicViewer(\"), line_after: Some(\"                Stylesheet({\"), error_line: Some(\"            myBreakpoint({\"), error_on_line: 39, error_on_col: 25, error_on_pos: 1166 } })".to_string()
        );
    }

    #[test]
    fn only_imports_central_is_valid() {
        let raw_nenyr = "Central {
        Declare Imports([
        Import('https://fonts.googleapis.com/css2?family=Matemasie&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap'),
        Import('https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap'),
        Import('../../../mocks/imports/another_external.css'),
        Import('../../../mocks/imports/external_styles.css'),
        Import('../../../mocks/imports/styles.css'),
    ])}";

        let mut parser = NenyrParser::new();

        parser.setup_dependencies(
            raw_nenyr.to_string(),
            "src/interfaces/central/central.nyr".to_string(),
        );

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: Some(NenyrImports { values: {\"https://fonts.googleapis.com/css2?family=Matemasie&display=swap\": (), \"https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap\": (), \"https://fonts.googleapis.com/css2?family=Bungee+Tint&display=swap\": (), \"../../../mocks/imports/another_external.css\": (), \"../../../mocks/imports/external_styles.css\": (), \"../../../mocks/imports/styles.css\": ()} }), typefaces: None, breakpoints: None, aliases: None, variables: None, themes: None, animations: None, classes: None })".to_string()
        );
    }

    #[test]
    fn only_typefaces_central_is_valid() {
        let raw_nenyr = "Central {
        Declare Typefaces({
        roseMartin: '../../../mocks/typefaces/rosemartin.regular.otf',
        regularEot: '../../../mocks/typefaces/showa-source-curry.regular-webfont.eot',
        regularSvg: '../../../mocks/typefaces/showa-source-curry.regular-webfont.svg',
        regularTtf: '../../../mocks/typefaces/showa-source-curry.regular-webfont.ttf',
        regularWoff: '../../../mocks/typefaces/showa-source-curry.regular-webfont.woff',
        regularWoff2: '../../../mocks/typefaces/showa-source-curry.regular-webfont.woff2'
    })}";

        let mut parser = NenyrParser::new();
        parser.setup_dependencies(
            raw_nenyr.to_string(),
            "src/interfaces/central/central.nyr".to_string(),
        );

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: None, typefaces: Some(NenyrTypefaces { values: {\"roseMartin\": \"../../../mocks/typefaces/rosemartin.regular.otf\", \"regularEot\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.eot\", \"regularSvg\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.svg\", \"regularTtf\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.ttf\", \"regularWoff\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.woff\", \"regularWoff2\": \"../../../mocks/typefaces/showa-source-curry.regular-webfont.woff2\"} }), breakpoints: None, aliases: None, variables: None, themes: None, animations: None, classes: None })".to_string()
        );
    }

    #[test]
    fn only_breakpoints_central_is_valid() {
        let raw_nenyr = "Central {
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
    })}";

        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: None, typefaces: None, breakpoints: Some(NenyrBreakpoints { mobile_first: Some({\"onMobTablet\": \"780px\", \"onMobDesktop\": \"1240px\", \"onMobXl\": \"1440px\", \"onMobXXl\": \"2240px\"}), desktop_first: Some({\"onDeskTablet\": \"780px\", \"onDeskDesktop\": \"1240px\", \"onDeskXl\": \"1440px\", \"onDeskXXl\": \"2240px\"}) }), aliases: None, variables: None, themes: None, animations: None, classes: None })".to_string()
        );
    }

    #[test]
    fn only_themes_central_is_valid() {
        let raw_nenyr = "Central {
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
    })}";

        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: None, typefaces: None, breakpoints: None, aliases: None, variables: None, themes: Some(NenyrThemes { light_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#FFFFFF\", \"secondaryColor\": \"#CCCCCC\", \"accentColorVar\": \"#FF5733\"} }), dark_schema: Some(NenyrVariables { values: {\"primaryColor\": \"#333333\", \"secondaryColor\": \"#666666\", \"accentColorVar\": \"#FF5733\"} }) }), animations: None, classes: None })".to_string()
        );
    }

    #[test]
    fn only_aliases_central_is_valid() {
        let raw_nenyr = "Central {
        Declare Aliases({
        bgd: background,
        bgd: backgroundColor,
        pdg: padding,
        dp: display,
        wd: width,
        hgt: height
    })}";

        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: None, typefaces: None, breakpoints: None, aliases: Some(NenyrAliases { values: {\"bgd\": \"background-color\", \"pdg\": \"padding\", \"dp\": \"display\", \"wd\": \"width\", \"hgt\": \"height\"} }), variables: None, themes: None, animations: None, classes: None })".to_string()
        );
    }

    #[test]
    fn only_variables_central_is_valid() {
        let raw_nenyr = "Central {
        Declare Variables({
        myColor: '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    })}";

        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: None, typefaces: None, breakpoints: None, aliases: None, variables: Some(NenyrVariables { values: {\"myColor\": \"#FF6677\", \"grayColor\": \"gray\", \"blueColor\": \"blue\", \"redColor\": \"red\", \"primaryColor\": \"yellow\", \"secondaryColor\": \"white\"} }), themes: None, animations: None, classes: None })".to_string()
        );
    }

    #[test]
    fn only_animation_central_is_valid() {
        let raw_nenyr = "Central {
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
        }),
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
    }}";

        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: None, typefaces: None, breakpoints: None, aliases: None, variables: None, themes: None, animations: Some({\"giddyRespond\": NenyrAnimation { animation_name: \"giddyRespond\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [30.0], properties: {\"nickname;bgd\": \"${accentColorVar}\", \"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"} }, Fraction { stops: [40.0], properties: {\"nickname;bgd\": \"${accentColorVar}\"} }, Fraction { stops: [4.0], properties: {\"nickname;bgd\": \"${accentColorVar}\"} }, Fraction { stops: [50.0, 70.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [5.0, 7.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [70.0, 80.0, 100.0], properties: {\"transform\": \"translate(50%, 50%)\"} }] }, \"spiritedSavings\": NenyrAnimation { animation_name: \"spiritedSavings\", kind: Some(Progressive), progressive_count: Some(3), keyframe: [Progressive({\"width\": \"${myVar}\"}), Progressive({\"border\": \"10px solid red\", \"background-color\": \"blue\", \"height\": \"100px\", \"width\": \"200px\"}), Progressive({\"background-color\": \"pink\"})] }, \"grotesquePtarmigan\": NenyrAnimation { animation_name: \"grotesquePtarmigan\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"width\": \"${myVar}\"}), Halfway({\"border\": \"1px solid red\"}), To({\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"})] }}), classes: None })".to_string()
        );
    }

    #[test]
    fn only_class_central_is_valid() {
        let raw_nenyr = "Central {
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
    }}";

        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_central_context()),
            "Ok(CentralContext { imports: None, typefaces: None, breakpoints: None, aliases: None, variables: None, themes: None, animations: None, classes: Some({\"miniatureTrogon\": NenyrStyleClass { class_name: \"miniatureTrogon\", deriving_from: Some(\"discreteAudio\"), is_important: Some(true), style_patterns: Some({\"_stylesheet\": {\"background-color\": \"#0000FF\", \"background\": \"#00FF00\", \"padding\": \"${m15px21}\", \"nickname;bdr\": \"5px\"}, \":hover\": {\"background\": \"${secondaryColor}\", \"padding\": \"${m15px21}\", \"nickname;bdr\": \"5px\"}}), responsive_patterns: Some({\"onMobTablet\": {\"_stylesheet\": {\"display\": \"block\"}}, \"onDeskDesktop\": {\":hover\": {\"nickname;bgd\": \"${secondaryColor}\", \"nickname;pdg\": \"${m15px}\"}}}) }, \"myTestingClass\": NenyrStyleClass { class_name: \"myTestingClass\", deriving_from: None, is_important: None, style_patterns: Some({\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}), responsive_patterns: Some({\"myBreakpoint\": {\"_stylesheet\": {\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"}}}) }}) })".to_string()
        );
    }
}
