mod api;
mod models;
mod ntp;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    println!("Hello! ntp-time-machine starting...");

    let shared_state = Arc::new(Mutex::new(models::AppState {
        time: models::Time {
            year: 2023,
            month: 2,
            day: 22,

            hour: 15,
            minute: 4,
            second: 0,
        },
    }));
    let shared_state_ntp = shared_state.clone();
    tokio::spawn(async move {
        let ntp_server = ntp::NtpServer::new(shared_state_ntp).await;
        ntp_server.run().await;
    });
    tokio::spawn(async move {
        api::start_server(shared_state).await;
    });
    loop {}
}
