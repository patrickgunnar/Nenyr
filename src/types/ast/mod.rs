use super::{central::CentralContext, layout::LayoutContext, module::ModuleContext};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrAst {
    pub central_context: Option<CentralContext>,
    pub layout_context: Option<LayoutContext>,
    pub module_context: Option<ModuleContext>,
}
