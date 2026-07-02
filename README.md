# Rust URL Shortener

This project now has a browser server and a lightweight command-line interface. The CLI is the better fit for this codebase because the core logic is simple, the data lives in JSON, and the existing web server already covers the visual experience.

## Features
- URL shortening into unique short codes.
- Redirection back to the saved long URL.
- Persistent JSON storage for links and contact submissions.
- A polished Nintendo-style browser UI for the web flow.
- A CLI for serving, shortening, resolving, and listing links.

## CLI Commands
- Start the server: `cargo run -- serve`
- Shorten a URL: `cargo run -- shorten "https://example.com"`
- Resolve a code: `cargo run -- resolve /ABC123`
- List stored links: `cargo run -- list`

The default command is `serve`, so `cargo run` still starts the web server.

## Browser UI
The served pages in `public/` were redesigned to match the console-chrome inspiration in `DESIGN.md`: layered periwinkle panels, carbon nav bars, amber action chips, bold outlined hero text, and compact card-style sections.

## Technologies Used
- Rust
- Serde / serde_json
- rand
- clap
- Standard library TCP networking

## Setup
1. Clone the repository.
2. Install Rust if needed.
3. Run the server: `cargo run`
4. Use the CLI commands above when you want terminal-based access.