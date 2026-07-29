#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use execlocus::{gui::LocalGuiServer, i18n::Language, model::Profile};
use tauri::{Manager, Theme, WebviewUrl, WebviewWindowBuilder};

const APP_TITLE: &str = "ExecLocus";

#[cfg(windows)]
fn style_native_title_bar(temporary_title: &str) -> Result<(), String> {
    use winsafe::{COLORREF, DwmAttr, HWND};

    let hwnd = HWND::FindWindow(None, Some(temporary_title))
        .map_err(|error| format!("failed to locate the native app window: {error}"))?
        .ok_or_else(|| "the native app window was not found".to_owned())?;

    for attribute in [
        DwmAttr::CaptionColor(COLORREF::from_rgb(0x0a, 0x18, 0x1e)),
        DwmAttr::TextColor(COLORREF::from_rgb(0xeb, 0xfb, 0xf7)),
        DwmAttr::BorderColor(COLORREF::from_rgb(0x26, 0x44, 0x4e)),
    ] {
        hwnd.DwmSetWindowAttribute(attribute)
            .map_err(|error| format!("failed to style the native title bar: {error}"))?;
    }

    Ok(())
}

fn main() {
    let server = LocalGuiServer::bind(Profile::Balanced, Language::Japanese, None, 0)
        .expect("the loopback-only diagnostic server could not start");
    let url = server
        .url()
        .expect("the loopback-only diagnostic URL could not be read");

    std::thread::Builder::new()
        .name("execlocus-loopback".to_owned())
        .spawn(move || {
            if let Err(error) = server.run_embedded() {
                eprintln!("ExecLocus desktop diagnostic server stopped: {error}");
            }
        })
        .expect("the loopback-only diagnostic thread could not start");

    tauri::Builder::default()
        .on_window_event(|window, event| {
            if window.label() == "main"
                && matches!(event, tauri::WindowEvent::CloseRequested { .. })
            {
                window.app_handle().exit(0);
            }
        })
        .setup(move |app| {
            let external_url: tauri::Url = url
                .parse()
                .map_err(|error| format!("invalid loopback-only GUI URL: {error}"))?;
            let allowed_port = external_url
                .port()
                .ok_or("the loopback-only GUI URL has no assigned port")?;
            let temporary_title = format!("{APP_TITLE}-starting-{}", std::process::id());
            let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(external_url))
                .title(&temporary_title)
                .theme(Some(Theme::Dark))
                .inner_size(1280.0, 820.0)
                .min_inner_size(980.0, 680.0)
                .on_navigation(move |candidate| {
                    candidate.scheme() == "http"
                        && candidate.host_str() == Some("127.0.0.1")
                        && candidate.port() == Some(allowed_port)
                })
                .build()?;

            #[cfg(windows)]
            if let Err(error) = style_native_title_bar(&temporary_title) {
                eprintln!("ExecLocus kept the system title-bar colors: {error}");
            }

            window.set_title(APP_TITLE)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("ExecLocus desktop exited unexpectedly");
}
