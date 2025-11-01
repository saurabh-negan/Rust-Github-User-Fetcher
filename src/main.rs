use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

/// 🧑‍💻 Simple GitHub User Info Fetcher
#[derive(Parser, Debug)]
#[command(author, version, about = "Fetch GitHub user info by username")]
struct Args {
    /// GitHub username
    #[arg(short, long)]
    username: String,
}

#[derive(Deserialize, Debug)]
struct GitHubUser {
    login: String,
    name: Option<String>,
    bio: Option<String>,
    public_repos: u32,
    followers: u32,
    following: u32,
    location: Option<String>,
    html_url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Create a reqwest client with a user-agent header
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Rust-GitHub-CLI"));

    let client = Client::builder().default_headers(headers).build()?;

    let url = format!("https://api.github.com/users/{}", args.username);

    println!("🔍 Fetching GitHub data for '{}'...\n", args.username);

    let response = client.get(&url).send()?;

    if !response.status().is_success() {
        eprintln!("❌ User not found or request failed! (Status: {})", response.status());
        return Ok(());
    }

    let user: GitHubUser = response.json()?;

    println!("👤 Username: {}", user.login);
    println!("🧾 Name: {}", user.name.unwrap_or("Not provided".into()));
    println!("📍 Location: {}", user.location.unwrap_or("Not specified".into()));
    println!("📚 Public Repos: {}", user.public_repos);
    println!("👥 Followers: {}", user.followers);
    println!("➡️  Following: {}", user.following);
    println!("💬 Bio: {}", user.bio.unwrap_or("No bio available".into()));
    println!("🔗 Profile: {}", user.html_url);

    Ok(())
}
