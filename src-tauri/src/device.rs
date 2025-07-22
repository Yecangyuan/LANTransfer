use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub device_type: String,
    pub is_online: bool,
}

pub struct DeviceManager {
    current_device: Device,
    discovered_devices: HashMap<String, Device>,
}

impl DeviceManager {
    pub async fn new() -> Self {
        let current_device = Self::create_current_device().await;
        
        Self {
            current_device,
            discovered_devices: HashMap::new(),
        }
    }
    
    async fn create_current_device() -> Device {
        let device_id = Uuid::new_v4().to_string();
        let device_name = hostname::get()
            .unwrap_or_else(|_| "Unknown Device".into())
            .to_string_lossy()
            .to_string();
        
        let local_ip = local_ip_address::local_ip()
            .unwrap_or_else(|_| "127.0.0.1".parse().unwrap())
            .to_string();
        
        let device_type = Self::detect_device_type();
        
        Device {
            id: device_id,
            name: device_name,
            ip: local_ip,
            device_type,
            is_online: true,
        }
    }
    
    fn detect_device_type() -> String {
        #[cfg(target_os = "windows")]
        return "desktop".to_string();
        
        #[cfg(target_os = "macos")]
        return "desktop".to_string();
        
        #[cfg(target_os = "linux")]
        return "desktop".to_string();
        
        #[cfg(target_os = "android")]
        return "mobile".to_string();
        
        #[cfg(target_os = "ios")]
        return "mobile".to_string();
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux", target_os = "android", target_os = "ios")))]
        return "unknown".to_string();
    }
    
    pub fn get_current_device(&self) -> &Device {
        &self.current_device
    }
    
    pub fn add_device(&mut self, device: Device) {
        self.discovered_devices.insert(device.id.clone(), device);
    }
    
    pub fn remove_device(&mut self, device_id: &str) {
        self.discovered_devices.remove(device_id);
    }
    
    pub fn update_device_status(&mut self, device_id: &str, is_online: bool) {
        if let Some(device) = self.discovered_devices.get_mut(device_id) {
            device.is_online = is_online;
        }
    }
    
    pub fn get_devices(&self) -> Vec<Device> {
        let mut devices = vec![self.current_device.clone()];
        devices.extend(self.discovered_devices.values().cloned());
        devices
    }
    
    pub fn get_device_by_id(&self, device_id: &str) -> Option<&Device> {
        if self.current_device.id == device_id {
            Some(&self.current_device)
        } else {
            self.discovered_devices.get(device_id)
        }
    }
} 