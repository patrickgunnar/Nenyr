use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(\d+(\.\d+)?|0)?\s*(px|em|rem|vh|vw|vmin|vmax|cm|mm|in|pt|pc|%)$").unwrap();
}

/// A trait responsible for validating breakpoint values.
///
/// The `NenyrBreakpointValidator` trait defines a method for checking the validity of breakpoint values
/// used in responsive design. It utilizes regular expressions to ensure that the given breakpoint strings
/// conform to a specific format. The format allows for various units, including pixels (px),
/// em units, rem units, viewport height (vh), viewport width (vw), and percentages (%). The trait
/// returns `true` if the breakpoint is valid and `false` otherwise.
///
/// # Implementation Details
///
/// The regular expression used for validation is:
/// ```regex
/// ^(\d+(\.\d+)?|0)?\s*(px|em|rem|vh|vw|vmin|vmax|cm|mm|in|pt|pc|%)$
/// ```
/// This regex checks for:
/// - An optional number, which can be an integer or a floating-point value.
/// - An optional unit, which can be one of the following: `px`, `em`, `rem`, `vh`, `vw`, `vmin`,
///   `vmax`, `cm`, `mm`, `in`, `pt`, `pc`, or `%`.
/// - Whitespace between the number and the unit is allowed.
///
/// # Trait Methods
pub trait NenyrBreakpointValidator {
    /// Validates a breakpoint value.
    ///
    /// This method takes a breakpoint string as input and checks its validity
    /// according to the defined regular expression. It returns `true` if the
    /// breakpoint matches the expected format; otherwise, it returns `false`.
    ///
    /// # Parameters
    /// - `breakpoint`: A string slice representing the breakpoint value to validate.
    ///
    /// # Returns
    /// - `bool`: `true` if the breakpoint is valid; `false` if invalid.
    fn is_valid_breakpoint(&self, breakpoint: &str) -> bool {
        RE.is_match(breakpoint)
    }
}

#[cfg(test)]
mod tests {
    use super::NenyrBreakpointValidator;

    struct Breakpoint {}

    impl Breakpoint {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrBreakpointValidator for Breakpoint {}

    #[test]
    fn all_breakpoints_are_valid() {
        let breakpoint = Breakpoint::new();
        let valid_breakpoints = vec![
            "320px", "768px", "1024px", "1280px", "100vw", "75%", "1em", "2rem", "60vh", "40%",
            "1%", "258rem", "978pc", "55pt", "78in", "45mm", "26cm", "956vmax", "1354vmin",
        ];

        for valid_breakpoint in valid_breakpoints {
            assert!(
                breakpoint.is_valid_breakpoint(valid_breakpoint),
                "Breakpoint '{}' should be valid.",
                valid_breakpoint
            );
        }
    }

    #[test]
    fn all_breakpoints_are_not_valid() {
        let breakpoint = Breakpoint::new();
        let invalid_breakpoints = vec![
            "abc", "100", "100pxpx", "12emem", "100px%", "200pp", "8wh", "&3#x", "none", "a44di",
            "wjew", "¨54", "&#21", "8s8", "55", ")(8", "dhw", "@#", "#000000", "2635ss", "&$#swj",
            "",
        ];

        for invalid_breakpoint in invalid_breakpoints {
            assert!(
                !breakpoint.is_valid_breakpoint(invalid_breakpoint),
                "Breakpoint '{}' should be invalid.",
                invalid_breakpoint
            );
        }
    }

    #[test]
    fn performance_test_valid_large_vector() {
        let breakpoint = Breakpoint::new();
        let valid_breakpoints: Vec<String> = (0..1000).map(|i| format!("{}px", i)).collect();

        for valid_breakpoint in valid_breakpoints {
            assert!(
                breakpoint.is_valid_breakpoint(&valid_breakpoint),
                "Breakpoint '{}' should be valid.",
                valid_breakpoint
            );
        }
    }

    #[test]
    fn performance_test_invalid_large_vector() {
        let breakpoint = Breakpoint::new();
        let valid_breakpoints: Vec<String> = (0..1000).map(|i| format!("{}lp", i)).collect();

        for valid_breakpoint in valid_breakpoints {
            assert!(
                !breakpoint.is_valid_breakpoint(&valid_breakpoint),
                "Breakpoint '{}' should be valid.",
                valid_breakpoint
            );
        }
    }
}
