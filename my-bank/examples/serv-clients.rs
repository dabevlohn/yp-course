use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc, Arc, RwLock,
};
use std::thread;

#[derive(Debug)]
enum MessageKind {
    FinishAll,
    NewServer,
    Work { work_spec: String },
    ChangeEpoch { epoch: String },
}

fn serve(
    rx: mpsc::Receiver<(usize, MessageKind)>,
    id: usize,
    success_count: Arc<AtomicUsize>,
    epoch: Arc<RwLock<String>>,
) {
    while let Ok((from, msg_kind)) = rx.recv() {
        match msg_kind {
            MessageKind::FinishAll => break,
            MessageKind::NewServer => {
                unreachable!("this message is not for server!")
            }
            MessageKind::ChangeEpoch { epoch: new_epoch } => {
                let mut lock = epoch.write().unwrap();
                println!( "worker-{} is being asked by client-{} at epoch '{}' to change epoch into '{}'",
                          id, from, lock, new_epoch );
                *lock = new_epoch;
            }
            MessageKind::Work { work_spec } => {
                println!( "worker-{} is being asked by client-{} at epoch '{}' to work '{}'",
                          id, from, epoch.read().unwrap(), work_spec );
            }
        }
        success_count.fetch_add(1, Ordering::SeqCst);
    }
    println!("Finishing worker-{}", id);
}

fn balance(
    rx: mpsc::Receiver<(usize, MessageKind)>,
    servers_count: usize,
    success_count: Arc<AtomicUsize>,
) {
    fn make_and_append_server(
        all_servers: &mut Vec<(
            mpsc::Sender<(usize, MessageKind)>,
            thread::JoinHandle<()>,
        )>,
        success_count: Arc<AtomicUsize>,
        epoch: Arc<RwLock<String>>,
    ) {
        let (tx, rx) = mpsc::channel();
        let new_server_id = all_servers.len();
        all_servers.push((
            tx,
            thread::spawn(move || {
                serve(rx, new_server_id, success_count, epoch)
            }),
        ));
    }

    let epoch = Arc::new(RwLock::new("epoch-1".into()));
    let mut servers = Vec::new();
    for _ in 0..servers_count {
        make_and_append_server(
            &mut servers,
            success_count.clone(),
            epoch.clone(),
        );
    }
    let mut next_server = 0usize;
    while let Ok((from, msg_kind)) = rx.recv() {
        match msg_kind {
            MessageKind::FinishAll => {
                for (tx, _) in &servers {
                    tx.send((from, MessageKind::FinishAll)).unwrap();
                }
                for (_, thread) in servers {
                    thread.join().unwrap();
                }
                break;
            }
            MessageKind::NewServer => {
                make_and_append_server(
                    &mut servers,
                    success_count.clone(),
                    epoch.clone(),
                );
            }
            MessageKind::Work { work_spec } => {
                servers[next_server]
                    .0
                    .send((from, MessageKind::Work { work_spec }))
                    .unwrap();
            }
            MessageKind::ChangeEpoch { epoch } => {
                servers[next_server]
                    .0
                    .send((from, MessageKind::ChangeEpoch { epoch }))
                    .unwrap();
            }
        }
        next_server = (next_server + 1).rem_euclid(servers.len());
    }
}

fn my_sleep() {
    thread::sleep(std::time::Duration::from_millis(10))
}

fn client1(server_tx: mpsc::Sender<(usize, MessageKind)>) {
    let id = 1;
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "prepare".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-1".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-2".into(),
            },
        ))
        .unwrap();
    my_sleep();
}
fn client2(server_tx: mpsc::Sender<(usize, MessageKind)>) {
    let id = 2;
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "prepare".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx.send((id, MessageKind::NewServer)).unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-1".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-2".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-3".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-4".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-5".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-6".into(),
            },
        ))
        .unwrap();
    my_sleep();
}
fn client3(server_tx: mpsc::Sender<(usize, MessageKind)>) {
    let id = 3;
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "prepare".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-1".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::ChangeEpoch {
                epoch: "epoch-2".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-2".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-3".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-4".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-5".into(),
            },
        ))
        .unwrap();
    my_sleep();
    server_tx
        .send((
            id,
            MessageKind::Work {
                work_spec: "work-6".into(),
            },
        ))
        .unwrap();
    my_sleep();
}

fn main() {
    println!("Hello, world!");
    let (server_tx, server_rx) = mpsc::channel();
    let success_count = Arc::new(AtomicUsize::new(0));

    let success_count_cloned = success_count.clone();
    let server =
        thread::spawn(move || balance(server_rx, 4, success_count_cloned));
    let (tx1, tx2, tx3) =
        (server_tx.clone(), server_tx.clone(), server_tx.clone());
    let clients = [
        thread::spawn(move || client1(tx1)),
        thread::spawn(move || client2(tx2)),
        thread::spawn(move || client3(tx3)),
    ];
    for client in clients {
        client.join().unwrap();
    }
    server_tx.send((0, MessageKind::FinishAll)).unwrap();
    server.join().unwrap();
    println!("\nDone jobs = {}", success_count.load(Ordering::SeqCst));
}
