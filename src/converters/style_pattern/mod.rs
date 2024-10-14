use crate::tokens::NenyrTokens;

/// A trait for converting Nenyr style pattern tokens into their corresponding CSS pseudo-selectors.
///
/// This trait defines a method that accepts a `NenyrTokens` enum and returns a corresponding CSS pseudo-selector string.
/// It simplifies the process of mapping abstract Nenyr style pattern tokens to their appropriate CSS representation for use in stylesheets.
///
/// ## Purpose
/// This trait is designed to facilitate the automatic conversion of style pattern tokens used in the Nenyr DSL into valid CSS pseudo-selectors.
/// Nenyr style pattern tokens represent abstract styling elements that are mapped to specific pseudo-selectors in CSS, allowing for complex
/// styling logic to be encapsulated in a single DSL (Nenyr).
///
/// By using this trait, you can convert style pattern tokens like `Hover`, `Before`, or `Focus` into strings that represent CSS pseudo-selectors.
/// This makes it easier to handle abstract styling logic and apply it in real-world CSS for web applications.
///
/// ## Usage
///
/// Implement this trait for any struct or object that needs to convert Nenyr style pattern tokens into CSS pseudo-selectors.
/// The method `convert_nenyr_style_pattern_to_pseudo_selector` will match the input style pattern token and return the correct string representation.
///
/// ## Method
///
/// ### `convert_nenyr_style_pattern_to_pseudo_selector`
/// Converts a given `NenyrTokens` style pattern token to a corresponding CSS pseudo-selector string.
///
/// - If the token matches a Nenyr style pattern (e.g., `Hover`, `After`), the method returns the corresponding CSS selector as `Some(String)`.
/// - If the token does not match a valid Nenyr style pattern, it returns `None`.
///
/// #### Parameters
/// - `nenyr_token`: A style pattern token from the `NenyrTokens` enum, which represents various style patterns.
///
/// #### Returns
/// - `Option<String>`:
///   - `Some(String)` if the token matches a valid Nenyr style pattern.
///   - `None` if the token does not match any known style pattern.
pub trait NenyrStylePatternConverter {
    /// Converts a Nenyr style pattern token into its corresponding CSS pseudo-selector string.
    ///
    /// This method performs the matching of a given `NenyrTokens` enum variant to a specific CSS pseudo-selector string.
    ///
    /// # Parameters
    /// - `nenyr_token`: The Nenyr token (`NenyrTokens`) that needs to be converted.
    ///
    /// # Returns
    /// - `Option<String>`: The CSS pseudo-selector as a `String` if the token is recognized, or `None` if the token is unsupported.
    fn convert_nenyr_style_pattern_to_pseudo_selector(
        &self,
        nenyr_token: NenyrTokens,
    ) -> Option<String> {
        match nenyr_token {
            NenyrTokens::Stylesheet => Some("_stylesheet".to_string()),
            NenyrTokens::After => Some("::after".to_string()),
            NenyrTokens::Before => Some("::before".to_string()),
            NenyrTokens::FirstLine => Some("::first-line".to_string()),
            NenyrTokens::FirstLetter => Some("::first-letter".to_string()),
            NenyrTokens::Hover => Some(":hover".to_string()),
            NenyrTokens::Active => Some(":active".to_string()),
            NenyrTokens::Focus => Some(":focus".to_string()),
            NenyrTokens::FirstChild => Some(":first-child".to_string()),
            NenyrTokens::LastChild => Some(":last-child".to_string()),
            NenyrTokens::FirstOfType => Some(":first-of-type".to_string()),
            NenyrTokens::LastOfType => Some(":last-of-type".to_string()),
            NenyrTokens::OnlyChild => Some(":only-child".to_string()),
            NenyrTokens::OnlyOfType => Some(":only-of-type".to_string()),
            NenyrTokens::Target => Some(":target".to_string()),
            NenyrTokens::Visited => Some(":visited".to_string()),
            NenyrTokens::Checked => Some(":checked".to_string()),
            NenyrTokens::Disabled => Some(":disabled".to_string()),
            NenyrTokens::Enabled => Some(":enabled".to_string()),
            NenyrTokens::ReadOnly => Some(":read-only".to_string()),
            NenyrTokens::ReadWrite => Some(":read-write".to_string()),
            NenyrTokens::PlaceholderShown => Some(":placeholder-shown".to_string()),
            NenyrTokens::Valid => Some(":valid".to_string()),
            NenyrTokens::Invalid => Some(":invalid".to_string()),
            NenyrTokens::Required => Some(":required".to_string()),
            NenyrTokens::Optional => Some(":optional".to_string()),
            NenyrTokens::Fullscreen => Some(":fullscreen".to_string()),
            NenyrTokens::FocusWithin => Some(":focus-within".to_string()),
            NenyrTokens::OutOfRange => Some(":out-of-range".to_string()),
            NenyrTokens::Root => Some(":root".to_string()),
            NenyrTokens::Empty => Some(":empty".to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::NenyrTokens;

    use super::NenyrStylePatternConverter;

    struct NenyrToken {}

    impl NenyrToken {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrStylePatternConverter for NenyrToken {}

    #[test]
    fn all_style_patterns_are_valid() {
        let nenyr_token = NenyrToken::new();

        assert_eq!(
            Some("_stylesheet".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Stylesheet)
        );
        assert_eq!(
            Some("::first-line".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FirstLine)
        );
        assert_eq!(
            Some("::first-letter".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FirstLetter)
        );
        assert_eq!(
            Some("::before".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Before)
        );
        assert_eq!(
            Some("::after".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::After)
        );
        assert_eq!(
            Some(":hover".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Hover)
        );
        assert_eq!(
            Some(":active".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Active)
        );
        assert_eq!(
            Some(":focus".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Focus)
        );
        assert_eq!(
            Some(":first-child".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FirstChild)
        );
        assert_eq!(
            Some(":last-child".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::LastChild)
        );
        assert_eq!(
            Some(":first-of-type".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FirstOfType)
        );
        assert_eq!(
            Some(":last-of-type".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::LastOfType)
        );
        assert_eq!(
            Some(":only-child".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::OnlyChild)
        );
        assert_eq!(
            Some(":only-of-type".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::OnlyOfType)
        );
        assert_eq!(
            Some(":target".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Target)
        );
        assert_eq!(
            Some(":visited".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Visited)
        );
        assert_eq!(
            Some(":checked".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Checked)
        );
        assert_eq!(
            Some(":disabled".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Disabled)
        );
        assert_eq!(
            Some(":enabled".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Enabled)
        );
        assert_eq!(
            Some(":read-only".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::ReadOnly)
        );
        assert_eq!(
            Some(":read-write".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::ReadWrite)
        );
        assert_eq!(
            Some(":placeholder-shown".to_string()),
            nenyr_token
                .convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::PlaceholderShown)
        );
        assert_eq!(
            Some(":valid".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Valid)
        );
        assert_eq!(
            Some(":invalid".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Invalid)
        );
        assert_eq!(
            Some(":required".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Required)
        );
        assert_eq!(
            Some(":optional".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Optional)
        );
        assert_eq!(
            Some(":fullscreen".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Fullscreen)
        );
        assert_eq!(
            Some(":focus-within".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FocusWithin)
        );
        assert_eq!(
            Some(":out-of-range".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::OutOfRange)
        );
        assert_eq!(
            Some(":root".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Root)
        );
        assert_eq!(
            Some(":empty".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Empty)
        );
    }

    #[test]
    fn all_style_patterns_are_not_valid() {
        let nenyr_token = NenyrToken::new();

        assert_ne!(
            Some("Stylesheet".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Stylesheet)
        );
        assert_ne!(
            Some("Hover".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Hover)
        );
        assert_ne!(
            Some("Active".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Active)
        );
        assert_ne!(
            Some("Focus".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Focus)
        );
        assert_ne!(
            Some("FirstChild".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FirstChild)
        );
        assert_ne!(
            Some("LastChild".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::LastChild)
        );
        assert_ne!(
            Some("FirstOfType".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FirstOfType)
        );
        assert_ne!(
            Some("LastOfType".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::LastOfType)
        );
        assert_ne!(
            Some("OnlyChild".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::OnlyChild)
        );
        assert_ne!(
            Some("OnlyOfType".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::OnlyOfType)
        );
        assert_ne!(
            Some("Target".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Target)
        );
        assert_ne!(
            Some("Visited".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Visited)
        );
        assert_ne!(
            Some("Checked".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Checked)
        );
        assert_ne!(
            Some("Disabled".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Disabled)
        );
        assert_ne!(
            Some("Enabled".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Enabled)
        );
        assert_ne!(
            Some("ReadOnly".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::ReadOnly)
        );
        assert_ne!(
            Some("ReadWrite".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::ReadWrite)
        );
        assert_ne!(
            Some("PlaceholderShown".to_string()),
            nenyr_token
                .convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::PlaceholderShown)
        );
        assert_ne!(
            Some("Valid".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Valid)
        );
        assert_ne!(
            Some("Invalid".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Invalid)
        );
        assert_ne!(
            Some("Required".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Required)
        );
        assert_ne!(
            Some("Optional".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Optional)
        );
        assert_ne!(
            Some("Fullscreen".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Fullscreen)
        );
        assert_ne!(
            Some("FocusWithin".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FocusWithin)
        );
        assert_ne!(
            Some("FirstLine".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FirstLine)
        );
        assert_ne!(
            Some("FirstLetter".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::FirstLetter)
        );
        assert_ne!(
            Some("Before".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Before)
        );
        assert_ne!(
            Some("After".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::After)
        );
        assert_ne!(
            Some("OutOfRange".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::OutOfRange)
        );
        assert_ne!(
            Some("Root".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Root)
        );
        assert_ne!(
            Some("Empty".to_string()),
            nenyr_token.convert_nenyr_style_pattern_to_pseudo_selector(NenyrTokens::Empty)
        );
    }
}
