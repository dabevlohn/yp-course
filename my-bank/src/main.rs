use bank_system::{
    errors::{BalanceManager, BalanceManagerError},
    storage::inmem::Storage,
    Balance, Name, OpKind, Transaction, Wallet,
};

fn process_if_deposit(
    storage: &mut Storage,
    is_deposit_and_sums: &[(bool, Name, Balance)],
) -> Result<(), BalanceManagerError> {
    for (is_deposit, name, sum) in is_deposit_and_sums {
        if *is_deposit {
            storage.deposit(name, sum)?;
        } else {
            storage.withdraw(name, sum)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), BalanceManagerError> {
    // допустим, latest_history был загружен из файла
    let latest_history: Vec<Transaction> = vec![
        (0, 32, 100).into(),
        (1, 32, -50).into(),
        (4, 32, 150).into(),
    ];
    let mut wallet = Wallet {
        id: 32,
        ballance: 100,
        latest_history,
    };

    // допустим, загрузили последние транзакции из сети
    let updates1: Vec<Transaction> = vec![
        (2, 10, -500).into(),
        (3, 21, 10000).into(),
        (4, 32, 150).into(),
        (5, 32, 50).into(),
        (6, 10, 1000).into(),
        (8, 32, 50).into(),
    ];
    let updates2 = std::collections::VecDeque::<Transaction>::from([
        Transaction::from((4, 32, 150)),
        (7, 32, 100).into(),
        (10, 10, 100).into(),
        (11, 32, -50).into(),
    ]);

    println!("Got transactions:");
    for update in updates1.iter().chain(updates2.iter()) {
        // итерация по транзакциям из updates1 и updates2
        println!("{:?}", update)
    }

    assert_eq!(wallet.update(updates1), 2);
    assert_eq!(wallet.ballance, 200);
    assert_eq!(wallet.update(updates2), 2);
    assert_eq!(wallet.ballance, 250);

    let mut storage = Storage {
        accounts: [
            (
                "Dad".to_string(),
                Balance {
                    result: 0,
                    last_ops: vec![
                        OpKind::Deposit(200000),
                        OpKind::Withdraw(100000),
                    ],
                },
            ),
            (
                "Mom".to_string(),
                Balance {
                    result: 0,
                    last_ops: vec![
                        OpKind::Deposit(120000),
                        OpKind::Withdraw(50000),
                        OpKind::Withdraw(20000),
                    ],
                },
            ),
            (
                "Son".to_string(),
                Balance {
                    result: 0,
                    last_ops: vec![
                        OpKind::Deposit(5000),
                        OpKind::Withdraw(500),
                        OpKind::Withdraw(1000),
                        OpKind::Withdraw(700),
                    ],
                },
            ),
        ]
        .into_iter()
        .collect(),
    };
    let is_deposit_and_sums = [(
        true,
        "Son".to_string(),
        Balance {
            result: 5,
            last_ops: Vec::new(),
        },
    )];
    process_if_deposit(&mut storage, &is_deposit_and_sums)?;
    if let Some((name, _)) = Balance::find_best(&storage) {
        println!(r#"best factor for "{}"!"#, name);
    } else {
        println!("storage is empty");
    }
    Ok(())
}
