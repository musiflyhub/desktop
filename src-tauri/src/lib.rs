use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use tauri_plugin_deep_link::DeepLinkExt;

#[tauri::command]
fn check_connection() -> bool {
    // Dynamically resolve the domain to its current IP (Cloudflare friendly)
    if let Ok(mut addrs) = "open.musifly.net:443".to_socket_addrs() {
        if let Some(addr) = addrs.next() {
            return TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok();
        }
    }
    false
}

#[tauri::command]
async fn show_main_window(app_handle: tauri::AppHandle) {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        if check_connection() {
            // Online: Ensure we are pointing to the live site
            let _ = main_window.navigate("https://open.musifly.net/".parse().unwrap());
        } else {
            // Offline: Redirect to local offline page
            let _ = main_window.navigate("tauri://localhost/offline.html".parse().unwrap());
        }
        
        let _ = main_window.show();
        let _ = main_window.set_focus();
    }
    
    if let Some(updater_window) = app_handle.get_webview_window("updater") {
        let _ = updater_window.close();
    }
}

#[tauri::command]
fn restart_app(app_handle: tauri::AppHandle) {
    app_handle.restart();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_main_window(app.clone());
        }))
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Initialize deep linking
            app.handle().plugin(tauri_plugin_deep_link::init())?;

            #[cfg(desktop)]
            app.deep_link().register("musifly")?;

            // Listen for deep links
            let app_handle = app.handle().clone();
            app.handle().deep_link().on_open_url(move |event| {
                let urls = event.urls();
                if let Some(url) = urls.first() {
                    let url_str = url.to_string();
                    // Example: musifly://some/path -> https://open.musifly.net/some/path
                    if let Some(path) = url_str.strip_prefix("musifly://") {
                        if let Some(main_window) = app_handle.get_webview_window("main") {
                            let redirect_url = format!("https://open.musifly.net/{}", path);
                            if let Ok(parsed_url) = redirect_url.parse() {
                                let _ = main_window.navigate(parsed_url);
                                let _ = main_window.show();
                                let _ = main_window.set_focus();
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Only intercept close for the main window
                if window.label() == "main" {
                    api.prevent_close();
                    window.hide().unwrap();
                }
            }
        })
        .on_page_load(|window, _payload| {
            let _ =
                window.eval("document.addEventListener('contextmenu', e => e.preventDefault());");
        })
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            restart_app,
            check_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
