#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use execlocus::{gui::LocalGuiServer, i18n::Language, model::Profile};
use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    let server = LocalGuiServer::bind(Profile::Balanced, Language::Japanese, None, 0)
        .expect("the loopback-only diagnostic server could not start");
    let url = server
        .url()
        .expect("the loopback-only diagnostic URL could not be read");

    std::thread::Builder::new()
        .name("execlocus-loopback".to_owned())
        .spawn(move || {
            if let Err(error) = server.run(false) {
                eprintln!("ExecLocus desktop diagnostic server stopped: {error}");
            }
        })
        .expect("the loopback-only diagnostic thread could not start");

    tauri::Builder::default()
        .setup(move |app| {
            let external_url: tauri::Url = url
                .parse()
                .map_err(|error| format!("invalid loopback-only GUI URL: {error}"))?;
            let allowed_port = external_url
                .port()
                .ok_or("the loopback-only GUI URL has no assigned port")?;
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(external_url))
                .title("ExecLocus")
                .inner_size(1280.0, 820.0)
                .min_inner_size(980.0, 680.0)
                .on_navigation(move |candidate| {
                    candidate.scheme() == "http"
                        && candidate.host_str() == Some("127.0.0.1")
                        && candidate.port() == Some(allowed_port)
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("ExecLocus desktop exited unexpectedly");
}
