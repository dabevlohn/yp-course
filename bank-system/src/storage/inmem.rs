use crate::{Balance, Name};
use std::collections::HashMap;

pub struct Storage {
    pub accounts: HashMap<Name, Balance>,
}

impl Storage {
    /// Создаёт новый пустой банк
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
