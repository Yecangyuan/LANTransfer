use crate::device::Device;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::{Manager, Emitter};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferProgress {
    pub file_name: String,
    pub progress: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileTransferRequest {
    file_name: String,
    file_size: u64,
    sender_device: Device,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileTransferResponse {
    accepted: bool,
    message: String,
}

pub struct FileTransferManager {
    transfer_port: u16,
}

impl FileTransferManager {
    pub fn new() -> Self {
        Self {
            transfer_port: 8081,
        }
    }

    pub async fn send_files(
        &mut self,
        target_device: Device,
        file_paths: Vec<String>,
        app_handle: tauri::AppHandle,
    ) -> Result<(), String> {
        for file_path in file_paths {
            self.send_single_file(&target_device, &file_path, &app_handle)
                .await?;
        }
        Ok(())
    }

    async fn send_single_file(
        &self,
        target_device: &Device,
        file_path: &str,
        app_handle: &tauri::AppHandle,
    ) -> Result<(), String> {
        let path = Path::new(file_path);
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid file name")?
            .to_string();

        // 获取文件大小
        let metadata = fs::metadata(file_path)
            .await
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let file_size = metadata.len();

        // 连接到目标设备
        let target_addr = format!("{}:{}", target_device.ip, self.transfer_port);
        let mut stream = TcpStream::connect(&target_addr)
            .await
            .map_err(|e| format!("Failed to connect to {}: {}", target_addr, e))?;

        // 发送传输请求
        let request = FileTransferRequest {
            file_name: file_name.clone(),
            file_size,
            sender_device: target_device.clone(), // 这里应该是当前设备信息
        };

        let request_json = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        stream
            .write_all(request_json.as_bytes())
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        stream
            .write_all(b"\n")
            .await
            .map_err(|e| format!("Failed to send newline: {}", e))?;

        // 读取响应
        let mut response_buffer = vec![0; 1024];
        let bytes_read = stream
            .read(&mut response_buffer)
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        let response_str = String::from_utf8_lossy(&response_buffer[..bytes_read]);
        let response: FileTransferResponse = serde_json::from_str(&response_str)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if !response.accepted {
            return Err(format!("Transfer rejected: {}", response.message));
        }

        // 开始传输文件
        let mut file = fs::File::open(file_path)
            .await
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let mut buffer = vec![0; 8192]; // 8KB 缓冲区
        let mut bytes_sent = 0u64;

        // 发送初始进度
        let progress = TransferProgress {
            file_name: file_name.clone(),
            progress: 0.0,
            status: "sending".to_string(),
        };
        let _ = app_handle.emit("transfer-progress", &progress);

        loop {
            let bytes_read = file
                .read(&mut buffer)
                .await
                .map_err(|e| format!("Failed to read file: {}", e))?;

            if bytes_read == 0 {
                break; // 文件读取完成
            }

            stream
                .write_all(&buffer[..bytes_read])
                .await
                .map_err(|e| format!("Failed to send file data: {}", e))?;

            bytes_sent += bytes_read as u64;
            let progress_percent = (bytes_sent as f64 / file_size as f64) * 100.0;

            // 发送进度更新
            let progress = TransferProgress {
                file_name: file_name.clone(),
                progress: progress_percent,
                status: "sending".to_string(),
            };
            let _ = app_handle.emit("transfer-progress", &progress);
        }

        // 传输完成
        let progress = TransferProgress {
            file_name: file_name.clone(),
            progress: 100.0,
            status: "completed".to_string(),
        };
        let _ = app_handle.emit("transfer-progress", &progress);

        Ok(())
    }

    pub async fn start_file_server(&self, app_handle: tauri::AppHandle) -> Result<(), String> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.transfer_port))
            .await
            .map_err(|e| format!("Failed to bind to port {}: {}", self.transfer_port, e))?;

        log::info!(
            "File transfer server listening on port {}",
            self.transfer_port
        );

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let app_handle_clone = app_handle.clone();
                        tokio::spawn(async move {
                            if let Err(e) =
                                Self::handle_incoming_transfer(stream, app_handle_clone).await
                            {
                                log::error!("Failed to handle incoming transfer: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        log::error!("Failed to accept connection: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    async fn handle_incoming_transfer(
        mut stream: TcpStream,
        app_handle: tauri::AppHandle,
    ) -> Result<(), String> {
        // 读取传输请求
        let mut request_buffer = vec![0; 1024];
        let bytes_read = stream
            .read(&mut request_buffer)
            .await
            .map_err(|e| format!("Failed to read request: {}", e))?;

        let request_str = String::from_utf8_lossy(&request_buffer[..bytes_read]);
        let lines: Vec<&str> = request_str.split('\n').collect();

        if lines.is_empty() {
            return Err("Empty request".to_string());
        }

        let request: FileTransferRequest = serde_json::from_str(lines[0])
            .map_err(|e| format!("Failed to parse request: {}", e))?;

        // 自动接受传输（在实际应用中，这里应该询问用户）
        let response = FileTransferResponse {
            accepted: true,
            message: "Transfer accepted".to_string(),
        };

        let response_json = serde_json::to_string(&response)
            .map_err(|e| format!("Failed to serialize response: {}", e))?;

        stream
            .write_all(response_json.as_bytes())
            .await
            .map_err(|e| format!("Failed to send response: {}", e))?;

        // 接收文件
        let downloads_dir = dirs::download_dir().ok_or("Failed to get downloads directory")?;
        let file_path = downloads_dir.join(&request.file_name);

        let mut file = fs::File::create(&file_path)
            .await
            .map_err(|e| format!("Failed to create file: {}", e))?;

        let mut buffer = vec![0; 8192];
        let mut bytes_received = 0u64;

        // 发送初始进度
        let progress = TransferProgress {
            file_name: request.file_name.clone(),
            progress: 0.0,
            status: "receiving".to_string(),
        };
        let _ = app_handle.emit("transfer-progress", &progress);

        loop {
            let bytes_read = stream
                .read(&mut buffer)
                .await
                .map_err(|e| format!("Failed to read from stream: {}", e))?;

            if bytes_read == 0 {
                break; // 连接关闭
            }

            file.write_all(&buffer[..bytes_read])
                .await
                .map_err(|e| format!("Failed to write to file: {}", e))?;

            bytes_received += bytes_read as u64;
            let progress_percent = (bytes_received as f64 / request.file_size as f64) * 100.0;

            // 发送进度更新
            let progress = TransferProgress {
                file_name: request.file_name.clone(),
                progress: progress_percent.min(100.0),
                status: "receiving".to_string(),
            };
            let _ = app_handle.emit("transfer-progress", &progress);

            if bytes_received >= request.file_size {
                break;
            }
        }

        // 传输完成
        let progress = TransferProgress {
            file_name: request.file_name.clone(),
            progress: 100.0,
            status: "completed".to_string(),
        };
        let _ = app_handle.emit("transfer-progress", &progress);

        log::info!("File received: {}", file_path.display());

        Ok(())
    }
}
