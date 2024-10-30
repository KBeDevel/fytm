use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = app.hide_menu();
            let _ = window.eval("window.location.replace('https://music.youtube.com')");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
