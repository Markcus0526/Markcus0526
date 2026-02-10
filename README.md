# Trending Rust Repositories 🔥🦀

<!-- Project badges -->
<p>
  <img src="https://img.shields.io/github/stars/Markcus0526/Markcus0526?style=social" alt="GitHub stars">
  <img src="https://img.shields.io/github/forks/Markcus0526/Markcus0526?style=social" alt="GitHub forks">
  <img src="https://img.shields.io/github/license/Markcus0526/Markcus0526" alt="License">
  <img src="https://img.shields.io/github/languages/top/Markcus0526/Markcus0526" alt="Top language">
  <img src="https://github.com/Markcus0526/Markcus0526/actions/workflows/update_readme.yml/badge.svg" alt="Workflow status">
</p>

A small Rust utility that fetches the most-starred Rust repositories and "agent"-related Rust repositories from GitHub and injects the results into this repository's README tables. The update runs automatically via a scheduled GitHub Action and can also be run locally.

---

## 👋 About Me
- Hi — I'm Markcus. I build small automation tools and enjoy working with Rust, APIs, and developer tooling.
- This repository is a playful automation: a Rust program that keeps the "Trending Rust Repositories" section of this README fresh by querying GitHub and updating the tables daily.

---

## 🧭 Core Expertise
- 🦀 Rust: system-level and async programming
- ⚙️ Automation: CI/CD and scheduled updates (GitHub Actions)
- 🔌 Web APIs: HTTP clients, JSON parsing, integrations
- 🛠️ Tooling: CLI utilities, tooling for developer productivity
- 📦 Data handling: parsing, formatting, and generating documentation

---

## 🧰 Technology Stack
- Language: Rust (edition 2021)
- Async runtime: tokio
- HTTP client: reqwest (with JSON feature)
- Serialization: serde / serde_json
- CI: GitHub Actions (scheduled + workflow_dispatch)
- Build system: Cargo

Key files:
- rust/Cargo.toml — project manifest and dependency list
- rust/src/main.rs — main application (fetches GitHub Search API, updates README)
- .github/workflows/update_readme.yml — scheduled workflow that runs cargo run and commits README changes

Why these choices:
- reqwest + tokio = robust async HTTP client for performing concurrent API calls.
- serde = convenient, type-safe JSON (de)serialization.
- GitHub Actions = simple scheduled automation in the same repository.

---

## ⚙️ How it works (summary)
1. The Rust program calls the GitHub Search API for:
   - top Rust repositories by stars
   - Rust repositories related to agents/AI (search by keywords)
2. It formats results into Markdown tables.
3. It opens the repository README (the program expects the README at "../README.md" when run from the rust/ directory), inserts/updates the "Top Trending Rust Repositories" and "Top Trending Rust Agent Repositories" sections, and writes changes.
4. The GitHub Action runs the program daily (cron at midnight) and commits/pushes updates.

---

## ▶️ Run locally — quickstart
Prerequisites:
- Rust toolchain (stable)
- Network access to api.github.com
- A GitHub token is NOT required for basic requests, but unauthenticated requests are subject to stricter rate limits (see Troubleshooting).

Commands:
1. Clone the repo and change into the directory that contains rust/ and README.md.
2. From the root of the repository run:
```bash
# Ensure you are at the repo root
cd rust
cargo run --release
```
Notes:
- The program writes to ../README.md (relative to rust/). Run cargo from the rust/ directory so the path resolves correctly.
- If you prefer debugging builds, omit --release.

Expected output:
- The program prints "README.md updated successfully!" on success.
- If run locally, check the README.md diff to review the modifications before committing.

---

## 🔁 CI / Automation
- File: .github/workflows/update_readme.yml
- The workflow:
  - Checks out the repo
  - Sets up Rust toolchain (stable)
  - Runs the Rust program (cargo run in rust/)
  - Commits and pushes README.md if there are changes
- Schedule: daily at midnight (cron). Also supports manual runs via "Workflow dispatch".

---

## 📊 GitHub Analytics
Live repository badges (dynamic):
<p>
  <img src="https://img.shields.io/github/stars/Markcus0526/Markcus0526" alt="Stars">
  <img src="https://img.shields.io/github/forks/Markcus0526/Markcus0526" alt="Forks">
  <img src="https://img.shields.io/github/issues/Markcus0526/Markcus0526" alt="Open issues">
  <img src="https://img.shields.io/github/last-commit/Markcus0526/Markcus0526" alt="Last commit">
</p>

Note: The README contents (the trending tables) are generated programmatically by rust/src/main.rs. If you want to change the search query or formatting, edit that file and test locally.

---

## 🛠️ Troubleshooting & Tips
- Rate limiting: Unauthenticated requests to the GitHub API have limited rate. If you encounter 403 responses, consider:
  - Setting up a GitHub token and modifying the client to send an Authorization header (not currently implemented).
  - Reducing frequency of requests during testing.
- README path: The program opens "../README.md". If you move the rust/ directory or run cargo from a different working directory, the program may create or modify a different README path.
- JSON schema mismatches: The program expects certain fields from the Search API. If GitHub changes the API responses, adjust structs in rust/src/main.rs.

---

## 🤝 Contributing
- Contributions, suggestions, and fixes are welcome!
- Suggested workflow:
  - Fork this repository
  - Create a feature branch
  - Update rust/src/main.rs or README templates as needed
  - Test locally: cd rust && cargo run
  - Open a pull request with a clear description

---

## 📬 Connect With Me
- GitHub: https://github.com/Markcus0526
- Email: markcus (replace with your real email) — markcus@example.com
- Twitter: https://twitter.com/yourhandle (replace with your handle)
- LinkedIn: https://linkedin.com/in/yourprofile (replace with your profile)

Feel free to open issues or PRs — I appreciate feedback and collaboration!

---

## ⚖️ License
This project is licensed under the MIT License — see the LICENSE file for details.

---

Thank you for visiting — happy hacking! ✨