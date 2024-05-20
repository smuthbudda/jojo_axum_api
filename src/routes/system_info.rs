use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::Response,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use sysinfo::{Disks, System};

use super::routes::AppState;

pub async fn get_system_details_handler(
    State(_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let mut sys: System = System::new_all();
    sys.refresh_all();
    sys.refresh_cpu();
    let mut disk_list: Vec<String> = vec![];
    let total_memory: u64 = sys.total_memory();
    let os: String = System::os_version().unwrap_or_default();
    let cpu = sys.cpus().first().unwrap();
    let cpu_info = CPUInfo {
        name: cpu.name().to_string(),
        brand: cpu.brand().to_string(),
        id: cpu.vendor_id().to_string(),
    };

    for disk in &Disks::new_with_refreshed_list() {
        let name: String = disk
            .name()
            .to_str()
            .to_owned()
            .unwrap_or_default()
            .to_string();
        disk_list.push(name);
    }
    let sys_info = SystemInfo::new(cpu_info, os, total_memory, disk_list);

    let response = Response::new(json!({"status": "success", "data": sys_info}).to_string());
    return Ok(response);
}

pub async fn realtime_cpu_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async { web_socket(socket, state).await })
}

async fn web_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        socket
            .send(Message::Text(serde_json::to_string(&msg).unwrap()))
            .await
            .unwrap();
    }
}

// pub async fn realtime_cpu_handler(
//     ws: WebSocketUpgrade,
//     State(state): State<Arc<AppState>>,
// ) -> impl IntoResponse {
//     ws.on_upgrade(|socket| async { web_socket(socket, state).await })
// }
// async fn web_socket(mut socket: WebSocket, state: Arc<AppState>) {
//     let mut rx = state.tx.subscribe();

//     let cpu_info_task = task::spawn(gather_cpu_info(state.tx.clone()));

//     while let Ok(msg) = rx.recv().await {
//         socket
//             .send(Message::Text(serde_json::to_string(&msg).unwrap()))
//             .await
//             .unwrap();
//     }

//     cpu_info_task.abort();
// }

// async fn gather_cpu_info(tx: broadcast::Sender<super::routes::Snapshot>) {
//     let mut sys = System::new();
//     loop {
//         sys.refresh_cpu();
//         let v: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
//         let _ = tx.send(v);
//         tokio::time::sleep(std::time::Duration::from_secs(3)).await;
//         // let three_seconds = std::time::Duration::new(3, 0);
//         // thread::sleep(three_seconds);
//     }
// }

#[derive(Serialize)]
pub struct SystemInfo {
    pub cpu_info: CPUInfo,
    pub os: String,
    pub ram_total: u64,
    pub disks: Vec<String>,
    //pub temps: Vec<SysComponent>, do that later
}

impl SystemInfo {
    pub fn new(
        cpu_info: CPUInfo,
        os: String,
        ram_total: u64,
        disks: Vec<String>,
        // temps: Vec<SysComponent>,
    ) -> SystemInfo {
        SystemInfo {
            os,
            cpu_info,
            ram_total,
            disks,
            // temps,
        }
    }
}

#[derive(Serialize)]
pub struct SysComponent {
    pub name: String,
    pub temp: f32,
}

#[derive(Serialize)]
pub struct CPUInfo {
    pub name: String,
    pub brand: String,
    pub id: String,
}
