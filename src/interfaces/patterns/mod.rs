use crate::{
    converters::{property::NenyrPropertyConverter, style_pattern::NenyrStylePatternConverter},
    error::{NenyrError, NenyrErrorKind},
    loop_while_not,
    tokens::NenyrTokens,
    types::class::NenyrStyleClass,
    validators::style_syntax::NenyrStyleSyntaxValidator,
    NenyrParser, NenyrResult,
};

impl<'a> NenyrParser<'a> {
    pub(crate) fn process_patterns_methods(
        &mut self,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
        is_panoramic: bool,
        panoramic_name: &Option<String>,
    ) -> NenyrResult<()> {
        if is_panoramic {
            self.processing_state.set_internal_block_active(true);
        } else {
            self.processing_state.set_block_active(true);
        }

        if let NenyrTokens::Important = self.current_token {
            let is_important = self.retrieve_important_value(class_name)?;

            style_class.set_importance(is_important);

            return Ok(());
        } else if let NenyrTokens::PanoramicViewer = self.current_token {
            if is_panoramic {
                // TODO: Throw an error, cannot call PanoramicViewer inside PanoramicViewer.
            }

            // TODO: Create the Panoramic Viewer parsers.

            return Ok(());
        } else {
            if let Some(pattern_name) =
                self.convert_nenyr_style_pattern_to_pseudo_selector(&self.current_token)
            {
                self.handle_parenthesized_curly_bracketed_section(
                    &pattern_name,
                    class_name,
                    is_panoramic,
                    style_class,
                    panoramic_name,
                )?;

                return Ok(());
            }
        }

        Err(NenyrError::new(
            Some("".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(""),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    fn retrieve_important_value(&mut self, class_name: &str) -> NenyrResult<bool> {
        self.process_next_token()?;

        self.parse_parenthesized_delimiter(
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            |parser| parser.parse_boolean_literal(Some("".to_string()), "", true),
        )
    }

    fn handle_parenthesized_curly_bracketed_section(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        style_class: &mut NenyrStyleClass,
        panoramic_name: &Option<String>,
    ) -> NenyrResult<()> {
        self.process_next_token()?;

        // First, parse the expression within the parentheses.
        self.parse_parenthesized_delimiter(
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            |parser| {
                // Once inside the parentheses, parse the expression within the curly brackets.
                let parsed_value = parser.parse_curly_bracketed_delimiter(
                    Some("".to_string()),
                    "",
                    Some("".to_string()),
                    "",
                    |parser| {
                        parser.handle_method_block(
                            pattern_name,
                            class_name,
                            is_panoramic,
                            style_class,
                            panoramic_name,
                        )
                    },
                )?;

                // Processes the next token
                parser.process_next_token()?;

                Ok(parsed_value)
            },
        )
    }

    fn handle_method_block(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        style_class: &mut NenyrStyleClass,
        panoramic_name: &Option<String>,
    ) -> NenyrResult<()> {
        if is_panoramic {
            match panoramic_name {
                Some(name) => self.process_method_block_on_panoramic(
                    pattern_name,
                    class_name,
                    style_class,
                    name,
                ),
                None => unreachable!(),
            }
        } else {
            self.process_method_block(pattern_name, class_name, style_class)
        }
    }

    fn process_method_block(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        // Set/Reset the pattern node before inserting into it.
        style_class.reset_pattern_node(pattern_name);

        loop_while_not!(
            self,
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            || self.processing_state.is_nested_block_active(),
            |is_active| self.processing_state.set_nested_block_active(is_active),
            {
                self.retrieve_nenyr_property(pattern_name, class_name, false, style_class)?;
            }
        );

        self.processing_state.set_nested_block_active(false);

        Ok(())
    }

    fn process_method_block_on_panoramic(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        style_class: &mut NenyrStyleClass,
        panoramic_name: &str,
    ) -> NenyrResult<()> {
        // Set/Reset the pattern node on panoramic node before inserting into it.
        style_class.reset_pattern_node_on_panoramic_node(panoramic_name, pattern_name);

        loop_while_not!(
            self,
            Some("".to_string()),
            "",
            Some("".to_string()),
            "",
            || self.processing_state.is_extra_block_active(),
            |is_active| self.processing_state.set_extra_block_active(is_active),
            {
                self.retrieve_nenyr_property(pattern_name, class_name, true, style_class)?;
            }
        );

        self.processing_state.set_extra_block_active(false);

        Ok(())
    }

    fn retrieve_nenyr_property(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        is_panoramic: bool,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        if is_panoramic {
            self.processing_state.set_extra_block_active(true);
        } else {
            self.processing_state.set_nested_block_active(true);
        }

        if let Some(property) = self.convert_nenyr_property_to_css_property(&self.current_token) {
            return self.retrieve_nenyr_value(pattern_name, class_name, property, style_class);
        } else if let NenyrTokens::Identifier(nickname) = self.current_token.clone() {
            return self.retrieve_nenyr_value(pattern_name, class_name, nickname, style_class);
        }

        Err(NenyrError::new(
            Some("".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(""),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    fn retrieve_nenyr_value(
        &mut self,
        pattern_name: &str,
        class_name: &str,
        property: String,
        style_class: &mut NenyrStyleClass,
    ) -> NenyrResult<()> {
        self.process_next_token()?;
        self.parse_colon_delimiter(Some("".to_string()), "", true)?;

        let value = self.parse_string_literal(Some("".to_string()), "", false)?;

        if self.is_valid_style_syntax(&value) {
            style_class.add_style_rule(pattern_name.to_string(), property, value);

            return Ok(());
        }

        Err(NenyrError::new(
            Some("".to_string()),
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(""),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}
