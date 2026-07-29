use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
    time::Duration,
};

use crate::{
    collect_report, collect_report_with_shell_snapshot,
    i18n::{self, Language},
    model::{Profile, Report},
    privacy,
    probes::shell::{ShellKind, ShellSessionSnapshot},
    renderers,
};

const MAX_REQUEST_BYTES: usize = 16 * 1024;
const INDEX_HTML: &str = include_str!("../docs/demo/prototype/index.html");
const APP_JS: &str = include_str!("../docs/demo/prototype/app.js");
const STYLES_CSS: &str = include_str!("../docs/demo/prototype/styles.css");
const OTTER_LAND: &str = include_str!("../docs/demo/assets/otter-guide.svg");
const OTTER_SWIM: &str = include_str!("../docs/demo/assets/otter-swim.svg");

/// A loopback-only GUI server that can be hosted by the CLI or a native shell.
pub struct LocalGuiServer {
    listener: TcpListener,
    profile: Profile,
    language: Language,
    shell_snapshot: Option<(ShellKind, ShellSessionSnapshot)>,
}

impl LocalGuiServer {
    /// Creates a server bound only to `127.0.0.1`.
    ///
    /// # Errors
    ///
    /// Returns an error when the loopback listener cannot be created.
    pub fn bind(
        profile: Profile,
        language: Language,
        shell_snapshot: Option<&(ShellKind, ShellSessionSnapshot)>,
        port: u16,
    ) -> Result<Self, String> {
        let listener = TcpListener::bind(("127.0.0.1", port))
            .map_err(|error| format!("could not bind the local GUI server: {error}"))?;
        Ok(Self {
            listener,
            profile,
            language,
            shell_snapshot: shell_snapshot.cloned(),
        })
    }

    /// Returns the local URL that the browser or native `WebView` should load.
    ///
    /// # Errors
    ///
    /// Returns an error when the assigned listener address cannot be inspected.
    pub fn url(&self) -> Result<String, String> {
        let address = self
            .listener
            .local_addr()
            .map_err(|error| format!("could not read the local GUI address: {error}"))?;
        Ok(format!(
            "http://127.0.0.1:{}/?mode=live&lang={}&profile={}",
            address.port(),
            self.language.code(),
            self.profile.label()
        ))
    }

    /// Serves the embedded read-only interface until the host process exits.
    ///
    /// # Errors
    ///
    /// Returns an error when the listener address cannot be inspected.
    pub fn run(self, open_browser: bool) -> Result<(), String> {
        let address = self
            .listener
            .local_addr()
            .map_err(|error| format!("could not read the local GUI address: {error}"))?;
        let url = self.url()?;

        println!(
            "{}\n  {url}\n{}",
            self.language.text(
                "ExecLocus local GUI is ready:",
                "ExecLocusローカルGUIを起動しました:"
            ),
            self.language.text(
                "Keep this terminal open. Press Ctrl+C to stop the local server.",
                "このターミナルを開いたままにしてください。終了するにはCtrl+Cを押します。"
            )
        );

        if open_browser {
            if let Err(error) = launch_browser(&url) {
                eprintln!(
                    "{}: {error}",
                    self.language.text(
                        "The browser could not be opened automatically; open the URL above",
                        "ブラウザを自動で開けませんでした。上記URLを手動で開いてください"
                    )
                );
            }
        }

        let expected_origin = format!("http://127.0.0.1:{}", address.port());
        let expected_host = format!("127.0.0.1:{}", address.port());
        for connection in self.listener.incoming() {
            match connection {
                Ok(mut stream) => {
                    if let Err(error) = handle_connection(
                        &mut stream,
                        self.profile,
                        self.language,
                        self.shell_snapshot.as_ref(),
                        &expected_origin,
                        &expected_host,
                    ) {
                        eprintln!("local GUI request failed: {error}");
                    }
                }
                Err(error) => eprintln!("local GUI connection failed: {error}"),
            }
        }
        Ok(())
    }
}

/// # Errors
///
/// Returns an error when the loopback listener cannot be created or inspected.
pub fn serve(
    profile: Profile,
    language: Language,
    shell_snapshot: Option<&(ShellKind, ShellSessionSnapshot)>,
    port: u16,
    open_browser: bool,
) -> Result<(), String> {
    LocalGuiServer::bind(profile, language, shell_snapshot, port)?.run(open_browser)
}

fn handle_connection(
    stream: &mut TcpStream,
    default_profile: Profile,
    default_language: Language,
    shell_snapshot: Option<&(ShellKind, ShellSessionSnapshot)>,
    expected_origin: &str,
    expected_host: &str,
) -> Result<(), String> {
    let request = match read_request(stream) {
        Ok(request) => request,
        Err(error) => {
            write_response(stream, 400, "text/plain; charset=utf-8", error.as_bytes())?;
            return Ok(());
        }
    };
    let route = request.path.split('?').next().unwrap_or(&request.path);

    match (request.method.as_str(), route) {
        ("GET", "/" | "/index.html") => write_response(
            stream,
            200,
            "text/html; charset=utf-8",
            INDEX_HTML.as_bytes(),
        ),
        ("GET", "/app.js") => write_response(
            stream,
            200,
            "text/javascript; charset=utf-8",
            APP_JS.as_bytes(),
        ),
        ("GET", "/styles.css") => write_response(
            stream,
            200,
            "text/css; charset=utf-8",
            STYLES_CSS.as_bytes(),
        ),
        ("GET", "/assets/otter-guide.svg") => {
            write_response(stream, 200, "image/svg+xml", OTTER_LAND.as_bytes())
        }
        ("GET", "/assets/otter-swim.svg") => {
            write_response(stream, 200, "image/svg+xml", OTTER_SWIM.as_bytes())
        }
        ("GET", "/api/health") => write_response(
            stream,
            200,
            "application/json; charset=utf-8",
            br#"{"status":"ready","network":"loopback-only","mode":"read-only"}"#,
        ),
        ("GET", "/favicon.ico") => write_response(stream, 204, "image/x-icon", &[]),
        ("POST", "/api/diagnose") => {
            if !authorized_api_request(&request, expected_origin, expected_host) {
                return write_response(
                    stream,
                    403,
                    "application/json; charset=utf-8",
                    br#"{"error":"request origin or diagnostic header was rejected"}"#,
                );
            }
            let profile = query_value(&request.path, "profile")
                .and_then(parse_profile)
                .unwrap_or(default_profile);
            let language = query_value(&request.path, "lang")
                .and_then(parse_language)
                .unwrap_or(default_language);
            let report = collect(profile, shell_snapshot);
            let body = diagnostic_payload(&report, language)
                .map_err(|error| format!("could not serialize GUI report: {error}"))?;
            write_response(
                stream,
                200,
                "application/json; charset=utf-8",
                body.as_bytes(),
            )
        }
        _ => write_response(
            stream,
            404,
            "application/json; charset=utf-8",
            br#"{"error":"not found"}"#,
        ),
    }
}

fn collect(profile: Profile, shell_snapshot: Option<&(ShellKind, ShellSessionSnapshot)>) -> Report {
    shell_snapshot.map_or_else(
        || collect_report(profile),
        |(shell, snapshot)| collect_report_with_shell_snapshot(profile, *shell, snapshot),
    )
}

fn diagnostic_payload(report: &Report, language: Language) -> serde_json::Result<String> {
    let local = i18n::localize_report(report, language);
    let shareable = privacy::redact_for_sharing(report);
    let shareable = i18n::localize_report(&shareable, language);
    let markdown = renderers::markdown::render_with_language(report, language);
    serde_json::to_string(&serde_json::json!({
        "mode": "live",
        "language": language.code(),
        "network": "loopback-only",
        "mutations": false,
        "report": local,
        "shareable_report": shareable,
        "shareable_markdown": markdown,
    }))
}

struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
}

fn read_request(stream: &mut TcpStream) -> Result<HttpRequest, String> {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|error| format!("could not set read timeout: {error}"))?;
    let mut request = Vec::new();
    let mut chunk = [0_u8; 2048];
    loop {
        let count = stream
            .read(&mut chunk)
            .map_err(|error| format!("could not read request: {error}"))?;
        if count == 0 {
            break;
        }
        request.extend_from_slice(&chunk[..count]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
        if request.len() > MAX_REQUEST_BYTES {
            return Err("request headers exceed the local GUI limit".to_owned());
        }
    }
    if request.len() > MAX_REQUEST_BYTES {
        return Err("request headers exceed the local GUI limit".to_owned());
    }
    let request = std::str::from_utf8(&request)
        .map_err(|_| "request headers are not valid UTF-8".to_owned())?;
    let header_text = request
        .split_once("\r\n\r\n")
        .map_or(request, |(headers, _)| headers);
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().ok_or("request line is missing")?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().ok_or("request method is missing")?;
    let path = parts.next().ok_or("request path is missing")?;
    let version = parts.next().ok_or("HTTP version is missing")?;
    if parts.next().is_some()
        || !matches!(version, "HTTP/1.0" | "HTTP/1.1")
        || !path.starts_with('/')
    {
        return Err("unsupported request line".to_owned());
    }
    let mut headers = HashMap::new();
    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            return Err("malformed request header".to_owned());
        };
        headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
    }
    if headers
        .get("content-length")
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|length| length > 0)
    {
        return Err("diagnostic requests must not contain a body".to_owned());
    }
    Ok(HttpRequest {
        method: method.to_owned(),
        path: path.to_owned(),
        headers,
    })
}

fn authorized_api_request(
    request: &HttpRequest,
    expected_origin: &str,
    expected_host: &str,
) -> bool {
    request.headers.get("host").is_some_and(|host| {
        host.eq_ignore_ascii_case(expected_host)
            || host.eq_ignore_ascii_case(&expected_host.replace("127.0.0.1", "localhost"))
    }) && request
        .headers
        .get("x-execlocus-request")
        .is_some_and(|value| value == "diagnose")
        && request
            .headers
            .get("origin")
            .is_none_or(|origin| origin.eq_ignore_ascii_case(expected_origin))
}

fn query_value<'a>(path: &'a str, key: &str) -> Option<&'a str> {
    path.split_once('?')?.1.split('&').find_map(|pair| {
        let (candidate, value) = pair.split_once('=')?;
        (candidate == key).then_some(value)
    })
}

fn parse_profile(value: &str) -> Option<Profile> {
    match value {
        "share-first" => Some(Profile::ShareFirst),
        "balanced" => Some(Profile::Balanced),
        "linux-first" => Some(Profile::LinuxFirst),
        _ => None,
    }
}

fn parse_language(value: &str) -> Option<Language> {
    match value {
        "en" => Some(Language::English),
        "ja" => Some(Language::Japanese),
        _ => None,
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> Result<(), String> {
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        _ => "Error",
    };
    let headers = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: default-src 'self'; style-src 'self'; script-src 'self'; img-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'\r\n\r\n",
        body.len()
    );
    stream
        .write_all(headers.as_bytes())
        .and_then(|()| stream.write_all(body))
        .map_err(|error| format!("could not write response: {error}"))
}

#[cfg(target_os = "windows")]
fn launch_browser(url: &str) -> Result<(), String> {
    Command::new("rundll32.exe")
        .args(["url.dll,FileProtocolHandler", url])
        .spawn()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg(not(target_os = "windows"))]
fn launch_browser(url: &str) -> Result<(), String> {
    let program = if std::env::var_os("WSL_INTEROP").is_some() {
        "explorer.exe"
    } else {
        "xdg-open"
    };
    Command::new(program)
        .arg(url)
        .spawn()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{i18n::Language, model::Profile};

    use super::{
        HttpRequest, LocalGuiServer, authorized_api_request, parse_language, parse_profile,
        query_value,
    };

    #[test]
    fn local_gui_server_uses_an_assigned_loopback_port() {
        let server = LocalGuiServer::bind(Profile::Balanced, Language::English, None, 0)
            .expect("loopback bind should succeed");
        let url = server.url().expect("assigned URL should be readable");
        assert!(url.starts_with("http://127.0.0.1:"));
        assert!(url.ends_with("/?mode=live&lang=en&profile=balanced"));
    }

    #[test]
    fn query_values_are_bounded_to_known_profiles_and_languages() {
        let path = "/api/diagnose?profile=linux-first&lang=ja";
        assert_eq!(
            query_value(path, "profile").and_then(parse_profile),
            Some(Profile::LinuxFirst)
        );
        assert_eq!(
            query_value(path, "lang").and_then(parse_language),
            Some(Language::Japanese)
        );
        assert_eq!(parse_profile("../../private"), None);
    }

    #[test]
    fn diagnostic_api_requires_loopback_host_origin_and_custom_header() {
        let mut headers = HashMap::from([
            ("host".to_owned(), "127.0.0.1:43117".to_owned()),
            ("origin".to_owned(), "http://127.0.0.1:43117".to_owned()),
            ("x-execlocus-request".to_owned(), "diagnose".to_owned()),
        ]);
        let request = |headers| HttpRequest {
            method: "POST".to_owned(),
            path: "/api/diagnose".to_owned(),
            headers,
        };
        assert!(authorized_api_request(
            &request(headers.clone()),
            "http://127.0.0.1:43117",
            "127.0.0.1:43117"
        ));
        headers.insert("origin".to_owned(), "https://example.invalid".to_owned());
        assert!(!authorized_api_request(
            &request(headers),
            "http://127.0.0.1:43117",
            "127.0.0.1:43117"
        ));
    }
}
