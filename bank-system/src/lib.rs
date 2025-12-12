use storage::inmem::Storage;

pub mod errors;
pub mod storage;

pub type Name = String;

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

#[derive(Debug, Clone)]
pub struct Balance {
    pub result: u64,
    pub last_ops: Vec<OpKind>,
}

impl Balance {
    pub fn find_best(storage: &Storage) -> Option<(&str, f32)> {
        if storage.accounts.is_empty() {
            return None;
        }
        let mut best_factor = f32::MIN;
        let mut best_name = "";
        for (name, balance) in &storage.accounts {
            let mut all_positive = 0;
            for op in &balance.last_ops {
                // match op {
                //     OpKind::Deposit(value) => all_positive += *value as u64,
                //     _ => (),
                // }
                if let OpKind::Deposit(value) = op {
                    all_positive += *value as u64
                }
            }
            // почти то же самое на итераторах!
            let all_negative: u64 = balance
                .last_ops
                .iter()
                .filter_map(|op| match op {
                    OpKind::Withdraw(value) => Some(*value as u64),
                    _ => None,
                })
                .sum();
            let factor = all_positive as f32 / all_negative as f32;
            if factor > best_factor {
                best_factor = factor;
                // best_name = &name;
                best_name = name;
            }
        }
        Some((best_name, best_factor))
    }
}
