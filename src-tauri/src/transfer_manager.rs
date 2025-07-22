use crate::crypto::{FileEncryption, generate_secure_code};
use crate::device::Device;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::Manager;
use tokio::fs;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTask {
    pub id: String,
    pub source_path: String,
    pub target_device: Device,
    pub file_size: u64,
    pub transferred: u64,
    pub status: TransferStatus,
    pub created_at: i64,
    pub encrypted: bool,
    pub secure_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Paused,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferHistory {
    pub tasks: Vec<TransferTask>,
}

pub struct AdvancedTransferManager {
    active_transfers: Arc<Mutex<HashMap<String, TransferTask>>>,
    transfer_history: Arc<Mutex<TransferHistory>>,
    bandwidth_limit: Option<u64>, // bytes per second
}

impl AdvancedTransferManager {
    pub fn new() -> Self {
        Self {
            active_transfers: Arc::new(Mutex::new(HashMap::new())),
            transfer_history: Arc::new(Mutex::new(TransferHistory { tasks: Vec::new() })),
            bandwidth_limit: None,
        }
    }
    
    pub async fn add_transfer(&self, source_path: String, target_device: Device, encrypted: bool) -> Result<String, String> {
        let task_id = Uuid::new_v4().to_string();
        let path = Path::new(&source_path);
        
        // 检查文件或文件夹大小
        let file_size = if path.is_dir() {
            self.calculate_folder_size(path).await?
        } else {
            fs::metadata(path).await.map_err(|e| e.to_string())?.len()
        };
        
        let secure_code = if encrypted {
            Some(generate_secure_code())
        } else {
            None
        };
        
        let task = TransferTask {
            id: task_id.clone(),
            source_path,
            target_device,
            file_size,
            transferred: 0,
            status: TransferStatus::Pending,
            created_at: chrono::Utc::now().timestamp(),
            encrypted,
            secure_code,
        };
        
        self.active_transfers.lock().await.insert(task_id.clone(), task.clone());
        
        Ok(task_id)
    }
    
    pub async fn start_transfer(&self, task_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
        let task = {
            let mut transfers = self.active_transfers.lock().await;
            let task = transfers.get_mut(task_id).ok_or("Transfer task not found")?;
            task.status = TransferStatus::InProgress;
            task.clone()
        };
        
        // 在后台启动传输
        let transfers = self.active_transfers.clone();
        let history = self.transfer_history.clone();
        let bandwidth_limit = self.bandwidth_limit;
        
        tokio::spawn(async move {
            let result = Self::execute_transfer(task.clone(), app_handle.clone(), bandwidth_limit).await;
            
            // 更新任务状态
            let mut transfers_lock = transfers.lock().await;
            if let Some(mut active_task) = transfers_lock.remove(&task.id) {
                match result {
                    Ok(_) => {
                        active_task.status = TransferStatus::Completed;
                        active_task.transferred = active_task.file_size;
                    }
                    Err(e) => {
                        active_task.status = TransferStatus::Failed(e);
                    }
                }
                
                // 添加到历史记录
                let mut history_lock = history.lock().await;
                history_lock.tasks.push(active_task.clone());
                
                // 通知前端
                let _ = app_handle.emit_all("transfer-completed", &active_task);
            }
        });
        
        Ok(())
    }
    
    async fn execute_transfer(
        task: TransferTask, 
        app_handle: tauri::AppHandle, 
        bandwidth_limit: Option<u64>
    ) -> Result<(), String> {
        let source_path = Path::new(&task.source_path);
        
        if source_path.is_dir() {
            Self::transfer_folder(task, app_handle, bandwidth_limit).await
        } else {
            Self::transfer_file(task, app_handle, bandwidth_limit).await
        }
    }
    
    async fn transfer_file(
        task: TransferTask, 
        app_handle: tauri::AppHandle, 
        bandwidth_limit: Option<u64>
    ) -> Result<(), String> {
        // 实现文件传输逻辑，支持加密和带宽限制
        // 这里是简化版本，实际实现会更复杂
        log::info!("Starting file transfer: {}", task.source_path);
        
        // 模拟传输进度
        for i in 0..=10 {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            let progress = (i as f64 / 10.0) * 100.0;
            let _ = app_handle.emit_all("transfer-progress", &serde_json::json!({
                "task_id": task.id,
                "progress": progress,
                "transferred": (task.file_size as f64 * progress / 100.0) as u64,
                "status": "transferring"
            }));
        }
        
        Ok(())
    }
    
    async fn transfer_folder(
        task: TransferTask, 
        app_handle: tauri::AppHandle, 
        bandwidth_limit: Option<u64>
    ) -> Result<(), String> {
        // 实现文件夹传输逻辑
        log::info!("Starting folder transfer: {}", task.source_path);
        
        // 递归传输文件夹内容
        // 这里是简化版本
        for i in 0..=10 {
            tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
            
            let progress = (i as f64 / 10.0) * 100.0;
            let _ = app_handle.emit_all("transfer-progress", &serde_json::json!({
                "task_id": task.id,
                "progress": progress,
                "transferred": (task.file_size as f64 * progress / 100.0) as u64,
                "status": "transferring",
                "type": "folder"
            }));
        }
        
        Ok(())
    }
    
    async fn calculate_folder_size(&self, folder_path: &Path) -> Result<u64, String> {
        let mut total_size = 0u64;
        let mut entries = fs::read_dir(folder_path).await.map_err(|e| e.to_string())?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let metadata = entry.metadata().await.map_err(|e| e.to_string())?;
            
            if metadata.is_file() {
                total_size += metadata.len();
            } else if metadata.is_dir() {
                total_size += self.calculate_folder_size(&entry.path()).await?;
            }
        }
        
        Ok(total_size)
    }
    
    pub async fn pause_transfer(&self, task_id: &str) -> Result<(), String> {
        let mut transfers = self.active_transfers.lock().await;
        if let Some(task) = transfers.get_mut(task_id) {
            task.status = TransferStatus::Paused;
            Ok(())
        } else {
            Err("Transfer task not found".to_string())
        }
    }
    
    pub async fn resume_transfer(&self, task_id: &str) -> Result<(), String> {
        let mut transfers = self.active_transfers.lock().await;
        if let Some(task) = transfers.get_mut(task_id) {
            task.status = TransferStatus::InProgress;
            Ok(())
        } else {
            Err("Transfer task not found".to_string())
        }
    }
    
    pub async fn cancel_transfer(&self, task_id: &str) -> Result<(), String> {
        let mut transfers = self.active_transfers.lock().await;
        if let Some(task) = transfers.get_mut(task_id) {
            task.status = TransferStatus::Cancelled;
            Ok(())
        } else {
            Err("Transfer task not found".to_string())
        }
    }
    
    pub async fn get_active_transfers(&self) -> Vec<TransferTask> {
        self.active_transfers.lock().await.values().cloned().collect()
    }
    
    pub async fn get_transfer_history(&self) -> TransferHistory {
        self.transfer_history.lock().await.clone()
    }
    
    pub fn set_bandwidth_limit(&mut self, limit: Option<u64>) {
        self.bandwidth_limit = limit;
    }
} 