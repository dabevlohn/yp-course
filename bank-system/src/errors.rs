use crate::Storage;
use crate::{Balance, Name};

#[derive(Debug)]
pub enum BalanceManagerError {
    UserNotFound(Name),
    NotEnoughMoney { required: u64, available: u64 },
}

pub trait BalanceManager {
    fn deposit(
        &mut self,
        name: &Name,
        amount: &Balance,
    ) -> Result<(), BalanceManagerError>;

    fn withdraw(
        &mut self,
        name: &Name,
        amount: &Balance,
    ) -> Result<(), BalanceManagerError>;
}

impl BalanceManager for Storage {
    fn deposit(
        &mut self,
        name: &Name,
        amount: &Balance,
    ) -> Result<(), BalanceManagerError> {
        if let Some(balance) = self.accounts.get_mut(name) {
            balance.result += amount.result;
            Ok(())
        } else {
            // "Пользователь не найден".into()
            Err(BalanceManagerError::UserNotFound(name.clone()))
        }
    }

    fn withdraw(
        &mut self,
        name: &Name,
        amount: &Balance,
    ) -> Result<(), BalanceManagerError> {
        if let Some(balance) = self.accounts.get_mut(name) {
            if balance.result >= amount.result {
                balance.result -= amount.result;
                Ok(())
            } else {
                // "Недостаточно средств".into()
                Err(BalanceManagerError::NotEnoughMoney {
                    required: amount.result,
                    available: balance.result,
                })
            }
        } else {
            // "Пользователь не найден".into()
            Err(BalanceManagerError::UserNotFound(name.clone()))
        }
    }
}
