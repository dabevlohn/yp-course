use storage::inmem::Storage;

pub mod errors;
pub mod storage;

pub type Name = String;
pub type Balance = i64;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u128,
    pub wallet_id: u128,
    pub sum: i64,
}

impl From<(u128, u128, i64)> for Transaction {
    fn from((id, wallet_id, sum): (u128, u128, i64)) -> Self {
        Self { id, wallet_id, sum }
    }
}

pub struct Wallet {
    pub id: u128,
    pub ballance: i64,
    pub latest_history: Vec<Transaction>,
}

impl Wallet {
    pub fn update(
        &mut self,
        transactions: impl IntoIterator<Item = Transaction, IntoIter: Clone>,
    ) -> usize {
        let filtered = transactions.into_iter().filter(|t| {
            t.wallet_id == self.id
                && self.latest_history.iter().all(|hist| hist.id != t.id)
        });
        let count_usize = filtered.clone().count();
        self.ballance += filtered.clone().map(|t| t.sum).sum::<i64>();
        let filtered_transactions_vec = filtered.collect::<Vec<_>>();
        println!(
            "Updated with {} transactions: {:?}",
            count_usize, filtered_transactions_vec
        );
        self.latest_history.extend(filtered_transactions_vec);
        count_usize
    }
}

#[derive(Debug, Clone)]
pub enum OpKind {
    // пополнить/потратить счёт
    Deposit(u32),
    Withdraw(u32),
    // закрыть аккаунт - все средства выведены
    CloseAccount,
} // вот и всё, никаких посторонних операций и данных!
