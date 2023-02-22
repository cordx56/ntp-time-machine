use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub year: usize,
    pub month: usize,
    pub day: usize,

    pub hour: usize,
    pub minute: usize,
    pub second: usize,
}
impl Time {
    pub fn format_string(&self) -> String {
        format!(
            "{}/{}/{} {}:{}:{}",
            self.year,
            self.month + 1,
            self.day,
            self.hour,
            self.minute,
            self.second
        )
    }
}

#[derive(Debug)]
pub struct AppState {
    pub time: Time,
}
pub type SharedState = Arc<Mutex<AppState>>;
