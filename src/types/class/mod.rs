use indexmap::IndexMap;

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrStyleClass {
    class_name: String,
    deriving_from: Option<String>,
    is_important: Option<bool>,

    style_patterns: Option<IndexMap<String, IndexMap<String, String>>>,
    responsive_patterns: Option<IndexMap<String, IndexMap<String, IndexMap<String, String>>>>,
}

impl NenyrStyleClass {
    pub fn new(class_name: String, deriving_from: Option<String>) -> Self {
        Self {
            class_name,
            deriving_from,
            is_important: None,
            style_patterns: None,
            responsive_patterns: None,
        }
    }

    pub(crate) fn set_importance(&mut self, is_important: bool) {
        self.is_important = Some(is_important);
    }

    pub(crate) fn reset_pattern_node(&mut self, pattern_name: &str) {
        if self.style_patterns == None {
            self.style_patterns = Some(IndexMap::new());
        }

        if let Some(style_pattern) = &mut self.style_patterns {
            style_pattern.insert(pattern_name.to_string(), IndexMap::new());
        }
    }

    pub(crate) fn add_style_rule(&mut self, pattern_name: String, property: String, value: String) {
        if self.style_patterns == None {
            self.style_patterns = Some(IndexMap::new());
        }

        if let Some(style_pattern) = &mut self.style_patterns {
            if let Some(existing_pattern) = style_pattern.get_mut(&pattern_name) {
                existing_pattern.insert(property, value);
            } else {
                let property_value = IndexMap::from([(property, value)]);

                style_pattern.insert(pattern_name, property_value);
            }
        }
    }

    pub(crate) fn reset_panoramic_node(&mut self, panoramic_name: &str) {
        if self.responsive_patterns == None {
            self.responsive_patterns = Some(IndexMap::new());
        }

        if let Some(responsive_patterns) = &mut self.responsive_patterns {
            responsive_patterns.insert(panoramic_name.to_string(), IndexMap::new());
        }
    }

    pub(crate) fn reset_pattern_node_on_panoramic_node(
        &mut self,
        panoramic_name: &str,
        pattern_name: &str,
    ) {
        if self.responsive_patterns == None {
            self.responsive_patterns = Some(IndexMap::new());
        }

        if let Some(responsive_patterns) = &mut self.responsive_patterns {
            if let Some(panoramic_patterns) = responsive_patterns.get_mut(panoramic_name) {
                panoramic_patterns.insert(pattern_name.to_string(), IndexMap::new());
            }
        }
    }

    pub(crate) fn add_responsive_style_rule(
        &mut self,
        panoramic_name: String,
        pattern_name: String,
        property: String,
        value: String,
    ) {
        if self.responsive_patterns == None {
            self.responsive_patterns = Some(IndexMap::new());
        }

        if let Some(responsive_patterns) = &mut self.responsive_patterns {
            if let Some(panoramic_patterns) = responsive_patterns.get_mut(&panoramic_name) {
                if let Some(existing_pattern) = panoramic_patterns.get_mut(&pattern_name) {
                    existing_pattern.insert(property, value);
                } else {
                    let property_value = IndexMap::from([(property, value)]);

                    panoramic_patterns.insert(pattern_name, property_value);
                }
            } else {
                let property_value = IndexMap::from([(property, value)]);
                let style_pattern = IndexMap::from([(pattern_name, property_value)]);

                responsive_patterns.insert(panoramic_name, style_pattern);
            }
        }
    }
}
