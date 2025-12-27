use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ============================================================================
// Типы и структуры данных
// ============================================================================

#[derive(Clone, Debug)]
pub enum Command {
    IncrementCounter,
    SetEpoch(String),
    SpawnServer,
    ShutdownAll,
}

/// Общее состояние, разделяемое между серверами
#[derive(Clone)]
pub struct SharedState {
    counter: Arc<Mutex<u64>>,
    epoch: Arc<Mutex<String>>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            counter: Arc::new(Mutex::new(0)),
            epoch: Arc::new(Mutex::new("epoch_0".to_string())),
        }
    }

    pub fn increment_counter(&self) {
        let mut cnt = self.counter.lock().unwrap();
        *cnt += 1;
    }

    pub fn set_epoch(&self, epoch: String) {
        let mut ep = self.epoch.lock().unwrap();
        *ep = epoch;
    }

    pub fn get_state(&self) -> (u64, String) {
        let cnt = *self.counter.lock().unwrap();
        let ep = self.epoch.lock().unwrap().clone();
        (cnt, ep)
    }
}

// ============================================================================
// Сервер (обработчик событий в отдельном потоке)
// ============================================================================

pub struct Server {
    id: usize,
    state: SharedState,
    rx: Receiver<Command>,
}

impl Server {
    pub fn new(id: usize, state: SharedState, rx: Receiver<Command>) -> Self {
        Server { id, state, rx }
    }

    /// Запускает сервер в отдельном потоке
    pub fn run(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            println!("[Server {}] Started", self.id);
            let mut running = true;

            while running {
                // Блокирующий recv с таймаутом для проверки команд
                match self.rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(cmd) => match cmd {
                        Command::IncrementCounter => {
                            self.state.increment_counter();
                            let (cnt, epoch) = self.state.get_state();
                            println!(
                                    "[Server {}] IncrementCounter -> counter={}, epoch={}",
                                    self.id, cnt, epoch
                                );
                        }
                        Command::SetEpoch(new_epoch) => {
                            self.state.set_epoch(new_epoch.clone());
                            let (cnt, epoch) = self.state.get_state();
                            println!(
                                "[Server {}] SetEpoch -> counter={}, epoch={}",
                                self.id, cnt, epoch
                            );
                        }
                        Command::ShutdownAll => {
                            println!("[Server {}] Shutting down...", self.id);
                            running = false;
                        }
                        Command::SpawnServer => {
                            println!("[Server {}] SpawnServer command received but ignored by server", self.id);
                        }
                    },
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // Таймаут, продолжаем опрос
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        println!(
                            "[Server {}] Channel disconnected, shutting down",
                            self.id
                        );
                        running = false;
                    }
                }
            }

            println!("[Server {}] Stopped", self.id);
        })
    }
}

// ============================================================================
// Клиент (актор, отправляющий команды серверам)
// ============================================================================

pub struct Client {
    state: SharedState,
    server_handles: Vec<thread::JoinHandle<()>>,
    server_senders: Vec<Sender<Command>>,
    next_server_id: usize,
}

impl Client {
    pub fn new(state: SharedState) -> Self {
        Client {
            state,
            server_handles: Vec::new(),
            server_senders: Vec::new(),
            next_server_id: 0,
        }
    }

    /// Порождает новый сервер с собственным каналом команд
    pub fn spawn_server(&mut self) {
        let (tx, rx) = mpsc::channel();
        let server = Server::new(self.next_server_id, self.state.clone(), rx);
        let handle = server.run();

        self.server_senders.push(tx);
        self.server_handles.push(handle);
        self.next_server_id += 1;

        println!("[Client] Spawned server #{}", self.next_server_id - 1);
    }

    /// Отправляет команду всем серверам
    pub fn broadcast_command(&self, cmd: Command) {
        for (i, tx) in self.server_senders.iter().enumerate() {
            match tx.send(cmd.clone()) {
                Ok(_) => println!("[Client] Sent {:?} to server {}", cmd, i),
                Err(_) => {
                    println!("[Client] Failed to send command to server {}", i)
                }
            }
        }
    }

    /// Увеличивает счётчик на уровне клиента
    pub fn increment_counter(&self) {
        self.state.increment_counter();
        let (cnt, epoch) = self.state.get_state();
        println!(
            "[Client] IncrementCounter -> counter={}, epoch={}",
            cnt, epoch
        );
    }

    /// Меняет эпоху на уровне клиента
    pub fn set_epoch(&self, epoch: String) {
        self.state.set_epoch(epoch.clone());
        let (cnt, new_epoch) = self.state.get_state();
        println!(
            "[Client] SetEpoch({}) -> counter={}, epoch={}",
            epoch, cnt, new_epoch
        );
    }

    /// Отправляет команду завершения всем серверам и ждёт их окончания
    pub fn shutdown_all(&mut self) {
        println!("[Client] Sending shutdown command to all servers...");
        self.broadcast_command(Command::ShutdownAll);

        for a in self.server_handles.drain(..) {
            a.join().unwrap();
            println!("[Client] Server joined");
        }
        self.server_senders.clear();

        println!("[Client] All servers shut down");
    }
}

// ============================================================================
// Главная функция: демонстрация работы
// ============================================================================

fn main() {
    println!("=== Actor-based Server System ===\n");

    let state = SharedState::new();
    let mut client = Client::new(state.clone());
    let mut client1 = Client::new(state.clone());

    // Спауним 3 сервера
    client.spawn_server();
    client.spawn_server();
    client.spawn_server();
    client1.spawn_server();
    client1.spawn_server();

    thread::sleep(Duration::from_millis(100));

    // Клиент увеличивает счётчик (это изменит состояние для всех серверов)
    println!("\n--- Incrementing counter via client ---");
    client.increment_counter();
    thread::sleep(Duration::from_millis(100));

    // Брокаст команды увеличения счётчика серверам
    println!("\n--- Broadcasting IncrementCounter to servers ---");
    client.broadcast_command(Command::IncrementCounter);
    thread::sleep(Duration::from_millis(200));

    // Меняем эпоху на уровне клиента
    println!("\n--- Setting epoch via client ---");
    client.set_epoch("epoch_alpha".to_string());
    thread::sleep(Duration::from_millis(100));

    // Брокаст команды изменения эпохи
    println!("\n--- Broadcasting SetEpoch to servers ---");
    client.broadcast_command(Command::SetEpoch("epoch_beta".to_string()));
    thread::sleep(Duration::from_millis(200));

    // Порождаем новый сервер
    println!("\n--- Spawning new server ---");
    client.spawn_server();
    thread::sleep(Duration::from_millis(100));

    // Отправляем команды новому серверу
    println!("\n--- Broadcasting to all servers (including new one) ---");
    client.broadcast_command(Command::IncrementCounter);
    thread::sleep(Duration::from_millis(200));

    // Отправляем команды со второго клиента
    client1.broadcast_command(Command::SetEpoch("epoch_delta".to_string()));
    client1.broadcast_command(Command::IncrementCounter);

    // Завершаем все серверы
    println!("\n--- Shutting down all servers ---");
    client.shutdown_all();
    client1.shutdown_all();

    println!("\n=== System shutdown complete ===");
}
