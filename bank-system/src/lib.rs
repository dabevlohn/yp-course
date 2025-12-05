//pub mod storage;
// pub struct Storage {
//     accounts: HashMap<Name, Balance>,
// }

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
    pub ballance: u64,
    pub latest_history: Vec<Transaction>,
}

impl Wallet {
    // update принимает транзакции, возвращает количество транзакций, которые относились к этому кошельку
    pub fn update(
        &mut self,
        transactions: impl IntoIterator<Item = Transaction, IntoIter: Clone>,
    ) -> usize {
        let filtered = transactions.into_iter().filter(|t| {
            t.wallet_id == self.id
                && self.latest_history.iter().all(|hist| hist.id != t.id)
        });
        let count_usize = filtered.clone().count();
        self.ballance += filtered.clone().map(|t| t.sum).sum::<i64>() as u64;
        let filtered_transactions_vec = filtered.collect::<Vec<_>>();
        println!(
            "Updated with {} transactions: {:?}",
            count_usize, filtered_transactions_vec
        );
        self.latest_history.extend(filtered_transactions_vec);
        count_usize
    }
}

#[derive(Debug)]
pub enum OpKind {
    // пополнить/потратить счёт
    Deposit(u32),
    Withdraw(u32),
    // закрыть аккаунт - все средства выведены
    CloseAccount,
} // вот и всё, никаких посторонних операций и данных!

// обернём баланс в новый тип, чтобы можно было реализовывать метод
// и запретим балансу опускаться ниже нуля
pub struct Balance(pub u64);

impl Balance {
    // Можно пойти ещё дальше и в качестве аргумента принимать любой тип,
    // который может итерироваться по OpKind, с помощью дженерика:
    // fn process<'a>(&mut self, impl IntoIterator<Item=&'a OpKind>) -> Vec<&'a OpKind>
    // Пробуйте, дерзайте!
    pub fn process<'a>(&mut self, ops: &[&'a OpKind]) -> Vec<&'a OpKind> {
        let mut remaining = ops.into_iter();
        let mut bad_ops = Vec::new();
        for op in &mut remaining {
            match op {
                OpKind::Deposit(value) => {
                    self.0 += *value as u64;
                }
                OpKind::Withdraw(value) if self.0 > *value as u64 => {
                    self.0 -= *value as u64;
                }
                other @ _ => {
                    bad_ops.push(*other);
                    break;
                }
            }
        }
        bad_ops.extend(remaining);
        bad_ops
    }
}
