use crate::{Balance, Name};
use std::collections::HashMap;

pub struct Storage {
    pub accounts: HashMap<Name, Balance>,
}

// pub mod inmem;
pub mod csvfile;
