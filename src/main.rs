use clap::{Parser, Subcommand};
use std::net::TcpListener;

mod methods;
use methods::{get_response, open_page, render_shortened_page};

mod contact;
use contact::{decode_form_body, save_to_json_file};

mod shorten;
use shorten::{decode_url_form_body, generate_short_code};

mod redirection;
use redirection::{find_long_url_for_code, redirect_to};

mod storage;
use storage::{list_url_maps, save_url_map, URLmap};

const DEFAULT_SHORTENED_FILE: &str = "data/shortened.json";
const DEFAULT_CONTACT_FILE: &str = "data/submissions.json";

#[derive(Parser)]
#[command(
    name = "url-shortener",
    about = "Rust URL shortener with a browser server and command-line tools"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the HTTP server
    Serve {
        #[arg(long, default_value = "127.0.0.1:8080")]
        address: String,
    },
    /// Create a new short code for a URL
    Shorten {
        url: String,
        #[arg(long, default_value = DEFAULT_SHORTENED_FILE)]
        file: String,
    },
    /// Resolve a short code back to its original URL
    Resolve {
        code: String,
        #[arg(long, default_value = DEFAULT_SHORTENED_FILE)]
        file: String,
    },
    /// List all saved short links
    List {
        #[arg(long, default_value = DEFAULT_SHORTENED_FILE)]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Serve {
        address: String::from("127.0.0.1:8080"),
    }) {
        Commands::Serve { address } => run_server(&address),
        Commands::Shorten { url, file } => shorten_url(&file, &url),
        Commands::Resolve { code, file } => resolve_code(&file, &code),
        Commands::List { file } => list_links(&file),
    }
}

fn run_server(address: &str) {
    let listener = TcpListener::bind(address).expect("Failed to bind");
    println!("Listening at {}", address);

    for stream in listener.incoming() {
        let stream = stream.expect("could not receive stream");
        let (path, method, body) = get_response(stream.try_clone().expect("Failed to clone stream"));

        let long_url = find_long_url_for_code(path.as_str());
        if long_url.is_some() {
            let url = long_url.unwrap();
            println!("Redirecting to: {}", url);
            redirect_to(stream.try_clone().expect("failed to clone stream"), url.as_str());
            continue;
        }

        if path == "/shorten" && method == "POST" {
            let url = decode_url_form_body(&body).to_string();
            let short_code = generate_short_code();
            save_url_map(
                DEFAULT_SHORTENED_FILE,
                URLmap {
                    longurl: url.clone(),
                    code: format!("/{}", short_code.clone()),
                },
            )
            .expect("Failed to save shortened URL");
            println!("Shortened URL: {} -> {}", url, short_code);
            render_shortened_page(
                short_code.as_str(),
                stream.try_clone().expect("Failed to clone stream"),
            );
            continue;
        }

        if path == "/contact" && method == "POST" {
            let form_data = decode_form_body(&body);
            save_to_json_file(&form_data, DEFAULT_CONTACT_FILE);
        }

        open_page(path.as_str(), stream.try_clone().expect("Failed to clone stream"));
    }
}

fn shorten_url(file_path: &str, url: &str) {
    let short_code = generate_short_code();
    let short_path = format!("/{}", short_code);

    save_url_map(
        file_path,
        URLmap {
            longurl: url.to_string(),
            code: short_path.clone(),
        },
    )
    .expect("Failed to save shortened URL");

    println!("Shortened URL saved");
    println!("Long URL : {}", url);
    println!("Short URL: http://localhost:8080{}", short_path);
}

fn resolve_code(file_path: &str, code: &str) {
    match storage::find_long_url_for_code(file_path, code) {
        Some(url) => {
            println!("{} -> {}", code, url);
        }
        None => {
            eprintln!("No mapping found for {}", code);
            std::process::exit(1);
        }
    }
}

fn list_links(file_path: &str) {
    let links = list_url_maps(file_path);

    if links.is_empty() {
        println!("No shortened URLs found in {}", file_path);
        return;
    }

    println!("{:<16} | Long URL", "Short Code");
    println!("{}", "-".repeat(80));

    for link in links {
        println!("{:<16} | {}", link.code, link.longurl);
    }
}
