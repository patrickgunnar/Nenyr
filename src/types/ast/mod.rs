use super::{central::CentralContext, layout::LayoutContext, module::ModuleContext};

/// An enumeration representing the Abstract Syntax Tree (AST) for the Nenyr framework.
///
/// The `NenyrAst` enum encapsulates the various contexts that can be defined within the Nenyr
/// framework. Each variant corresponds to a specific context, allowing for the representation
/// of complex styling structures in a modular and organized manner.
///
/// # Variants
/// - `CentralContext`: Represents the central context of the Nenyr framework, containing global
///   configurations and settings.
/// - `LayoutContext`: Represents the layout-specific context, which includes styling information
///   related to a particular layout.
/// - `ModuleContext`: Represents the context for a module within the Nenyr framework, which can
///   include aliases, variables, animations, and style classes specific to that module.
#[derive(Debug, PartialEq, Clone)]
pub enum NenyrAst {
    /// Represents the central context of the Nenyr framework.
    ///
    /// The `CentralContext` variant encapsulates global settings and configurations that are
    /// applicable across the entire application. This may include information relevant to themes,
    /// global variables, and other overarching styling considerations.
    CentralContext(CentralContext),

    /// Represents the layout-specific context.
    ///
    /// The `LayoutContext` variant encapsulates styling information specific to a particular
    /// layout. This may include details such as layout-specific aliases, variables, animations,
    /// and style classes that apply to the layout context.
    LayoutContext(LayoutContext),

    /// Represents the module context within the Nenyr framework.
    ///
    /// The `ModuleContext` variant encapsulates the context for a specific module, which can
    /// include module-level aliases, variables, animations, and style classes. It allows for
    /// modular and reusable design patterns by providing a way to define styles that can be
    /// shared or extended across different modules.
    ModuleContext(ModuleContext),
}
