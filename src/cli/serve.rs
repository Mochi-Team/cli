use anyhow::{Context, Result};
use local_ip_address::local_ip;

use std::{collections::HashMap, ffi::OsStr, fs::File, io, path::PathBuf, str::FromStr};

use clap::Parser;
use tiny_http::{Header, Response, Server, StatusCode};

use super::build;

pub type WebserverCmd = ServeArguments;

#[derive(Parser, Default, Debug)]
pub struct ServeArguments {
    // The port to broadcast the repository (default is 10443)
    #[arg(long, default_value = None)]
    port: Option<usize>,

    // The repository output (default is "dist").
    #[arg(long, default_value = None)]
    output: Option<PathBuf>,
}

pub fn handle(cmd: WebserverCmd) -> Result<()> {
    start_webserver(cmd)
}

fn start_webserver(cmd: WebserverCmd) -> Result<()> {
    println!("Generating server files...");

    let dist_path = cmd.output.unwrap_or(
        std::env::current_dir()
            .with_context(|| "failed to get current working directory")?
            .join("dist"),
    );

    build::handle(build::BuildCmd {
        path: None,
        output: Some(dist_path.clone()),
        site: true,
    })?;

    println!("Starting webserver...");

    let port = cmd.port.unwrap_or(10443);

    let server = Server::http(format!("{}:{}", "0.0.0.0", port))
        .expect("There was an issue starting local server.");

    println!(
        "starting server at http://{}:{}",
        local_ip()
            .map(|i| i.to_string())
            .unwrap_or("0.0.0.0".into()),
        port
    );

    for request in server.incoming_requests() {
        let mut file_path = dist_path.clone();

        if request.url().len() > 1 {
            for chunk in request.url().trim_start_matches('/').split('/') {
                file_path.push(chunk);
            }
        };

        println!("Requested file: {}", file_path.display());

        if file_path == dist_path {
            _ = handle_file_response(request, &dist_path.join("index").with_extension("html"));
        } else if !file_path.is_file() {
            let status = StatusCode(404);
            println!("Status: {} ({})", status.default_reason_phrase(), status.0);
            request.respond(Response::empty(status))?;
        } else {
            _ = handle_file_response(request, &file_path);
        }
    }
    unreachable!("`incoming_requests` never stops")
}

fn handle_file_response(request: tiny_http::Request, path: &PathBuf) -> Result<(), io::Error> {
    let content_type_by_extension: HashMap<&'static str, &'static str> = [
        ("html", "text/plain"),
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("png", "image/png"),
        ("wasm", "application/wasm"),
    ]
    .iter()
    .cloned()
    .collect();

    match File::open(path) {
        Ok(file) => {
            let mut response = Response::from_file(file);
            let content_type = path
                .extension()
                .and_then(OsStr::to_str)
                .and_then(|ext| content_type_by_extension.get(ext).copied())
                .unwrap_or("text/plain");
            response.add_header(
                Header::from_str(&format!("Content-Type: {}", content_type))
                    .map_err(|_| io::Error::from(io::ErrorKind::Other))?,
            );
            request.respond(response)
        }
        Err(err) => {
            let status = StatusCode(500);
            println!("Status: {} ({})", status.default_reason_phrase(), status.0);
            println!("Error: {:?}", err);
            request.respond(Response::empty(status))
        }
    }
}
