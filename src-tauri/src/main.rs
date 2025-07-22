// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod device;
mod file_transfer;
mod network;
mod crypto;

use device::{Device, DeviceManager};
use file_transfer::FileTransferManager;
use network::NetworkManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TransferProgress {
    file_name: String,
    progress: f64,
    status: String,
}

struct AppState {
    device_manager: Arc<Mutex<DeviceManager>>,
    transfer_manager: Arc<Mutex<FileTransferManager>>,
    network_manager: Arc<Mutex<NetworkManager>>,
}

#[tauri::command]
async fn get_device_info(state: State<'_, AppState>) -> Result<Device, String> {
    let device_manager = state.device_manager.lock().await;
    Ok(device_manager.get_current_device().clone())
}

#[tauri::command]
async fn start_device_scan(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let network_manager = state.network_manager.clone();
    let device_manager = state.device_manager.clone();
    
    tokio::spawn(async move {
        let mut nm = network_manager.lock().await;
        let dm = device_manager.lock().await;
        
        if let Err(e) = nm.start_discovery(&app_handle, dm.get_current_device()).await {
            log::error!("Failed to start device discovery: {}", e);
        }
    });
    
    Ok(())
}

#[tauri::command]
async fn send_files(
    target_device_id: String,
    file_paths: Vec<String>,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let transfer_manager = state.transfer_manager.clone();
    let device_manager = state.device_manager.lock().await;
    
    // 查找目标设备
    let target_device = device_manager
        .get_devices()
        .iter()
        .find(|d| d.id == target_device_id)
        .cloned()
        .ok_or("Target device not found")?;
    
    drop(device_manager);
    
    tokio::spawn(async move {
        let mut tm = transfer_manager.lock().await;
        if let Err(e) = tm.send_files(target_device, file_paths, app_handle).await {
            log::error!("Failed to send files: {}", e);
        }
    });
    
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let device_manager = Arc::new(Mutex::new(DeviceManager::new().await));
    let transfer_manager = Arc::new(Mutex::new(FileTransferManager::new()));
    let network_manager = Arc::new(Mutex::new(NetworkManager::new()));
    
    let app_state = AppState {
        device_manager,
        transfer_manager: transfer_manager.clone(),
        network_manager,
    };
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_device_info,
            start_device_scan,
            send_files
        ])
        .setup(move |app| {
            let handle = app.handle();
            let tm = transfer_manager.clone();
            tokio::spawn(async move {
                let ftm = tm.lock().await;
                if let Err(e) = ftm.start_file_server(handle).await {
                    log::error!("Failed to start file server: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 