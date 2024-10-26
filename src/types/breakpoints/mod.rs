use indexmap::IndexMap;

/// Enum representing breakpoint strategies in the Nenyr styling framework.
///
/// `NenyrBreakpointKind` allows to specify the type of responsive breakpoint
/// approach to apply within the framework. Each variant represents
/// a different method for scaling styles based on device screen sizes.
#[derive(Debug, PartialEq, Clone)]
pub enum NenyrBreakpointKind {
    MobileFirst,
    DesktopFirst,
}

/// Represents the breakpoints used in the Nenyr styling framework.
///
/// The `NenyrBreakpoints` struct is designed to manage responsive styling
/// breakpoints for both mobile-first and desktop-first design approaches.
/// It allows for the addition and configuration of breakpoints based on the
/// selected `NenyrBreakpointKind`, supporting flexible, adaptive styling
/// definitions for different screen sizes.
///
/// # Fields
/// - `mobile_first`: An `Option<IndexMap<String, String>>` containing breakpoint
///   properties specific to mobile-first designs. Defaults to `None` until set.
/// - `desktop_first`: An `Option<IndexMap<String, String>>` containing breakpoint
///   properties specific to desktop-first designs. Defaults to `None` until set.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrBreakpoints {
    pub mobile_first: Option<IndexMap<String, String>>,
    pub desktop_first: Option<IndexMap<String, String>>,
}

impl NenyrBreakpoints {
    /// Creates a new instance of `NenyrBreakpoints`.
    ///
    /// This function initializes a new `NenyrBreakpoints` struct with both
    /// `mobile_first` and `desktop_first` properties set to `None`. It provides
    /// a clean starting point for defining breakpoints.
    ///
    /// # Returns
    /// A new `NenyrBreakpoints` instance with unset breakpoints.
    pub fn new() -> Self {
        Self {
            mobile_first: None,
            desktop_first: None,
        }
    }

    /// Adds a set of breakpoints based on the specified `NenyrBreakpointKind`.
    ///
    /// This function assigns breakpoint properties to either `mobile_first` or
    /// `desktop_first` based on the `breakpoint_kind` parameter. The `properties`
    /// parameter is an `IndexMap` that contains properties associated with
    /// the breakpoint, allowing for customized breakpoint definitions.
    ///
    /// # Parameters
    /// - `breakpoint_kind`: A reference to `NenyrBreakpointKind` which determines
    ///   if the provided properties are applied to `mobile_first` or `desktop_first`.
    /// - `properties`: An `IndexMap<String, String>` containing properties that
    ///   define the breakpoint settings (e.g., `myBreakpointName: "600px"` for mobile-first).
    pub(crate) fn add_breakpoints(
        &mut self,
        breakpoint_kind: &NenyrBreakpointKind,
        properties: IndexMap<String, String>,
    ) {
        match breakpoint_kind {
            NenyrBreakpointKind::MobileFirst => {
                self.mobile_first = Some(properties);
            }
            NenyrBreakpointKind::DesktopFirst => {
                self.desktop_first = Some(properties);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn test_nenyr_breakpoints_new() {
        let breakpoints = NenyrBreakpoints::new();

        // Assert that both mobile_first and desktop_first are None
        assert_eq!(breakpoints.mobile_first, None);
        assert_eq!(breakpoints.desktop_first, None);
    }

    #[test]
    fn test_add_breakpoints_mobile_first() {
        let mut breakpoints = NenyrBreakpoints::new();
        let mut properties = IndexMap::new();

        properties.insert("small".to_string(), "600px".to_string());
        properties.insert("medium".to_string(), "768px".to_string());

        // Add breakpoints to mobile_first
        breakpoints.add_breakpoints(&NenyrBreakpointKind::MobileFirst, properties.clone());

        // Assert that mobile_first is set with correct properties and desktop_first remains None
        assert_eq!(breakpoints.mobile_first, Some(properties));
        assert_eq!(breakpoints.desktop_first, None);
    }

    #[test]
    fn test_add_breakpoints_desktop_first() {
        let mut breakpoints = NenyrBreakpoints::new();
        let mut properties = IndexMap::new();

        properties.insert("large".to_string(), "1024px".to_string());
        properties.insert("xLarge".to_string(), "1280px".to_string());

        // Add breakpoints to desktop_first
        breakpoints.add_breakpoints(&NenyrBreakpointKind::DesktopFirst, properties.clone());

        // Assert that desktop_first is set with correct properties and mobile_first remains None
        assert_eq!(breakpoints.desktop_first, Some(properties));
        assert_eq!(breakpoints.mobile_first, None);
    }

    #[test]
    fn test_add_breakpoints_both_kinds() {
        let mut breakpoints = NenyrBreakpoints::new();

        let mut mobile_properties = IndexMap::new();
        mobile_properties.insert("small".to_string(), "600px".to_string());

        let mut desktop_properties = IndexMap::new();
        desktop_properties.insert("large".to_string(), "1024px".to_string());

        // Add breakpoints to both mobile_first and desktop_first
        breakpoints.add_breakpoints(&NenyrBreakpointKind::MobileFirst, mobile_properties.clone());
        breakpoints.add_breakpoints(
            &NenyrBreakpointKind::DesktopFirst,
            desktop_properties.clone(),
        );

        // Assert that both fields are correctly set
        assert_eq!(breakpoints.mobile_first, Some(mobile_properties));
        assert_eq!(breakpoints.desktop_first, Some(desktop_properties));
    }

    #[test]
    fn test_nenyr_breakpoint_kind_enum() {
        // Ensure that NenyrBreakpointKind enum variants can be matched and compared
        assert_eq!(
            NenyrBreakpointKind::MobileFirst,
            NenyrBreakpointKind::MobileFirst
        );
        assert_eq!(
            NenyrBreakpointKind::DesktopFirst,
            NenyrBreakpointKind::DesktopFirst
        );
    }
}
