use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, AppHandle,
};

pub fn create_system_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItemBuilder::with_id("show", "显示窗口").build(app)?;
    let hide = MenuItemBuilder::with_id("hide", "隐藏窗口").build(app)?;
    let start_scan = MenuItemBuilder::with_id("start_scan", "开始扫描设备").build(app)?;
    let transfer_history = MenuItemBuilder::with_id("history", "传输历史").build(app)?;
    let dark_mode = MenuItemBuilder::with_id("dark_mode", "深色主题").build(app)?;
    let auto_start = MenuItemBuilder::with_id("auto_start", "开机启动").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
    
    let menu = MenuBuilder::new(app)
        .item(&show)
        .item(&hide)
        .separator()
        .item(&start_scan)
        .item(&transfer_history)
        .separator()
        .item(&dark_mode)
        .item(&auto_start)
        .separator()
        .item(&quit)
        .build()?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(handle_system_tray_event)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}

pub fn handle_system_tray_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "quit" => {
            app.exit(0);
        }
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "hide" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }
        }
        "start_scan" => {
            let _ = app.emit("tray-start-scan", ());
        }
        "history" => {
            let _ = app.emit("tray-show-history", ());
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "dark_mode" => {
            let _ = app.emit("tray-toggle-theme", ());
        }
        "auto_start" => {
            let _ = app.emit("tray-toggle-auto-start", ());
        }
        _ => {}
    }
}

// 显示托盘通知
pub fn show_tray_notification(app: &AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;
    
    let _ = app
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show();
} 