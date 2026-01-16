use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMetrics {
    pub timestamp: u64,   // Unix timestamp в секундах
    pub temperature: f32, // °C
    pub humidity: f32,    // %
    pub pressure: f32,    // hPa
    pub door_open: bool,
    pub vibration_level: f32,
}

impl RoomMetrics {
    pub fn new(
        temperature: f32,
        humidity: f32,
        pressure: f32,
        door_open: bool,
        vibration_level: f32,
    ) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            temperature,
            humidity,
            pressure,
            door_open,
            vibration_level,
        }
    }

    // Метод для имитации метрик
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self::new(
            rng.gen_range(18.0..25.0),
            rng.gen_range(30.0..60.0),
            rng.gen_range(980.0..1020.0),
            rng.gen_bool(0.1), // 10% chance door is open
            rng.gen_range(10.0..20.0),
        )
    }

    // Метод для форматированного отображения времени
    pub fn formatted_time(&self) -> String {
        format!("{}s", self.timestamp)
    }
}
