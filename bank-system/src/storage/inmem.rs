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

    pub fn get_balance(&self, name: &Name) -> Option<Balance> {
        self.accounts.get(name).cloned()
    }

    pub fn get_all(&self) -> Vec<(Name, Balance)> {
        self.accounts
            .iter()
            .map(|(n, b)| (n.clone(), b.clone()))
            .collect()
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
