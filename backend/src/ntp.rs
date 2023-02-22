mod format;

use tokio::net::UdpSocket;

use crate::models::SharedState;

pub struct NtpServer {
    socket: UdpSocket,
    shared_state: SharedState,
}

impl NtpServer {
    pub async fn new(shared_state: SharedState) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:123").await.unwrap();
        NtpServer {
            socket,
            shared_state,
        }
    }
    pub async fn run(self) {
        let mut buf = [0; 1024];
        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await.unwrap();

            self.socket
                .send_to(
                    self.shared_state
                        .lock()
                        .await
                        .time
                        .format_string()
                        .as_str()
                        .as_bytes(),
                    addr,
                )
                .await
                .ok();
        }
    }
}
