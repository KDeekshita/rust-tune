# 🎧 RustTune – High Performance Music Streaming App

RustTune is an open-source music streaming application built using **Rust**, focused on performance, safety, and scalability. This project demonstrates how Rust can be used to build real-world web applications while providing a smooth and interactive music experience.

---

## 🚀 Features

### Available today
- ⚡ High-performance backend using Rust + [Actix Web](https://actix.rs/) 4
- 🏠 Responsive homepage UI (header, now-playing section, song list placeholders)
- 📦 Static asset serving via `actix-files` (`/static/*`)
- 🩺 JSON health endpoint at `/api/hello`

### Planned / in progress
- 🎵 Interactive music player (play / pause / seek / next / prev) — see open PRs
- 🎶 Songs API (`/api/songs`) — see open PRs
- 📂 Playlist creation and management
- 🔍 Search functionality

---

## 🌐 API Endpoints

| Method | Path          | Description                                  |
| ------ | ------------- | -------------------------------------------- |
| GET    | `/`           | Serves the homepage (`templates/index.html`) |
| GET    | `/api/hello`  | JSON health check                            |
| GET    | `/static/*`   | Static CSS / JS / asset files                |

---

## 🛠️ Tech Stack

- **Backend:** Rust, [Actix Web](https://actix.rs/) 4
- **Frontend:** HTML, CSS, vanilla JavaScript
- **Static serving:** `actix-files`
- **Version Control:** Git & GitHub

---

##  Prerequisites

- **Rust** (stable, edition 2021) — install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Cargo** (bundled with Rust)
- **Git**

Verify your installation:
```bash
rustc --version
cargo --version
```

---

## ⚙️ Installation

### 1. Clone the repository
```bash
git clone https://github.com/KDeekshita/rust-tune.git
cd rust-tune
```

### 2. Build the project
```bash
cargo build
```

### 3. Run the development server
```bash
cargo run
```

You should see:
```
Server running at http://127.0.0.1:8000
```

### 4. Open in your browser
Visit **http://127.0.0.1:8000**

---

## 📁 Project Structure

```
rust-tune/
├── Cargo.toml          # Rust dependencies
├── README.md
├── src/
│   └── main.rs         # Actix Web server + routes
├── static/
│   ├── css/style.css   # UI styling
│   └── js/script.js    # Music player logic
└── templates/
    └── index.html      # Homepage
```

---

## 🤝 Contribution Guidelines

Contributions are welcome and appreciated! Please follow the steps below.

### 1. Fork & clone
Fork the repository on GitHub, then clone your fork:
```bash
git clone https://github.com/<your-username>/rust-tune.git
cd rust-tune
git remote add upstream https://github.com/KDeekshita/rust-tune.git
```

### 2. Create a feature branch
Use a descriptive name with a prefix:
- `feat/<short-description>` — new features
- `fix/<short-description>` — bug fixes
- `docs/<short-description>` — documentation
- `refactor/<short-description>` — code cleanup

```bash
git checkout -b feat/your-feature-name
```

### 3. Make your changes
- Keep changes focused — **one feature/fix per PR**.
- Run `cargo fmt` and `cargo clippy` before committing.
- Make sure `cargo build` and `cargo run` succeed.

### 4. Commit using Conventional Commits
```
feat: add volume slider to player
fix: prevent crash when audio src is empty
docs: clarify installation steps
```

### 5. Push & open a Pull Request
```bash
git push origin feat/your-feature-name
```
Then open a PR against `main` of `KDeekshita/rust-tune`. In the description:
- Reference the related issue (e.g. `Closes #12`).
- Summarize what changed and why.
- Add screenshots/GIFs for UI changes.

### 6. Code Review
A maintainer will review your PR. Please respond to feedback and keep the branch up to date with `main`:
```bash
git fetch upstream
git rebase upstream/main
```

### Code of Conduct
Be respectful, inclusive, and constructive. Harassment of any kind will not be tolerated.

### Reporting Issues
Open a [GitHub Issue](https://github.com/KDeekshita/rust-tune/issues) with:
- A clear title
- Steps to reproduce (for bugs)
- Expected vs actual behavior
- Screenshots / logs if relevant

---

## 🎯 Goal of the Project

RustTune is designed to:
- Help developers learn **Rust through practical implementation**
- Provide a **beginner-friendly open-source project**
- Demonstrate how to build **scalable web applications**
- Encourage collaboration in open source

---

## 👩‍💻 Who Can Contribute?

- Beginners who want to learn Rust 🦀
- Developers interested in web development
- Open-source enthusiasts

