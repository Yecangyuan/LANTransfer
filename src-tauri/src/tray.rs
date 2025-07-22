use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu, WindowEvent,
};

pub fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let show = CustomMenuItem::new("show".to_string(), "显示窗口");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口");
    let separator = SystemTrayMenuItem::Separator;
    
    // 传输相关菜单
    let start_scan = CustomMenuItem::new("start_scan".to_string(), "开始扫描设备");
    let transfer_history = CustomMenuItem::new("history".to_string(), "传输历史");
    
    // 设置子菜单
    let dark_mode = CustomMenuItem::new("dark_mode".to_string(), "深色主题");
    let auto_start = CustomMenuItem::new("auto_start".to_string(), "开机启动");
    let settings_submenu = SystemTraySubmenu::new(
        "设置",
        SystemTrayMenu::new()
            .add_item(dark_mode)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(auto_start),
    );
    
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(separator.clone())
        .add_item(start_scan)
        .add_item(transfer_history)
        .add_native_item(separator.clone())
        .add_submenu(settings_submenu)
        .add_native_item(separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            // 左键点击显示/隐藏窗口
            let window = app.get_window("main").unwrap();
            if window.is_visible().unwrap_or(false) {
                let _ = window.hide();
            } else {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            // 右键点击显示菜单（自动处理）
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            // 双击显示窗口
            let window = app.get_window("main").unwrap();
            let _ = window.show();
            let _ = window.set_focus();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    // 退出应用
                    app.exit(0);
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.hide();
                }
                "start_scan" => {
                    // 触发设备扫描
                    let _ = app.emit_all("tray-start-scan", {});
                }
                "history" => {
                    // 显示传输历史
                    let _ = app.emit_all("tray-show-history", {});
                    let window = app.get_window("main").unwrap();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
                "dark_mode" => {
                    // 切换主题
                    let _ = app.emit_all("tray-toggle-theme", {});
                }
                "auto_start" => {
                    // 切换开机启动
                    let _ = app.emit_all("tray-toggle-auto-start", {});
                }
                _ => {}
            }
        }
        _ => {}
    }
}

// 窗口事件处理
pub fn handle_window_event(event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            // 阻止窗口关闭，改为隐藏到托盘
            api.prevent_close();
            // 获取窗口引用并隐藏
            if let Some(window) = api.window().get_window("main") {
                let _ = window.hide();
            }
        }
        _ => {}
    }
}

// 更新托盘图标状态
pub fn update_tray_status(app: &AppHandle, is_transferring: bool) {
    let tray_handle = app.tray_handle();
    
    if is_transferring {
        // 传输中状态 - 可以考虑使用不同的图标
        let _ = tray_handle.set_tooltip("LANTransfer - 传输中...");
    } else {
        let _ = tray_handle.set_tooltip("LANTransfer - 就绪");
    }
}

// 显示托盘通知
pub fn show_tray_notification(app: &AppHandle, title: &str, body: &str) {
    use tauri::api::notification::Notification;
    
    let _ = Notification::new(&app.config().tauri.bundle.identifier)
        .title(title)
        .body(body)
        .icon("icons/32x32.png")
        .show();
} 