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

    fn throw_animation_type_error(
        &mut self,
        animation_name: &str,
        current_kind: NenyrTokens,
        animation_kind: NenyrAnimationKind,
    ) -> NenyrResult<()> {
        Err(NenyrError::new(
            Some(format!("The type of an animation is determined by the first pattern statement it receives. For instance, if an animation begins with a `Fraction` pattern, it can only accept other `Fraction` patterns within its block. Similarly, an animation that starts with a `Progressive` pattern will only accept `Progressive` patterns as children. The patterns `From`, `Halfway`, and `To` represent the same animation type and can be used together inside an animation block. To resolve this issue, ensure that the `{}` animation only contains patterns matching its initial type or modify the first pattern to align with the intended type.", animation_name)),
            self.context_name.clone(),
            self.context_path.to_string(),
            format!("The `{}` animation is of type `{:?}`, and it cannot receive a different statement type. Assigning a `{:?}` pattern to the `{}` animation is invalid because its type was already defined by a different pattern statement.", animation_name, animation_kind, current_kind, animation_name),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

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

    fn process_animation_property(
        &mut self,
        animation_name: &str,
        keyframe: &mut IndexMap<String, String>,
    ) -> NenyrResult<()> {
        self.processing_state.set_complementary_block_active(true);

        if let Some(property) = self.convert_nenyr_property_to_css_property(&self.current_token) {
            return self.process_animation_value(animation_name, property, keyframe);
        } else if let NenyrTokens::Identifier(nickname) = self.current_token.clone() {
            return self.process_animation_value(animation_name, nickname, keyframe);
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
mod tests {}
