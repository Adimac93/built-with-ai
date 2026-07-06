//! Out-of-process smoke test: spawns the compiled binary and hits it over
//! TCP. Replaces the deleted Jest `api-e2e` project, which asserted the same
//! contract via Nx task orchestration.

use std::net::TcpStream;
use std::process::{Child, Command};
use std::time::{Duration, Instant};

/// Kills the spawned server even when the test panics.
struct ApiProcess(Child);

impl Drop for ApiProcess {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

#[tokio::test]
async fn compiled_binary_serves_health_endpoint() {
    let port: u16 = 3910;
    let _api = ApiProcess(
        Command::new(env!("CARGO_BIN_EXE_api"))
            .env("PORT", port.to_string())
            .spawn()
            .expect("failed to spawn api binary"),
    );

    let deadline = Instant::now() + Duration::from_secs(10);
    while TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(Instant::now() < deadline, "api did not open port {port} within 10s");
        std::thread::sleep(Duration::from_millis(100));
    }

    let body: serde_json::Value = reqwest::get(format!("http://127.0.0.1:{port}/api"))
        .await
        .expect("request failed")
        .json()
        .await
        .expect("invalid JSON");

    assert_eq!(body, serde_json::json!({ "message": "Hello API" }));
}
