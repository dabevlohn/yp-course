use my_macros::Transaction;

use crate::Storage;

#[derive(Debug)]
pub enum TxError {
    InsufficientFunds,
    InvalidAccount,
}

pub trait Transaction {
    fn apply(&self, accounts: &mut Storage) -> Result<(), TxError>;
}

#[derive(Transaction)]
// тут не нужно указывать #[transaction("deposit")], так как это значение по умолчанию
pub struct Deposit {
    pub account: String,
    pub amount: i64,
}

#[derive(Transaction)]
#[transaction("withdraw")]
pub struct Withdraw {
    pub account: String,
    pub amount: i64,
}

#[derive(Transaction)]
#[transaction("transfer")]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub amount: i64,
}
