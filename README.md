# Rust URL Shortener

This project now has a browser server and a lightweight command-line interface. The CLI is the better fit for this codebase because the core logic is simple, the data lives in JSON, and the existing web server already covers the visual experience.

## Features
- URL shortening into unique short codes.
- Redirection back to the saved long URL.
- Persistent JSON storage for links and contact submissions so that shortened links and contact info survive server restarts.
- A polished Nintendo-style browser UI for the web flow & some *nostalgia*.
- A CLI for serving, shortening, resolving, and listing links.

## CLI Commands
- Start the server: 
```bash
cargo run -- serve
```
- Shorten a URL:
```bash
# Replace with desired link
cargo run -- shorten "https://example.com"
```
- Resolve a code:
```bash
# Replace with the shortened code
cargo run -- resolve /ABC123
```
- List stored links:
```bash
cargo run -- list
```
The default command is `serve`, so `cargo run` still starts the web server.

## Setup
1. Clone the repository.
```bash
git clone https://github.com/bandhuamajdoor/url-shortener.git
cd url-shortener
```
2. Install Rust if needed.
3. Run the server: `cargo run`
4. Use the CLI commands above when you want terminal-based access.

## Technologies Used
- Rust
- Serde / serde_json
- rand
- clap
- Standard library TCP networking
