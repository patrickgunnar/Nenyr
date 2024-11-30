use crate::{
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::variables::NenyrVariables,
    validators::variable_value::NenyrVariableValueValidator,
    NenyrParser, NenyrResult,
};

impl NenyrParser {
    /// Processes the `Variables` method within the Nenyr syntax.
    ///
    /// This method begins by checking for the opening parenthesis that should follow
    /// the `Variables` keyword. If this parenthesis is missing, it returns an error
    /// with an appropriate message based on whether the `Variables` block is defined
    /// within a `Themes` context or not.
    ///
    /// The method subsequently parses the contents within the parentheses and expects
    /// to find an opening curly bracket `{` for the properties block. It also ensures
    /// that the corresponding closing parenthesis `)` and closing curly bracket `}`
    /// are present.
    ///
    /// # Parameters
    /// - `is_from_themes`: A boolean indicating whether the `Variables` block is part
    ///   of a `Themes` context, influencing the error messages generated.
    ///
    /// # Returns
    /// - `NenyrResult<NenyrVariables>`: A result type containing the parsed variables
    ///   if successful, or an error if any validation fails.
    ///
    /// # Errors
    /// This function may return errors related to missing or misplaced parentheses and
    /// curly brackets, or incorrect property definitions.
    pub(crate) fn process_variables_method(
        &mut self,
        is_from_themes: bool,
    ) -> NenyrResult<NenyrVariables> {
        self.process_next_token()?;

        let error_message_on_open = if is_from_themes {
            "Within `Themes`, the `Variables` block is missing an opening parenthesis `(` after the `Variables` keyword. The parser expected a parenthesis to begin the variable declarations."
        } else {
            "The `Variables` block is missing an opening parenthesis `(` after the `Variables` keyword. The parser expected an opening parenthesis to begin the variable declarations."
        };

        let error_message_on_close = if is_from_themes {
            "A closing parenthesis `)` is missing for the `Variables` declaration within the `Themes` block. The parser expected a closing parenthesis to properly end the variable declarations."
        } else {
            "A closing parenthesis `)` is missing for the `Variables` declaration block. The parser expected a closing parenthesis to properly end the variable declarations."
        };

        self.parse_parenthesized_delimiter(
            Some("Ensure that the `Variables` declaration block is enclosed with both an opening and a closing parenthesis. Correct syntax example: `Variables({ ... })`.".to_string()),
            error_message_on_open,
            Some("Ensure that the `Variables` block includes both an opening and a closing parenthesis. The syntax should follow the correct format: `Variables({ ... })`.".to_string()),
            error_message_on_close,
            |parser| {
                let error_message_on_open  = if is_from_themes {
                    "Within `Themes`, the `Variables` declaration was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis."
                } else {
                    "The `Variables` declaration block was expected to receive an object as a value, but an opening curly bracket `{` was not found after the opening parenthesis."
                };

                let error_message_on_close  = if is_from_themes {
                    "The `Variables` declaration within `Themes` is missing a closing curly bracket `}` to close the properties block properly."
                } else {
                    "The `Variables` declaration block is missing a closing curly bracket `}` to properly close the properties block."
                };

                let variables = parser.parse_curly_bracketed_delimiter(
                    Some("After the opening parenthesis, an opening curly bracket `{` is required to properly define the properties block in the `Variables` declaration. Ensure the pattern follows correct Nenyr syntax, like `Variables({ key: 'value', ... })`.".to_string()),
                    error_message_on_open,
                    Some("Ensure that the properties block within the `Variables` declaration is properly closed with a closing curly bracket `}`. The correct syntax should look like: `Variables({ key: 'value', ... })`.".to_string()),
                    error_message_on_close,
                    |parser| parser.process_variables_children(is_from_themes),
                )?;

                parser.process_next_token()?;

                Ok(variables)
            },
        )
    }

    /// Processes the child properties of the `Variables` declaration.
    ///
    /// This method validates that each property within the `Variables` block is correctly
    /// formatted and separated by commas. It also checks for any duplicate commas and
    /// reports appropriate error messages based on whether the block is within a `Themes`
    /// context or not.
    ///
    /// # Parameters
    /// - `is_from_themes`: A boolean indicating if the properties are defined within a
    ///   `Themes` context, affecting the generated error messages.
    ///
    /// # Returns
    /// - `NenyrResult<NenyrVariables>`: A result containing the parsed variables if successful,
    ///   or an error if validation fails.
    ///
    /// # Errors
    /// This method may return errors related to duplicate commas or missing commas
    /// between property declarations.
    fn process_variables_children(&mut self, is_from_themes: bool) -> NenyrResult<NenyrVariables> {
        let mut variables = NenyrVariables::new();

        let error_message_on_duplicated = if is_from_themes {
            "A duplicated comma was detected in the properties block of the `Variables` declaration within `Themes`. The parser expected a new property statement but found none."
        } else {
            "A duplicated comma was found in the properties block of the `Variables` declarations. The parser expected to find a new property statement but none was found."
        };

        let error_message_on_missing = if is_from_themes {
            "Within `Themes`, each property in the `Variables` declaration must be separated by a comma. A comma is missing between properties, which is required by the parser to correctly parse the elements."
        } else {
            "The properties in the `Variables` declaration must be separated by commas. A comma is missing between the properties in the `Variables` declaration. The parser expected a comma to separate elements but did not find one."
        };

        loop_while_not!(
            self,
            Some("Remove any duplicated commas from the properties block in the `Variables` declaration. Ensure proper syntax by following valid delimiters. Example: `Variables({ key: 'value', anotherKey: 'anotherValue', ... })`.".to_string()),
            error_message_on_duplicated,
            Some("Ensure that a comma is placed after each property definition inside the `Variables` declaration to separate elements correctly. Proper syntax is required for the parser to process the context. Example: `Variables({ key: 'value', anotherKey: 'anotherValue', ... })`.".to_string()),
            error_message_on_missing,
            || self.processing_state.is_block_active(),
            |is_active| self.processing_state.set_block_active(is_active),
            {
                self.process_variable_identifier(is_from_themes, &mut variables)?;
            }
        );

        self.processing_state.set_block_active(false);

        Ok(variables)
    }

    /// Processes individual variable identifiers within the `Variables` declaration.
    ///
    /// This method validates the identifier format and ensures that it conforms to
    /// the expected naming conventions. It checks for alphanumeric characters and
    /// that the first character is a letter. If the identifier is valid, it proceeds
    /// to parse the associated value.
    ///
    /// # Parameters
    /// - `is_from_themes`: A boolean indicating whether the identifier is being parsed
    ///   within a `Themes` context, influencing the error messages generated.
    /// - `variables`: A mutable reference to the `NenyrVariables` struct where the
    ///   processed variable will be stored.
    ///
    /// # Returns
    /// - `NenyrResult<()>`: A result indicating success or failure. An error is returned
    ///   if the identifier is invalid or if parsing the variable value fails.
    ///
    /// # Errors
    /// This function may return errors related to invalid identifier formatting and
    /// general parsing issues.
    fn process_variable_identifier(
        &mut self,
        is_from_themes: bool,
        variables: &mut NenyrVariables,
    ) -> NenyrResult<()> {
        self.processing_state.set_block_active(true);

        if let NenyrTokens::Identifier(identifier) = self.current_token.clone() {
            return self.process_variable_value(is_from_themes, identifier, variables);
        }

        let error_message = if is_from_themes {
            "In the `Themes` block, the `Variables` declaration contains an invalid identifier for the variable name. Please ensure the identifier follows the required format."
        } else {
            "The `Variables` declaration contains an invalid identifier for the variable name. Please ensure the identifier follows the required format."
        };

        Err(NenyrError::new(
            Some("Specify a valid identifier for the variables that consists only of alphanumeric characters, with the first character being a letter. For example: 'myVariable1', 'exampleVariable', etc.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Processes the value of a variable within a `Variables` or `Themes` declaration.
    ///
    /// This method is responsible for validating and parsing the value assigned to a variable
    /// after the variable's name has been defined. It expects that the variable has a valid
    /// identifier and is invoked within the context of either a general `Variables` block or
    /// a specific `Themes` block.
    ///
    /// # Parameters
    /// - `is_from_themes`: A boolean indicating whether the variable is being processed
    ///   within the context of a `Themes` block. This affects the error messages generated.
    /// - `identifier`: A `String` that represents the name of the variable being defined.
    ///   It is assumed that this variable name has been validated and is valid according
    ///   to the naming conventions of the language.
    /// - `variables`: A mutable reference to a `NenyrVariables` instance, where the
    ///   successfully parsed variable will be added upon validation.
    ///
    /// # Errors
    /// This function can return a `NenyrResult<()>`, which may encapsulate errors
    /// of type `NenyrError` if any of the following conditions are not met:
    /// - The variable declaration is missing a colon (`:`) after the variable name.
    /// - The variable value is expected to be a non-empty string but is not found.
    /// - The variable value does not pass validation checks defined by `is_valid_variable_value`.
    fn process_variable_value(
        &mut self,
        is_from_themes: bool,
        identifier: String,
        variables: &mut NenyrVariables,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        let error_message = if is_from_themes {
            &format!("The `{}` variable declaration in the`Themes` block is missing a colon after the variable name definition.", identifier)
        } else {
            &format!("The `{}` variable in the `Variables` declaration is missing a colon after the variable name definition.", identifier)
        };

        self.parse_colon_delimiter(
            Some(format!("Ensure that each variable is defined with a colon after it. The correct syntax is: `Variables({{ {}: 'variable value', ... }})`.", identifier)),
            error_message,
            true
        )?;

        let error_message = if is_from_themes {
            &format!("The `{}` variable declaration in the `Themes` block should receive a non-empty string as a value, but none was found.", identifier)
        } else {
            &format!("The `{}` variable in the `Variables` declaration should receive a non-empty string as a value, but none was found.", identifier)
        };

        let value = self.parse_string_literal(
            Some(format!("Ensure that all variables are assigned non-empty string values. You can either remove the variable or specify a non-empty string value for it: `Variables({{ {}: 'variable value', ... }})`.", identifier)),
            error_message,
            false
        )?;

        if self.is_valid_variable_value(&value) {
            variables.add_variable(identifier, value);

            return Ok(());
        }

        let error_message = if is_from_themes {
            &format!("In the `Themes` block, the `{}` variable declaration contains an invalid value and could not be validated.", identifier)
        } else {
            &format!("The `{}` variable in the `Variables` declaration contains an invalid value and could not be validated.", identifier)
        };

        Err(NenyrError::new(
            Some("Ensure that all variable values are semantically correct to be validated. Please refer to the documentation to verify the correct way to define variable values.".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn variables_are_valid() {
        let raw_nenyr = "Variables({
        myColor: '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_variables_method(false)),
            "Ok(NenyrVariables { values: {\"myColor\": \"#FF6677\", \"grayColor\": \"gray\", \"blueColor\": \"blue\", \"redColor\": \"red\", \"primaryColor\": \"yellow\", \"secondaryColor\": \"white\"} })".to_string()
        );
    }

    #[test]
    fn variables_are_not_valid() {
        let raw_nenyr = "Variables({
        myColor '#FF6677',
        grayColor: 'gray',
        blueColor: 'blue',
        redColor: 'red',
        primaryColor: 'red',
        primaryColor: 'yellow',
        secondaryColor: 'white'
    })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_variables_method(false)),
            "Err(NenyrError { suggestion: Some(\"Ensure that each variable is defined with a colon after it. The correct syntax is: `Variables({ myColor: 'variable value', ... })`.\"), context_name: None, context_path: \"\", error_message: \"The `myColor` variable in the `Variables` declaration is missing a colon after the variable name definition. However, found `#FF6677` instead.\", error_kind: SyntaxError, error_tracing: NenyrErrorTracing { line_before: Some(\"Variables({\"), line_after: Some(\"        grayColor: 'gray',\"), error_line: Some(\"        myColor '#FF6677',\"), error_on_line: 2, error_on_col: 26, error_on_pos: 37 } })".to_string()
        );
    }

    #[test]
    fn empty_variables_are_valid() {
        let raw_nenyr = "Variables({ })";
        let mut parser = NenyrParser::new();
        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            format!("{:?}", parser.process_variables_method(false)),
            "Ok(NenyrVariables { values: {} })".to_string()
        );
    }
}
