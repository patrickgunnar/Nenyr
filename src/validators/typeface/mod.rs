use std::path::Path;

/// `NenyrTypefaceValidator` is a trait responsible for validating the provided path to a typeface file.
///
/// # Overview
///
/// This trait defines a method `is_valid_typeface` that checks if a given `typeface_path` is valid relatively to a given `context_path`.
/// The validation follows these steps:
/// 1. **Empty Path Check**: The method first verifies whether the provided `typeface_path` is empty. If it is, the validation fails and returns `false`.
/// 2. **Context Path Transformation**: The `context_path` is converted into a `Path` instance using `Path::new()`.
/// 3. **Parent Directory Resolution**: It attempts to resolve the parent directory of the provided `context_path`. If the parent directory is found, the method joins this directory with the `typeface_path` to form the relatively typeface file path.
/// 4. **Existence Check**: The method then checks if the resolved `joined_path` exists on the filesystem. If it doesn't exist, the validation fails and returns `false`.
/// 5. **Extension Validation**: If the file exists, the method extracts the file extension from the `typeface_path`. It checks whether the extension matches one of the valid font extensions: `woff`, `woff2`, `ttf`, `otf`, `eot`, or `svg`.
///
/// If all checks are successful, the method returns `true`, indicating that the typeface path is valid. Otherwise, it returns `false`.
///
/// # Method
///
/// - `is_valid_typeface(&self, typeface_path: &str, context_path: &str) -> bool`
///
/// This method returns `true` if the typeface path is valid within the given context, otherwise returns `false`.
///
/// # Valid File Extensions
/// - `.woff`
/// - `.woff2`
/// - `.ttf`
/// - `.otf`
/// - `.eot`
/// - `.svg`
pub trait NenyrTypefaceValidator {
    /// Validates whether the provided `typeface_path` is a valid font file within the given `context_path`.
    ///
    /// # Arguments
    ///
    /// - `typeface_path`: The relative path to the typeface file being validated.
    /// - `context_path`: The base context path from which the typeface path is being resolved.
    ///
    /// # Returns
    ///
    /// A boolean value:
    /// - `true` if the `typeface_path` exists, and its extension matches one of the valid typeface extensions.
    /// - `false` if the path is invalid, non-existent, or has an unsupported file extension.
    fn is_valid_typeface(&self, typeface_path: &str) -> bool {
        if !typeface_path.is_empty() {
            let typeface_path = Path::new(typeface_path);

            if typeface_path.is_absolute() || typeface_path.parent().is_some() {
                if let Some(ext) = typeface_path.extension() {
                    let ext = ext.to_string_lossy().to_string();

                    return vec!["woff", "woff2", "ttf", "otf", "eot", "svg"]
                        .contains(&ext.as_str());
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::NenyrTypefaceValidator;

    struct Typeface {}

    impl Typeface {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrTypefaceValidator for Typeface {}

    #[test]
    fn all_typefaces_are_valid() {
        let typeface = Typeface::new();
        let typeface_paths = vec![
            "../../../mocks/typefaces/rosemartin.regular.otf",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.eot",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.svg",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.ttf",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.woff",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.woff2",
        ];

        for typeface_path in typeface_paths {
            assert!(typeface.is_valid_typeface(typeface_path));
        }
    }

    #[test]
    fn all_typefaces_are_not_valid() {
        let typeface = Typeface::new();
        let typeface_paths = vec![
            "../../../mocks/typefaces/rosemartin.regular.ot2",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.fot",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.sbg",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.tf",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.wof",
            "../../../mocks/typefaces/showa-source-curry.regular-webfont.wof2",
            "../../mocks/typefaces/rosemartin.regular.otf",
            "../mocks/typefaces/showa-source-curry.regular-webfont.eot",
            "mocks/typefaces/showa-source-curry.regular-webfont.svg",
            "../../../typefaces/showa-source-curry.regular-webfont.ttf",
            "../../../mocks/showa-source-curry.regular-webfont.woff",
            "../../../mocks/typefaces",
            "",
        ];

        for typeface_path in typeface_paths {
            assert!(!typeface.is_valid_typeface(typeface_path));
        }
    }

    #[test]
    fn test_empty_typeface_path() {
        let typeface = Typeface::new();

        assert!(!typeface.is_valid_typeface(""));
    }
}
