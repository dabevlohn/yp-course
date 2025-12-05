use bank_system::{Balance, OpKind, Transaction, Wallet};

fn main() {
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

    let ops = [
        &OpKind::Deposit(32),
        &OpKind::Withdraw(64),
        &OpKind::CloseAccount,
    ];
    let bad_ops = Balance(0).process(&ops);
    assert_eq!(bad_ops.len(), 2);
    println!("{:?}", bad_ops);
}
