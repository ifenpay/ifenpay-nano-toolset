use std::process::{Command, Stdio, Child};
use std::sync::Arc;
use tokio::sync::Mutex;

use dotenvy::dotenv;
use tokio::spawn;

use crate::services::{app::initialize::initialize_rust_app, mcp::initialize_mcp_stdio_server, routers::router::initialize_api_routers};

mod handlers;
mod services;
mod structs;
mod helpers;
mod tests;
mod enums;

pub mod managers;

#[tokio::main]
async fn main() {
	dotenv().ok();

    initialize_rust_app().await;

    spawn(initialize_api_routers());

    initialize_mcp_stdio_server(tokio::runtime::Handle::current());

    let child = Arc::new(Mutex::new(initilialize_work_server()));
    let child_clone = Arc::clone(&child);
    
    spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            let mut child = child_clone.lock().await;
            #[cfg(windows)]
            let _ = child.kill();
            #[cfg(unix)]
            let _ = child.kill();
        }
    });

    tokio::signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
}

fn initilialize_work_server() -> Child {
    Command::new("cargo")
        .arg("run")
        .arg("--release")
        .current_dir("nano-work-server")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start nano-work-server")
}