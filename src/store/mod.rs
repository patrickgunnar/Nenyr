#[derive(Debug, PartialEq, Clone)]
pub enum NenyrState {
    Active,
    Inactive,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrProcessStore {
    is_context_active: NenyrState,
    is_block_active: NenyrState,
    is_nested_block_active: NenyrState,
    is_internal_block_active: NenyrState,
    is_extra_block_active: NenyrState,
    is_complementary_block_active: NenyrState,
    is_nested_content_active: NenyrState,
}

impl NenyrProcessStore {
    pub fn new() -> Self {
        Self {
            is_context_active: NenyrState::Inactive,
            is_block_active: NenyrState::Inactive,
            is_nested_block_active: NenyrState::Inactive,
            is_internal_block_active: NenyrState::Inactive,
            is_extra_block_active: NenyrState::Inactive,
            is_complementary_block_active: NenyrState::Inactive,
            is_nested_content_active: NenyrState::Inactive,
        }
    }

    pub fn set_context_active(&mut self, is_active: bool) {
        if is_active {
            self.is_context_active = NenyrState::Active;
        } else {
            self.is_context_active = NenyrState::Inactive;
        }
    }

    pub fn set_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_block_active = NenyrState::Active;
        } else {
            self.is_block_active = NenyrState::Inactive;
        }
    }

    pub fn set_nested_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_nested_block_active = NenyrState::Active;
        } else {
            self.is_nested_block_active = NenyrState::Inactive;
        }
    }

    pub fn set_internal_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_internal_block_active = NenyrState::Active;
        } else {
            self.is_internal_block_active = NenyrState::Inactive;
        }
    }

    pub fn set_extra_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_extra_block_active = NenyrState::Active;
        } else {
            self.is_extra_block_active = NenyrState::Inactive;
        }
    }

    pub fn set_complementary_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_complementary_block_active = NenyrState::Active;
        } else {
            self.is_complementary_block_active = NenyrState::Inactive;
        }
    }

    pub fn set_nested_content_active(&mut self, is_active: bool) {
        if is_active {
            self.is_nested_content_active = NenyrState::Active;
        } else {
            self.is_nested_content_active = NenyrState::Inactive;
        }
    }

    pub fn is_context_active(&self) -> bool {
        self.is_context_active == NenyrState::Active
    }

    pub fn is_block_active(&self) -> bool {
        self.is_block_active == NenyrState::Active
    }

    pub fn is_nested_block_active(&self) -> bool {
        self.is_nested_block_active == NenyrState::Active
    }

    pub fn is_internal_block_active(&self) -> bool {
        self.is_internal_block_active == NenyrState::Active
    }

    pub fn is_extra_block_active(&self) -> bool {
        self.is_extra_block_active == NenyrState::Active
    }

    pub fn is_complementary_block_active(&self) -> bool {
        self.is_complementary_block_active == NenyrState::Active
    }

    pub fn is_nested_content_active(&self) -> bool {
        self.is_nested_content_active == NenyrState::Active
    }
}
