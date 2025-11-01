# 🧑‍💻 GitHub User Fetcher (Rust)

A simple command-line tool that fetches and displays information about a GitHub user using the [GitHub REST API](https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28).  
Built entirely in **Rust**, this project demonstrates API integration, JSON parsing, and CLI argument parsing.

---

## 🚀 Features
- Fetch user profile info by username  
- Display name, bio, location, follower stats, and repo count  
- Uses GitHub’s public API (no auth required)  
- Clean and colorful CLI-style output  

---

## 🧩 Example Usage

```bash
cargo run -- --username saurabh-negan
