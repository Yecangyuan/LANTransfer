use crate::device::Device;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::time::Duration;
use tauri::Manager;
use tokio::time;

pub struct NetworkManager {
    service_daemon: Option<ServiceDaemon>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            service_daemon: None,
        }
    }
    
    pub async fn start_discovery(&mut self, app_handle: &tauri::AppHandle, current_device: &Device) -> Result<(), String> {
        // 创建 mDNS 服务
        let mdns = ServiceDaemon::new().map_err(|e| format!("Failed to create mDNS daemon: {}", e))?;
        
        // 注册当前设备服务
        let service_type = "_lantransfer._tcp.local.";
        let instance_name = &current_device.name;
        let host_name = format!("{}.local.", current_device.name.replace(" ", "-"));
        let port = 8080;
        
        let mut properties = HashMap::new();
        properties.insert("device_id".to_string(), current_device.id.clone());
        properties.insert("device_type".to_string(), current_device.device_type.clone());
        properties.insert("ip".to_string(), current_device.ip.clone());
        
        let service_info = ServiceInfo::new(
            service_type,
            instance_name,
            &host_name,
            &current_device.ip,
            port,
            Some(properties),
        ).map_err(|e| format!("Failed to create service info: {}", e))?;
        
        mdns.register(service_info).map_err(|e| format!("Failed to register service: {}", e))?;
        
        // 开始监听服务发现
        let receiver = mdns.browse(service_type).map_err(|e| format!("Failed to start browse: {}", e))?;
        
        self.service_daemon = Some(mdns);
        
        // 在后台任务中处理发现的设备
        let app_handle_clone = app_handle.clone();
        let current_device_id = current_device.id.clone();
        
        tokio::spawn(async move {
            let mut timeout = time::interval(Duration::from_secs(30));
            
            loop {
                tokio::select! {
                    event = receiver.recv_async() => {
                        match event {
                            Ok(ServiceEvent::ServiceResolved(info)) => {
                                if let Some(device) = Self::parse_device_info(info, &current_device_id) {
                                    let _ = app_handle_clone.emit_all("device-discovered", &device);
                                }
                            }
                            Ok(ServiceEvent::ServiceRemoved(_, _)) => {
                                // 处理设备离线
                            }
                            Err(e) => {
                                log::error!("Error receiving mDNS event: {}", e);
                                break;
                            }
                            _ => {}
                        }
                    }
                    _ = timeout.tick() => {
                        // 定期检查设备状态
                    }
                }
            }
        });
        
        Ok(())
    }
    
    fn parse_device_info(service_info: ServiceInfo, current_device_id: &str) -> Option<Device> {
        let properties = service_info.get_properties();
        
        let device_id = properties.get("device_id")?.to_string();
        let device_type = properties.get("device_type")?.to_string();
        let ip = properties.get("ip")?.to_string();
        
        // 忽略自己的设备
        if device_id == current_device_id {
            return None;
        }
        
        Some(Device {
            id: device_id,
            name: service_info.get_fullname().replace(&format!(".{}", service_info.get_type()), ""),
            ip,
            device_type,
            is_online: true,
        })
    }
    
    pub async fn stop_discovery(&mut self) {
        if let Some(daemon) = self.service_daemon.take() {
            let _ = daemon.shutdown();
        }
    }
}

impl Drop for NetworkManager {
    fn drop(&mut self) {
        if let Some(daemon) = self.service_daemon.take() {
            let _ = daemon.shutdown();
        }
    }
} 