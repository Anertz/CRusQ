// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _};
use qrcode::QrCode;
use serde::Serialize;
use tauri::{Manager, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(Serialize)]
struct QrResponse {
    image: String,
    text: String,
}

#[tauri::command]
async fn generate_qr_from_clipboard(window: Window) -> Result<QrResponse, String> {
    // クリップボードからテキストを取得
    let clipboard_text = window
        .app_handle()
        .clipboard()
        .read_text()
        .map_err(|e| format!("Failed to read clipboard: {}", e))?;

    if clipboard_text.is_empty() {
        return Err("Clipboard is empty".to_string());
    }

    // QRコード生成 (誤り訂正レベル: High)
    let code = QrCode::with_error_correction_level(&clipboard_text, qrcode::EcLevel::H)
        .map_err(|e| format!("Failed to generate QR code: {}", e))?;

    // PNG形式で直接エンコード
    let image = code.render::<image::Luma<u8>>()
        .quiet_zone(false)
        .min_dimensions(200, 200)
        .build();

    // PNG形式でエンコード
    let mut png_data = Vec::new();
    image
        .write_to(
            &mut std::io::Cursor::new(&mut png_data),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    // Base64エンコード
    let base64_image = general_purpose::STANDARD.encode(&png_data);

    Ok(QrResponse {
        image: format!("data:image/png;base64,{}", base64_image),
        text: clipboard_text,
    })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::{apply_acrylic, apply_mica};
                // Windows 11ならMica、それ以外はAcrylicを試す
                if apply_mica(&window, Some(true)).is_err() {
                    let _ = apply_acrylic(&window, Some((18, 18, 18, 125)));
                }
            }

            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::apply_vibrancy;
                let _ = apply_vibrancy(
                    &window,
                    window_vibrancy::NSVisualEffectMaterial::HudWindow,
                    None,
                    None,
                );
            }

            #[cfg(target_os = "linux")]
            {
                let _ = window.set_decorations(false);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate_qr_from_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
