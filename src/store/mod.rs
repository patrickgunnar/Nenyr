#[derive(Debug, PartialEq, Clone)]
pub enum NenyrState {
    Active,
    Inactive,
}

/// Represents the process states within the Nenyr parser.
///
/// The `NenyrProcessStore` struct is designed to manage and track various stages of
/// activity within the Nenyr parsing process. It uses distinct states to determine
/// when specific contexts or blocks are active or inactive. These states enable the
/// parser to understand the current operational mode (e.g., inside a nested block,
/// handling complementary content, etc.) and handle parsing logic accordingly.
///
/// This struct contains several states that affect how parsing and other operations
/// are handled:
///
/// - **Context state**: Whether the main context is active.
/// - **Block state**: Whether the current parsing block is active.
/// - **Nested block state**: Whether a nested block is active.
/// - **Internal block state**: Whether an internal block within a structure is active.
/// - **Extra block state**: Used for additional block tracking beyond basic blocks.
/// - **Complementary block state**: Tracks the status of secondary or complementary parsing areas.
///
/// The parser may use these states to determine what action should be taken or how
/// certain conditions are interpreted based on the current state.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrProcessStore {
    /// Indicates if the main context is active.
    is_context_active: NenyrState,
    /// Tracks if the current block (outermost) is active.
    is_block_active: NenyrState,
    /// Tracks if a nested block within the current block is active.
    is_nested_block_active: NenyrState,
    /// Tracks if an internal block, such as internal braces or groups, is active.
    is_internal_block_active: NenyrState,
    /// Tracks if an extra block (beyond the regular block structure) is active.
    is_extra_block_active: NenyrState,
    /// Tracks if a complementary block, which may represent an optional or secondary section, is active.
    is_complementary_block_active: NenyrState,
}

impl NenyrProcessStore {
    /// Creates a new instance of `NenyrProcessStore` with all states set to `Inactive`.
    pub fn new() -> Self {
        Self {
            is_context_active: NenyrState::Inactive,
            is_block_active: NenyrState::Inactive,
            is_nested_block_active: NenyrState::Inactive,
            is_internal_block_active: NenyrState::Inactive,
            is_extra_block_active: NenyrState::Inactive,
            is_complementary_block_active: NenyrState::Inactive,
        }
    }

    /// Sets the context state to `Active` or `Inactive`.
    ///
    /// # Arguments
    ///
    /// * `is_active` - A boolean indicating whether the context should be active.
    pub fn set_context_active(&mut self, is_active: bool) {
        if is_active {
            self.is_context_active = NenyrState::Active;
        } else {
            self.is_context_active = NenyrState::Inactive;
        }
    }

    /// Sets the block state to `Active` or `Inactive`.
    ///
    /// # Arguments
    ///
    /// * `is_active` - A boolean indicating whether the block should be active.
    pub fn set_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_block_active = NenyrState::Active;
        } else {
            self.is_block_active = NenyrState::Inactive;
        }
    }

    /// Sets the nested block state to `Active` or `Inactive`.
    ///
    /// # Arguments
    ///
    /// * `is_active` - A boolean indicating whether the nested block should be active.
    pub fn set_nested_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_nested_block_active = NenyrState::Active;
        } else {
            self.is_nested_block_active = NenyrState::Inactive;
        }
    }

    /// Sets the internal block state to `Active` or `Inactive`.
    ///
    /// # Arguments
    ///
    /// * `is_active` - A boolean indicating whether the internal block should be active.
    pub fn set_internal_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_internal_block_active = NenyrState::Active;
        } else {
            self.is_internal_block_active = NenyrState::Inactive;
        }
    }

    /// Sets the extra block state to `Active` or `Inactive`.
    ///
    /// # Arguments
    ///
    /// * `is_active` - A boolean indicating whether the extra block should be active.
    pub fn set_extra_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_extra_block_active = NenyrState::Active;
        } else {
            self.is_extra_block_active = NenyrState::Inactive;
        }
    }

    /// Sets the complementary block state to `Active` or `Inactive`.
    ///
    /// # Arguments
    ///
    /// * `is_active` - A boolean indicating whether the complementary block should be active.
    pub fn set_complementary_block_active(&mut self, is_active: bool) {
        if is_active {
            self.is_complementary_block_active = NenyrState::Active;
        } else {
            self.is_complementary_block_active = NenyrState::Inactive;
        }
    }

    /// Checks if the context is currently active.
    ///
    /// # Returns
    ///
    /// * `true` if the context is active, `false` otherwise.
    pub fn is_context_active(&self) -> bool {
        self.is_context_active == NenyrState::Active
    }

    /// Checks if the current block is active.
    pub fn is_block_active(&self) -> bool {
        self.is_block_active == NenyrState::Active
    }

    /// Checks if a nested block is active.
    pub fn is_nested_block_active(&self) -> bool {
        self.is_nested_block_active == NenyrState::Active
    }

    /// Checks if an internal block is active.
    pub fn is_internal_block_active(&self) -> bool {
        self.is_internal_block_active == NenyrState::Active
    }

    /// Checks if an extra block is active.
    pub fn is_extra_block_active(&self) -> bool {
        self.is_extra_block_active == NenyrState::Active
    }

    /// Checks if the complementary block is active.
    pub fn is_complementary_block_active(&self) -> bool {
        self.is_complementary_block_active == NenyrState::Active
    }
}

#[cfg(test)]
mod tests {
    use super::NenyrProcessStore;

    #[test]
    fn all_states_must_be_active() {
        let mut store = NenyrProcessStore::new();

        store.set_block_active(true);
        store.set_complementary_block_active(true);
        store.set_context_active(true);
        store.set_extra_block_active(true);
        store.set_internal_block_active(true);
        store.set_nested_block_active(true);

        assert!(store.is_block_active());
        assert!(store.is_complementary_block_active());
        assert!(store.is_context_active());
        assert!(store.is_extra_block_active());
        assert!(store.is_internal_block_active());
        assert!(store.is_nested_block_active());
    }

    #[test]
    fn all_states_must_be_inactive() {
        let mut store = NenyrProcessStore::new();

        store.set_block_active(false);
        store.set_complementary_block_active(false);
        store.set_context_active(false);
        store.set_extra_block_active(false);
        store.set_internal_block_active(false);
        store.set_nested_block_active(false);

        assert!(!store.is_block_active());
        assert!(!store.is_complementary_block_active());
        assert!(!store.is_context_active());
        assert!(!store.is_extra_block_active());
        assert!(!store.is_internal_block_active());
        assert!(!store.is_nested_block_active());
    }

    #[test]
    fn all_states_must_be_valid() {
        let mut store = NenyrProcessStore::new();

        store.set_block_active(false);
        store.set_complementary_block_active(true);
        store.set_context_active(false);
        store.set_extra_block_active(true);
        store.set_internal_block_active(false);
        store.set_nested_block_active(true);

        assert_ne!(store.is_block_active(), true);
        assert_eq!(store.is_complementary_block_active(), true);
        assert_ne!(store.is_context_active(), true);
        assert_eq!(store.is_extra_block_active(), true);
        assert_ne!(store.is_internal_block_active(), true);
        assert_eq!(store.is_nested_block_active(), true);
    }

    #[test]
    fn all_states_must_not_be_valid() {
        let mut store = NenyrProcessStore::new();

        store.set_block_active(true);
        store.set_complementary_block_active(false);
        store.set_context_active(true);
        store.set_extra_block_active(false);
        store.set_internal_block_active(true);
        store.set_nested_block_active(false);

        assert_ne!(store.is_block_active(), false);
        assert_ne!(store.is_complementary_block_active(), true);
        assert_ne!(store.is_context_active(), false);
        assert_ne!(store.is_extra_block_active(), true);
        assert_ne!(store.is_internal_block_active(), false);
        assert_ne!(store.is_nested_block_active(), true);
    }
}
