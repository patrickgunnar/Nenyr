use indexmap::IndexMap;

use crate::{
    converters::property::NenyrPropertyConverter,
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::animations::{NenyrAnimation, NenyrAnimationKind, NenyrSubAnimationKind},
    validators::{identifier::NenyrIdentifierValidator, style_syntax::NenyrStyleSyntaxValidator},
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    /// Processes the entire animation method declaration, which includes
    /// parsing the animation name and its corresponding block.
    ///
    /// # Syntax
    /// Expected syntax for an animation declaration:
    ///
    /// ```nenyr
    /// Declare Animation('animationName') {
    ///     // Animation patterns and blocks go here
    /// }
    /// ```
    ///
    /// # Returns
    /// Returns a tuple containing the animation name and the parsed `NenyrAnimation`.
    ///
    /// # Errors
    /// Will return a `NenyrError` if:
    /// - The animation name is improperly formatted.
    /// - Curly brackets are missing to delimit the animation block.
    pub(crate) fn process_animation_method(&mut self) -> NenyrResult<(String, NenyrAnimation)> {
        self.process_next_token()?;

        let animation_name = self.retrieve_animation_name()?;

        self.process_next_token()?;

        self.parse_curly_bracketed_delimiter(
            Some(format!("Ensure that the `{}` animation name declaration is followed by an opening curly bracket `{{` to properly define the animation block. The correct syntax is: `Declare Animation('{}') {{ ... }}`.", &animation_name, &animation_name)),
            &format!("An opening curly bracket `{{` was expected after the `{}` animation name declaration to start the animation block, but it was not found.", &animation_name),
            Some(format!("Ensure that each animation definition block is properly closed with a corresponding closing curly bracket `}}`. Example: `Declare Animation('{}') {{ ... }}`.", &animation_name)),
            &format!("A closing curly bracket `}}` was expected to terminate the `{}` animation definition block, but it was not found.", &animation_name),
            |parser| parser.process_animation_block(&animation_name),
        )
    }

    /// Retrieves the name of the animation by parsing the text between the parentheses
    /// in the animation declaration.
    ///
    /// # Syntax
    /// The syntax for naming an animation is as follows:
    ///
    /// ```nenyr
    /// Animation('animationName')
    /// ```
    ///
    /// # Returns
    /// Returns the animation name as a string if successfully parsed.
    ///
    /// # Errors
    /// - A `NenyrError` will be returned if:
    ///   - Parentheses are missing or improperly placed.
    ///   - The animation name is empty or contains invalid characters.
    fn retrieve_animation_name(&mut self) -> NenyrResult<String> {
        let animation_name = self.parse_parenthesized_delimiter(
            Some("Ensure that an opening parenthesis `(` is placed after the keyword `Animation` to properly define the animation name. The correct syntax is: `Animation('animationName') { ... }`.".to_string()),
            "The declaration block of `Animation` was expecting an open parenthesis `(` after the keyword `Animation`, but none was found.",
            Some("Ensure that the animation name in the `Animation` declaration is properly closed with a parenthesis `)`. The correct syntax is: `Animation('animationName') { ... }`.".to_string()),
            "The `Animation` declaration is missing a closing parenthesis `)` after the animation name.",
            |parser| parser.parse_string_literal(
                Some("All `Animation` declarations must have a non-empty string as a name. The name should contain only alphanumeric characters, with the first character being a letter. The correct syntax is: `Animation('animationName') { ... }`.".to_string()),
                "The `Animation` declaration must receive a name that is a non-empty string, but no animation name was found.",
                true
            ),
        )?;

        if !self.is_valid_identifier(&animation_name) {
            return Err(NenyrError::new(
                Some("A valid animation name should contain only alphanumeric characters, with the first character being an alphabetic letter. Examples: `'myAnimationName01'`, `'animationName01'`, etc.".to_string()),
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error("The validation of the animation name failed. The provided name does not meet the required format."),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        return Ok(animation_name);
    }

    /// Processes the block of the animation which contains different animation patterns.
    ///
    /// # Syntax
    /// The block should follow the animation declaration with curly braces, containing
    /// the animation patterns:
    ///
    /// ```nenyr
    /// Declare Animation('animationName') {
    ///     From({ /* definition */ }),
    ///     To({ /* definition */ })
    /// }
    /// ```
    ///
    /// # Returns
    /// Returns a tuple with the animation name and the `NenyrAnimation` containing all
    /// parsed patterns and blocks.
    ///
    /// # Errors
    /// Will return a `NenyrError` if:
    /// - Curly braces are not properly opened or closed.
    /// - There are issues with the internal structure of the animation block, such as
    ///   missing commas or invalid pattern declarations.
    fn process_animation_block(
        &mut self,
        animation_name: &str,
    ) -> NenyrResult<(String, NenyrAnimation)> {
        let mut animation = NenyrAnimation::new(animation_name.to_string());

        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the `{}` animation inner block to ensure proper syntax. The parser expects every pattern block to follow valid delimiters. Example: `Declare Animation('{}') {{ Progressive({{ ... }}), Progressive({{ ... }}), ... }}`.", animation_name, animation_name)),
            &format!("A duplicated comma was found inside the `{}` animation block. The parser expected to find a new pattern block, but it was not found.", animation_name),
            Some(format!("Ensure that a comma is placed after each block definition inside the `{}` animation to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Declare Animation('{}') {{ From({{ ... }}), Halfway({{ ... }}), To({{ ... }}) }}`.", animation_name, animation_name)),
            &format!("All patterns inside the `{}` animation block must be separated by commas. A comma is missing after the pattern block definition. The parser expected a comma to separate elements but did not find one.", animation_name),
            || self.processing_state.is_block_active(),
            |is_active| self.processing_state.set_block_active(is_active),
            {
                self.process_animation_patterns(animation_name, &mut animation)?;
            }
        );

        self.processing_state.set_block_active(false);

        Ok((animation_name.to_string(), animation))
    }

    /// Processes the individual patterns within an animation block.
    /// Animation patterns include:
    /// - `Fraction`
    /// - `Progressive`
    /// - `From`
    /// - `Halfway`
    /// - `To`
    ///
    /// Each pattern must be defined correctly, and the parser ensures that commas
    /// separate each pattern, and that the proper delimiters are used for the
    /// pattern blocks.
    ///
    /// # Syntax
    /// Example of valid animation patterns:
    ///
    /// ```nenyr
    /// Declare Animation('myAnimation') {
    ///     Fraction([25, 50], { /* ... */ }),
    ///     Progressive({ /* ... */ }),
    ///     From({ /* ... */ }),
    ///     Halfway({ /* ... */ }),
    ///     To({ /* ... */ })
    /// }
    /// ```
    ///
    /// # Errors
    /// Will return a `NenyrError` if:
    /// - Invalid patterns are declared.
    /// - Incorrect commas or delimiters are found within the animation block.
    fn process_animation_patterns(
        &mut self,
        animation_name: &str,
        animation: &mut NenyrAnimation,
    ) -> NenyrResult<()> {
        self.processing_state.set_block_active(true);

        match self.current_token {
            NenyrTokens::Fraction => {
                if !animation.set_animation_kind(NenyrAnimationKind::Fraction) {
                    self.throw_animation_type_error(
                        animation_name,
                        NenyrTokens::Fraction,
                        animation.get_animation_kind(),
                    )?;
                }

                self.handle_faction_block(
                    animation_name,
                    animation,
                    &NenyrSubAnimationKind::Fraction,
                )
            }
            NenyrTokens::Progressive => {
                if !animation.set_animation_kind(NenyrAnimationKind::Progressive) {
                    self.throw_animation_type_error(
                        animation_name,
                        NenyrTokens::Progressive,
                        animation.get_animation_kind(),
                    )?;
                }

                animation.increment_progressive_count();

                return self.process_parenthesized_curly_bracketed_section(
                    animation_name,
                    animation,
                    &NenyrSubAnimationKind::Progressive,
                );
            }
            NenyrTokens::From => {
                if !animation.set_animation_kind(NenyrAnimationKind::Transitive) {
                    self.throw_animation_type_error(
                        animation_name,
                        NenyrTokens::From,
                        animation.get_animation_kind(),
                    )?;
                }

                self.process_parenthesized_curly_bracketed_section(
                    animation_name,
                    animation,
                    &NenyrSubAnimationKind::From,
                )
            }
            NenyrTokens::Halfway => {
                if !animation.set_animation_kind(NenyrAnimationKind::Transitive) {
                    self.throw_animation_type_error(
                        animation_name,
                        NenyrTokens::Halfway,
                        animation.get_animation_kind(),
                    )?;
                }

                self.process_parenthesized_curly_bracketed_section(
                    animation_name,
                    animation,
                    &NenyrSubAnimationKind::Halfway,
                )
            }
            NenyrTokens::To => {
                if !animation.set_animation_kind(NenyrAnimationKind::Transitive) {
                    self.throw_animation_type_error(
                        animation_name,
                        NenyrTokens::To,
                        animation.get_animation_kind(),
                    )?;
                }

                self.process_parenthesized_curly_bracketed_section(
                    animation_name,
                    animation,
                    &NenyrSubAnimationKind::To,
                )
            }
            _ => {
                return Err(NenyrError::new(
                    Some(format!("Fix or remove the invalid pattern declaration. Only valid and permitted patterns are allowed within the `{}` animation declaration. Please refer to the documentation to verify which patterns are permitted inside animations. Example: `Declare Animation('{}') {{ Fraction([25, 50], {{ ... }}), Fraction([75, 100], {{ ... }}), ... }}`.", animation_name, animation_name)),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error(&format!("The `{}` animation contains an invalid pattern statement. Please ensure that all methods within the animation are correctly defined and formatted.", animation_name)),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }
    }

    /// Throws an error when the animation type is incorrect or mismatched.
    ///
    /// # Parameters
    /// - `animation_name`: The name of the animation being processed.
    /// - `current_token`: The token representing the animation pattern (e.g., `Fraction`, `Progressive`).
    /// - `animation_kind`: The current kind of the animation (e.g., `Fraction`, `Transitive`).
    ///
    /// # Returns
    /// Returns a `NenyrError` indicating a mismatch in animation types.
    fn throw_animation_type_error(
        &mut self,
        animation_name: &str,
        current_token: NenyrTokens,
        animation_kind: NenyrAnimationKind,
    ) -> NenyrResult<()> {
        Err(NenyrError::new(
            Some(format!("The type of an animation is determined by the first pattern statement it receives. For instance, if an animation begins with a `Fraction` pattern, it can only accept other `Fraction` patterns within its block. Similarly, an animation that starts with a `Progressive` pattern will only accept `Progressive` patterns as children. The patterns `From`, `Halfway`, and `To` represent the same animation type and can be used together inside an animation block. To resolve this issue, ensure that the `{}` animation only contains patterns matching its initial type or modify the first pattern to align with the intended type.", animation_name)),
            self.context_name.clone(),
            self.context_path.to_string(),
            format!("The `{}` animation is of type `{:?}`, and it cannot receive a different statement type. Assigning a `{:?}` pattern to the `{}` animation is invalid because its type was already defined by a different pattern statement.", animation_name, animation_kind, current_token, animation_name),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Processes a section of an animation defined by both parenthesis
    /// and curly brackets `({})`. This function handles the parsing of
    /// patterns within animations, ensuring that each pattern is correctly
    /// enclosed and follows proper syntax.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation being processed.
    /// * `animation` - A mutable reference to the `NenyrAnimation` struct,
    ///   where the parsed animation details will be stored.
    /// * `sub_animation_kind` - The kind of sub-animation being processed
    ///   (e.g., `Progressive`, `From`, `Halfway`).
    ///
    /// # Errors
    ///
    /// Returns an error if there is a syntax issue, such as missing or
    /// mismatched parentheses or curly brackets, or invalid animation patterns.
    fn process_parenthesized_curly_bracketed_section(
        &mut self,
        animation_name: &str,
        animation: &mut NenyrAnimation,
        sub_animation_kind: &NenyrSubAnimationKind,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some(format!("Ensure that all patterns inside the `{}` animation block declaration are enclosed with both an opening and closing parenthesis. Correct syntax example: `Animation('{}') {{ Progressive({{ ... }}), Progressive({{ ... }}), ... }}`.", animation_name, animation_name)),
            &format!("One of the patterns in the `{}` animation is missing an open parenthesis `(` after the pattern keyword declaration. The parser expected a parenthesis to begin the pattern definition.", animation_name),
            Some(format!("Ensure that all patterns within the `{}` animation block have both an opening and a closing parenthesis. The syntax should follow the correct format, such as `Animation('{}') {{ From({{ ... }}), Halfway({{ ... }}), ... }}`.", animation_name, animation_name)),
            &format!("A closing parenthesis `)` is missing for one of the patterns in the `{}` animation. The parser expected a closing parenthesis to properly end the pattern declaration.", animation_name),
            |parser| {
                parser.parse_curly_bracketed_delimiter(
                    Some(format!("After the open parenthesis, an opening curly bracket `{{` is required to properly define the properties block in `{}` animation. Ensure the pattern follows the correct Nenyr syntax, such as `Animation('{}') {{ From({{ ... }}), Halfway({{ ... }}), ... }}`.", animation_name, animation_name)),
                    &format!("One of the patterns in the `{}` animation was expected to receive an object as a value, but an opening curly bracket `{{` was not found after the open parenthesis.", animation_name),
                    Some(format!("Ensure that the properties block within the pattern in `{}` animation is properly closed with a closing curly bracket `}}`. The correct syntax should look like: `Animation('{}') {{ Progressive({{ ... }}), Progressive({{ ... }}), ... }}`.", animation_name, animation_name)),
                    &format!("One of the patterns in the `{}` animation is missing a closing curly bracket `}}` to properly close the properties block.", animation_name),
                    |parser| {
                        parser.process_animation_children(
                            animation_name,
                            &None,
                            animation,
                            sub_animation_kind,
                        )
                    },
                )?;

                parser.process_next_token()
            },
        )
    }

    /// Processes a `Fraction` block in an animation. Fractions are used to
    /// define specific points in the animation timeline (e.g., 25%, 50%, etc.).
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation being processed.
    /// * `animation` - A mutable reference to the `NenyrAnimation` struct,
    ///   where the fraction stops will be recorded.
    /// * `sub_animation_kind` - The type of sub-animation for the fraction block.
    ///
    /// # Errors
    ///
    /// Returns an error if there is a syntax issue, such as missing parentheses
    /// or invalid stops.
    fn handle_faction_block(
        &mut self,
        animation_name: &str,
        animation: &mut NenyrAnimation,
        sub_animation_kind: &NenyrSubAnimationKind,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some(format!("Ensure that all fraction patterns inside the `{}` animation block declaration are enclosed with both an opening and closing parenthesis. Correct syntax example: `Animation('{}') {{ Fraction([25, 50], {{ ... }}), Fraction([75, 100], {{ ... }}), ... }}`.", animation_name, animation_name)),
            &format!("One of the fraction patterns in the `{}` animation is missing an open parenthesis `(` after the pattern keyword declaration. The parser expected a parenthesis to begin the fraction pattern definition.", animation_name),
            Some(format!("Ensure that all fraction patterns within the `{}` animation block have both an opening and a closing parenthesis. The syntax should follow the correct format, such as `Animation('{}') {{ Fraction(10.500, {{ ... }}), Fraction(100, {{ ... }}), ... }}`.", animation_name, animation_name)),
            &format!("A closing parenthesis `)` is missing for one of the fraction patterns in the `{}` animation. The parser expected a closing parenthesis to properly end the pattern declaration.", animation_name),
            |parser| {
                let stops = parser.process_faction_stops(animation_name)?;

                parser.process_comma_after_fraction_stops(
                    animation_name,
                    &stops,
                    animation,
                    sub_animation_kind,
                )
            },
        )
    }

    /// Processes the stops for a `Fraction` block, which define the points in
    /// the animation timeline (e.g., 25%, 50%, etc.). This function can handle
    /// both individual stop values and vectors of stops (e.g., [10, 25, 50]).
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation being processed.
    ///
    /// # Returns
    ///
    /// A vector of float values representing the stops in the animation.
    /// If no valid stops are found, the function returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an error if the stops are not valid numeric values or if there
    /// are syntax issues in the stop declaration.
    fn process_faction_stops(&mut self, animation_name: &str) -> NenyrResult<Option<Vec<f64>>> {
        match self.current_token {
            NenyrTokens::Number(stop) => Ok(Some(vec![stop])),
            NenyrTokens::SquareBracketOpen => {
                let stops = self.process_f64_vector(animation_name)?;

                Ok(Some(stops))
            }
            _ => {
                return Err(NenyrError::new(
                    Some(format!("Ensure that all stops in the `{}` animation are valid numeric values, either a single float or integer, or a vector of numeric values. Stops define the points in the animation timeline, and must be numeric to function correctly. Examples of valid stops include a single integer like `10`, a float like `15.5`, or a vector of values such as `[10, 15, 20.5]`. Use the following syntax to correctly define stops: `Animation('{}') {{ Fraction(10, {{ ... }}) }}` or `Animation('{}') {{ Fraction([10, 15.5, 20], {{ ... }}), ... }}`.", animation_name, animation_name, animation_name)),
                    self.context_name.clone(),
                    self.context_path.to_string(),
                    self.add_nenyr_token_to_error(&format!("The `{}` animation contains an invalid stop value. Each stop must be a numeric value (either float or integer), or a vector of numeric values. Non-numeric or empty values are not allowed.", animation_name)),
                    NenyrErrorKind::SyntaxError,
                    self.get_tracing(),
                ));
            }
        }
    }

    /// Parses a vector of `f64` values from the input token stream.
    /// This function ensures that the vector is properly delimited by square brackets `[]`
    /// and that the values within are numeric.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation being processed.
    ///
    /// # Returns
    ///
    /// A vector of `f64` values representing numeric stops for the animation.
    ///
    /// # Errors
    ///
    /// Returns an error if the values inside the square brackets are non-numeric
    /// or if there is an issue with the syntax of the stops.
    fn process_f64_vector(&mut self, animation_name: &str) -> NenyrResult<Vec<f64>> {
        self.process_next_token()?;

        let mut stops: Vec<f64> = Vec::new();

        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the stops statement of the fraction patterns block in the `{}` animation. Ensure proper syntax by following valid delimiters. Example: `Animation('{}') {{ Fraction([25, 50], {{ ... }}), Fraction([75, 100], {{ ... }}), ... }}`.", animation_name, animation_name)),
            &format!("A duplicated comma was found in the stops statement of one of the fraction patterns in the `{}` animation. The parser expected to find a new stop statement or a closing square bracket `]`, but none was found.", animation_name),
            Some(format!("Ensure that a comma is placed after each stop definition inside the fraction patterns statement in the `{}` animation to separate elements correctly. Proper syntax is required for the parser to process the animation. Example: `Animation('{}') {{ Fraction([25, 50], {{ ... }}), Fraction([75, 100], {{ ... }}), ... }}`.", animation_name, animation_name)),
            &format!("All stops in fraction patterns inside the `{}` animation block must be separated by commas. A comma is missing in the stops statement of the fraction patterns definition. The parser expected a comma to separate elements but did not find one.", animation_name),
            || self.processing_state.is_nested_block_active(),
            |is_active| self.processing_state.set_nested_block_active(is_active),
            {
                self.processing_state.set_nested_block_active(true);

                match self.current_token {
                    NenyrTokens::Number(stop) => stops.push(stop),
                    _ => {
                        return Err(NenyrError::new(
                            Some(format!("Ensure that all stops in the `{}` animation are valid numeric values, either a single float or integer, or a vector of numeric values. Stops define the points in the animation timeline, and must be numeric to function correctly. Examples of valid stops include a single integer like `10`, a float like `15.5`, or a vector of values such as `[10, 15, 20.5]`. Use the following syntax to correctly define stops: `Animation('{}') {{ Fraction(10, {{ ... }}) }}` or `Animation('{}') {{ Fraction([10, 15.5, 20], {{ ... }}), ... }}`.", animation_name, animation_name, animation_name)),
                            self.context_name.clone(),
                            self.context_path.to_string(),
                            self.add_nenyr_token_to_error(&&format!("The `{}` animation contains an invalid stop value. Each stop must be a numeric value (either float or integer), or a vector of numeric values. Non-numeric or empty values are not allowed.", animation_name)),
                            NenyrErrorKind::SyntaxError,
                            self.get_tracing(),
                        ));
                    }
                }
            }
        );

        self.processing_state.set_nested_block_active(false);

        Ok(stops)
    }

    /// Processes a comma after a fraction stop in a Nenyr animation block.
    ///
    /// This function expects a comma to follow the fraction stops in an animation pattern.
    /// If a comma is found, it continues to process the properties block associated with the
    /// fraction. It also validates the presence of curly brackets `{}` and ensures the correct
    /// syntax of the animation definition.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation being processed.
    /// * `stops` - A reference to an optional vector of fraction stops for the animation.
    /// * `animation` - A mutable reference to the NenyrAnimation object being built.
    /// * `sub_animation_kind` - A reference to the kind of sub-animation being processed.
    ///
    /// # Returns
    ///
    /// Returns `NenyrResult<()>`, an Ok result if the parsing was successful, or an error if there
    /// was a syntax issue.
    fn process_comma_after_fraction_stops(
        &mut self,
        animation_name: &str,
        stops: &Option<Vec<f64>>,
        animation: &mut NenyrAnimation,
        sub_animation_kind: &NenyrSubAnimationKind,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        if let NenyrTokens::Comma = self.current_token {
            self.process_next_token()?;

            self.parse_curly_bracketed_delimiter(
                Some(format!("After the stops statement, an opening curly bracket `{{` is required to properly define the properties block in `{}` animation. Ensure the pattern follows the correct Nenyr syntax, such as `Animation('{}') {{ Fraction([21.10, 22, 30], {{ ... }}), ... }}`.", animation_name, animation_name)),
                    &format!("One of the fraction patterns in the `{}` animation was expected to receive an object of properties as the second parameter, but an opening curly bracket `{{` was not found after the stops declaration.", animation_name),
                    Some(format!("Ensure that the properties block within the fraction pattern in `{}` animation is properly closed with a closing curly bracket `}}`. The correct syntax should look like: `Animation('{}') {{ Fraction([10, 50], {{ ... }}), Fraction([75, 100], {{ ... }}), ... }}`.", animation_name, animation_name)),
                    &format!("One of the fraction patterns in the `{}` animation is missing a closing curly bracket `}}` to properly close the properties block.", animation_name),
                |parser| {
                    parser.process_animation_children(
                        animation_name,
                        stops,
                        animation,
                        sub_animation_kind,
                    )
                },
            )?;

            return self.process_next_token();
        }

        Err(NenyrError::new(
            Some(format!("Ensure that a comma is placed after the stop statement of the fraction patterns in the `{}` animation to follow proper syntax. The parser expects commas to separate elements in the declaration. For example, after specifying the stops like `Fraction([10, 15], {{ ... }})`, place a comma after the stops: `Fraction([10, 15], {{ propertyName: 'value', ... }})`. This ensures correct parsing and avoids syntax errors.", animation_name)),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&format!("A comma was expected after the stops declaration of the fraction patterns in the `{}` animation, but none was found. The parser requires a comma to separate the stop values from the subsequent object of properties.", animation_name)),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Processes the child properties of an animation in the Nenyr DSL.
    ///
    /// This function iterates through the properties block of an animation pattern, validating
    /// and collecting properties into keyframes for the animation.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation being processed.
    /// * `stops` - A reference to an optional vector of fraction stops.
    /// * `animation` - A mutable reference to the NenyrAnimation object being built.
    /// * `sub_animation_kind` - A reference to the kind of sub-animation being processed.
    ///
    /// # Returns
    ///
    /// Returns `NenyrResult<()>`, indicating whether the child properties were successfully processed.
    ///
    /// # Errors
    ///
    /// This function returns a `NenyrError` if:
    /// - There are syntax issues with property declarations.
    /// - Duplicate commas are found in the properties block.
    fn process_animation_children(
        &mut self,
        animation_name: &str,
        stops: &Option<Vec<f64>>,
        animation: &mut NenyrAnimation,
        sub_animation_kind: &NenyrSubAnimationKind,
    ) -> NenyrResult<()> {
        let mut keyframe: IndexMap<String, String> = IndexMap::new();

        loop_while_not!(
            self,
            Some(format!("Remove any duplicated commas from the properties block of the patterns statement in the `{}` animation. Ensure proper syntax by following valid delimiters. Example: `Animation('{}') {{ Progressive({{ backgroundColor: 'blue', border: '1px solid red', ... }}), ... }}`.", animation_name, animation_name)),
            &format!("A duplicated comma was found in the properties block of one of the patterns in the `{}` animation. The parser expected to find a new property statement but none was found.", animation_name),
            Some(format!("Ensure that a comma is placed after each property definition inside the patterns statement in the `{}` animation to separate elements correctly. Proper syntax is required for the parser to process the animation. Example: `Animation('{}') {{ Fraction(50, {{ backgroundColor: 'blue', border: '1px solid red', ... }}), ... }}`.", animation_name, animation_name)),
            &format!("All properties blocks in patterns inside the `{}` animation block must be separated by commas. A comma is missing in the properties block of the patterns definition. The parser expected a comma to separate elements but did not find one.", animation_name),
            || self.processing_state.is_complementary_block_active(),
            |is_active| self
                .processing_state
                .set_complementary_block_active(is_active),
            {
                self.process_animation_property(animation_name, &mut keyframe)?;
            }
        );

        // Create the animation inside the `animation`.
        animation.add_animation_keyframe(sub_animation_kind, stops, keyframe);
        self.processing_state.set_complementary_block_active(false);

        Ok(())
    }

    /// Processes a single property inside an animation's keyframe in the Nenyr DSL.
    ///
    /// This function identifies valid properties and assigns their values to the keyframe.
    /// It supports both standard Nenyr properties and aliases defined within Nenyr.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation being processed.
    /// * `keyframe` - A mutable reference to the keyframe where properties will be added.
    ///
    /// # Returns
    ///
    /// Returns `NenyrResult<()>`, indicating successful parsing of the property.
    ///
    /// # Errors
    ///
    /// This function returns a `NenyrError` if:
    /// - The property is not a valid property or alias.
    fn process_animation_property(
        &mut self,
        animation_name: &str,
        keyframe: &mut IndexMap<String, String>,
    ) -> NenyrResult<()> {
        self.processing_state.set_complementary_block_active(true);

        if let Some(property) = self.convert_nenyr_property_to_css_property(&self.current_token) {
            return self.process_animation_value(animation_name, property, keyframe);
        } else if let NenyrTokens::Identifier(nickname) = self.current_token.clone() {
            return self.process_animation_value(
                animation_name,
                format!("nickname;{}", nickname),
                keyframe,
            );
        }

        Err(NenyrError::new(
            Some(format!("Ensure that all properties inside the patterns in the `{}` animation are either an alias or a valid property. Please verify the documentation to know which properties are valid inside the animation patterns.", animation_name)),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&format!("One of the properties inside one of the patterns in the `{}` animation is not either an alias or a valid property.", animation_name)),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Parses and assigns a value to a given property in an animation keyframe.
    ///
    /// This function processes the value of a property, ensuring proper syntax
    /// such as a colon separator between the property and its value. It validates the value
    /// format and ensures the property is correctly added to the keyframe.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation being processed.
    /// * `property` - The property to which a value will be assigned.
    /// * `keyframe` - A mutable reference to the keyframe where the property-value pair will be stored.
    ///
    /// # Returns
    ///
    /// Returns `NenyrResult<()>` if the value is valid and correctly parsed.
    ///
    /// # Errors
    ///
    /// This function returns a `NenyrError` if:
    /// - A colon is missing after the property.
    /// - The value is invalid or malformed.
    fn process_animation_value(
        &mut self,
        animation_name: &str,
        property: String,
        keyframe: &mut IndexMap<String, String>,
    ) -> NenyrResult<()> {
        self.process_next_token()?;
        self.parse_colon_delimiter(
            Some(format!("Ensure that each property is defined with a colon after it. The correct syntax is: `pattern({{ {}: 'property value', ... }})`.", &property)),
            &format!("The `{}` property inside one of the patterns in the `{}` animation is missing a colon after the property keyword definition.", &property, animation_name),
            true
        )?;

        let value = self.parse_string_literal(
            Some(format!("Ensure that all properties are assigned non-empty string values. You can either remove the property or specify a non-empty string value for it: `pattern({{ {}: 'property value', ... }})`.", &property)),
            &format!("The `{}` property inside one of the patterns in the `{}` animation should receive a non-empty string as a value, but none was found.", &property, animation_name),
            false
        )?;

        if self.is_valid_style_syntax(&value) {
            keyframe.insert(property, value);

            return Ok(());
        }

        Err(NenyrError::new(
            Some(format!("Ensure that all values are semantically correct to be validated. Please refer to the documentation to verify the correct way to define values.")),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(&format!("The `{}` property inside one of the patterns in the `{}` animation contains an invalid value, and it could not be validated.", &property, animation_name)),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn animation_fraction_is_valid() {
        let raw_nenyr = "Animation('giddyRespond') {
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
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Ok((\"giddyRespond\", NenyrAnimation { animation_name: \"giddyRespond\", kind: Some(Fraction), progressive_count: None, keyframe: [Fraction { stops: [30.0], properties: {\"bgd\": \"${accentColorVar}\", \"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"} }, Fraction { stops: [40.0], properties: {\"bgd\": \"${accentColorVar}\"} }, Fraction { stops: [4.0], properties: {\"bgd\": \"${accentColorVar}\"} }, Fraction { stops: [50.0, 70.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [5.0, 7.0], properties: {\"background-color\": \"blue\"} }, Fraction { stops: [70.0, 80.0, 100.0], properties: {\"transform\": \"translate(50%, 50%)\"} }] }))".to_string()
        );
    }

    #[test]
    fn animation_progressive_is_valid() {
        let raw_nenyr = "Animation('spiritedSavings') {
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
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Ok((\"spiritedSavings\", NenyrAnimation { animation_name: \"spiritedSavings\", kind: Some(Progressive), progressive_count: Some(3), keyframe: [Progressive({\"width\": \"${myVar}\"}), Progressive({\"border\": \"10px solid red\", \"background-color\": \"blue\", \"height\": \"100px\", \"width\": \"200px\"}), Progressive({\"background-color\": \"pink\"})] }))".to_string()
        );
    }

    #[test]
    fn animation_from_is_valid() {
        let raw_nenyr = "Animation('grotesquePtarmigan') {
        From({
            width: '${myVar}'
        })
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Ok((\"grotesquePtarmigan\", NenyrAnimation { animation_name: \"grotesquePtarmigan\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"width\": \"${myVar}\"})] }))".to_string()
        );
    }

    #[test]
    fn animation_halfway_is_valid() {
        let raw_nenyr = "Animation('grotesquePtarmigan') {
        Halfway({
            border: '1px solid red'
        })
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Ok((\"grotesquePtarmigan\", NenyrAnimation { animation_name: \"grotesquePtarmigan\", kind: Some(Transitive), progressive_count: None, keyframe: [Halfway({\"border\": \"1px solid red\"})] }))".to_string()
        );
    }

    #[test]
    fn animation_to_is_valid() {
        let raw_nenyr = "Animation('grotesquePtarmigan') {
        To({
            backgroundColor: 'pink',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        })
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Ok((\"grotesquePtarmigan\", NenyrAnimation { animation_name: \"grotesquePtarmigan\", kind: Some(Transitive), progressive_count: None, keyframe: [To({\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"})] }))".to_string()
        );
    }

    #[test]
    fn animation_transitive_is_valid() {
        let raw_nenyr = "Animation('grotesquePtarmigan') {
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
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Ok((\"grotesquePtarmigan\", NenyrAnimation { animation_name: \"grotesquePtarmigan\", kind: Some(Transitive), progressive_count: None, keyframe: [From({\"width\": \"${myVar}\"}), Halfway({\"border\": \"1px solid red\"}), To({\"background-color\": \"blue\", \"border\": \"10px solid red\", \"height\": \"100px\", \"width\": \"200px\"})] }))".to_string()
        );
    }

    #[test]
    fn animation_fraction_is_not_valid() {
        let raw_nenyr = "Animation('giddyRespond') {
        Fraction(30, {
            // Este é um comentário de linha.
            bgd: '${accentColorVar}',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        }),
        Fraction 40, {
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
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Err(NenyrError { suggestion: Some(\"Ensure that all fraction patterns inside the `giddyRespond` animation block declaration are enclosed with both an opening and closing parenthesis. Correct syntax example: `Animation('giddyRespond') { Fraction([25, 50], { ... }), Fraction([75, 100], { ... }), ... }`.\"), context_name: None, context_path: \"\", error_message: \"One of the fraction patterns in the `giddyRespond` animation is missing an open parenthesis `(` after the pattern keyword declaration. The parser expected a parenthesis to begin the fraction pattern definition. However, found `Number(40.0)` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        }),\"), line_after: Some(\"            // Este é um comentário de linha.\"), error_line: Some(\"        Fraction 40, {\"), error_on_line: 10, error_on_col: 20, error_on_pos: 299 } })".to_string()
        );
    }

    #[test]
    fn animation_progressive_is_not_valid() {
        let raw_nenyr = "Animation('spiritedSavings') {
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
        Progressive(
            backgroundColor: 'pink'
        }),
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Err(NenyrError { suggestion: Some(\"After the open parenthesis, an opening curly bracket `{` is required to properly define the properties block in `spiritedSavings` animation. Ensure the pattern follows the correct Nenyr syntax, such as `Animation('spiritedSavings') { From({ ... }), Halfway({ ... }), ... }`.\"), context_name: None, context_path: \"\", error_message: \"One of the patterns in the `spiritedSavings` animation was expected to receive an object as a value, but an opening curly bracket `{` was not found after the open parenthesis. However, found `BackgroundColor` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        Progressive(\"), line_after: Some(\"        }),\"), error_line: Some(\"            backgroundColor: 'pink'\"), error_on_line: 13, error_on_col: 28, error_on_pos: 345 } })".to_string()
        );
    }

    #[test]
    fn animation_from_is_not_valid() {
        let raw_nenyr = "Animation('grotesquePtarmigan') {
        From({
            width '${myVar}'
        })
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Err(NenyrError { suggestion: Some(\"Ensure that each property is defined with a colon after it. The correct syntax is: `pattern({ width: 'property value', ... })`.\"), context_name: None, context_path: \"\", error_message: \"The `width` property inside one of the patterns in the `grotesquePtarmigan` animation is missing a colon after the property keyword definition. However, found `StringLiteral(\\\"${myVar}\\\")` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"        From({\"), line_after: Some(\"        })\"), error_line: Some(\"            width '${myVar}'\"), error_on_line: 3, error_on_col: 29, error_on_pos: 77 } })".to_string()
        );
    }

    #[test]
    fn animation_halfway_is_not_valid() {
        let raw_nenyr = "Animation('grotesquePtarmigan') {
        Halfway0({
            border: '1px solid red'
        })
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Err(NenyrError { suggestion: Some(\"Fix or remove the invalid pattern declaration. Only valid and permitted patterns are allowed within the `grotesquePtarmigan` animation declaration. Please refer to the documentation to verify which patterns are permitted inside animations. Example: `Declare Animation('grotesquePtarmigan') { Fraction([25, 50], { ... }), Fraction([75, 100], { ... }), ... }`.\"), context_name: None, context_path: \"\", error_message: \"The `grotesquePtarmigan` animation contains an invalid pattern statement. Please ensure that all methods within the animation are correctly defined and formatted. However, found `Halfway0` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"Animation('grotesquePtarmigan') {\"), line_after: Some(\"            border: '1px solid red'\"), error_line: Some(\"        Halfway0({\"), error_on_line: 2, error_on_col: 17, error_on_pos: 50 } })".to_string()
        );
    }

    #[test]
    fn animation_to_is_not_valid() {
        let raw_nenyr = "Animation('grotesquePtarmigan' {
        To({
            backgroundColor: 'pink',
            backgroundColor: 'blue',
            border: '10px solid red',
            height: '100px',
            width: '200px'
        })
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Err(NenyrError { suggestion: Some(\"Ensure that the animation name in the `Animation` declaration is properly closed with a parenthesis `)`. The correct syntax is: `Animation('animationName') { ... }`.\"), context_name: None, context_path: \"\", error_message: \"The `Animation` declaration is missing a closing parenthesis `)` after the animation name. However, found `{` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: Some(\"        To({\"), error_line: Some(\"Animation('grotesquePtarmigan' {\"), error_on_line: 1, error_on_col: 33, error_on_pos: 32 } })".to_string()
        );
    }

    #[test]
    fn animation_transitive_is_not_valid() {
        let raw_nenyr = "Animation('') {
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
    }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Err(NenyrError { suggestion: Some(\"All `Animation` declarations must have a non-empty string as a name. The name should contain only alphanumeric characters, with the first character being a letter. The correct syntax is: `Animation('animationName') { ... }`.\"), context_name: None, context_path: \"\", error_message: \"The `Animation` declaration must receive a name that is a non-empty string, but no animation name was found. However, found `StringLiteral(\\\"\\\")` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: None, line_after: Some(\"        From({\"), error_line: Some(\"Animation('') {\"), error_on_line: 1, error_on_col: 13, error_on_pos: 12 } })".to_string()
        );
    }

    #[test]
    fn animation_with_different_types_is_not_valid() {
        let raw_nenyr = "Animation('spiritedSavings') {
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
            Progressive(
                backgroundColor: 'pink'
            }),
            Fraction(30, {
                // Este é um comentário de linha.
                bgd: '${accentColorVar}',
                backgroundColor: 'blue',
                border: '10px solid red',
                height: '100px',
                width: '200px'
            })
        }";
        let mut parser = NenyrParser::new(raw_nenyr, "");

        let _ = parser.process_next_token();

        assert_eq!(
            format!("{:?}", parser.process_animation_method()),
            "Err(NenyrError { suggestion: Some(\"After the open parenthesis, an opening curly bracket `{` is required to properly define the properties block in `spiritedSavings` animation. Ensure the pattern follows the correct Nenyr syntax, such as `Animation('spiritedSavings') { From({ ... }), Halfway({ ... }), ... }`.\"), context_name: None, context_path: \"\", error_message: \"One of the patterns in the `spiritedSavings` animation was expected to receive an object as a value, but an opening curly bracket `{` was not found after the open parenthesis. However, found `BackgroundColor` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"            Progressive(\"), line_after: Some(\"            }),\"), error_line: Some(\"                backgroundColor: 'pink'\"), error_on_line: 13, error_on_col: 32, error_on_pos: 393 } })".to_string()
        );
    }
}
