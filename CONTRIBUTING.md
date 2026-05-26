# Contributing to RustTune 🎵🦀

Thank you for your interest in contributing to **RustTune** — an open-source music streaming app built with Rust, Actix Web, and vanilla JavaScript! Whether you're fixing a bug, improving the UI, or optimizing backend performance, every contribution counts.

This guide will get you from zero to your first merged PR.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
  - [Fork & Clone the Repository](#fork--clone-the-repository)
  - [Setting Up the Development Environment](#setting-up-the-development-environment)
- [Project Structure](#project-structure)
- [Branch Naming Conventions](#branch-naming-conventions)
- [Making Changes](#making-changes)
  - [Commit Message Style](#commit-message-style)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Reporting Issues](#reporting-issues)
- [Need Help?](#need-help)

---

## Code of Conduct

By participating in this project, you agree to keep this a respectful, inclusive, and beginner-friendly space. Be constructive, patient, and kind — especially with first-time contributors.

---

## Getting Started

### Fork & Clone the Repository

1. **Fork** this repository by clicking the **Fork** button at the top-right of the [RustTune GitHub page](https://github.com/KDeekshita/rust-tune).

2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/rust-tune.git
   cd rust-tune
   ```

3. **Add the upstream remote** to keep your fork in sync with the original:
   ```bash
   git remote add upstream https://github.com/KDeekshita/rust-tune.git
   ```

4. **Verify your remotes:**
   ```bash
   git remote -v
   # origin    https://github.com/YOUR-USERNAME/rust-tune.git (fetch)
   # upstream  https://github.com/KDeekshita/rust-tune.git (fetch)
   ```

---

### Setting Up the Development Environment

#### Prerequisites

Make sure you have the following installed:

| Tool | Version | Purpose |
|------|---------|---------|
| [Rust](https://rustup.rs/) | Stable (latest) | Backend language |
| [Cargo](https://doc.rust-lang.org/cargo/) | Comes with Rust | Build & dependency manager |
| Git | Any recent version | Version control |
| A modern browser | Chrome / Firefox / Edge | Frontend testing |

#### Steps

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   rustc --version  # Verify install
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run the development server:**
   ```bash
   cargo run
   ```
   The app will be available at `http://localhost:8080` (or as configured).

4. **Run tests** to make sure everything is working:
   ```bash
   cargo test
   ```

5. **Keep your fork up to date** before starting any new work:
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

---

## Project Structure

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

- **Backend changes** → work inside `src/`
- **Frontend changes** → work inside `static/`
- **Dependency changes** → update `Cargo.toml` and commit the updated `Cargo.lock`

---

## Branch Naming Conventions

Always create a new branch for your changes. **Never commit directly to `main`.**

Use this format:

```
<type>/<short-description>
```

| Type | When to use | Example |
|------|-------------|---------|
| `feat` | Adding a new feature | `feat/playlist-support` |
| `fix` | Fixing a bug | `fix/audio-stream-dropout` |
| `docs` | Documentation only | `docs/add-contributing-guide` |
| `chore` | Maintenance (deps, config) | `chore/update-actix-web` |
| `refactor` | Code restructuring, no behavior change | `refactor/handler-separation` |
| `test` | Adding or updating tests | `test/streaming-edge-cases` |
| `style` | Formatting or UI tweaks | `style/fix-player-layout` |

**Create your branch:**
```bash
git checkout -b feat/your-feature-name
```

---

## Making Changes

- Keep each PR **focused** — one feature or fix per PR.
- For backend changes, make sure `cargo build` and `cargo test` pass before pushing.
- For frontend changes (HTML/CSS/JS), test in at least one modern browser.
- If you're adding new Cargo dependencies, briefly justify the addition in your PR description.
- For larger features or architectural changes, **open an issue first** to discuss before coding.

### Commit Message Style

Follow the [Conventional Commits](https://www.conventionalcommits.org/) standard:

```
<type>(<optional scope>): <short description>
```

**Examples for RustTune:**
```
feat(streaming): add range request support for audio seeking
fix(static): correct MIME type for .ogg files
docs: add setup steps to CONTRIBUTING.md
chore(deps): upgrade actix-web to 4.5.1
refactor(handlers): split music and auth routes into separate modules
style(player): fix mobile layout overflow on small screens
```

**Rules:**
- Use **imperative mood** — "add", not "added" or "adds"
- Keep the subject line under **72 characters**
- Reference related issues at the bottom: `Closes #12` or `Fixes #7`

---

## Submitting a Pull Request

1. **Push your branch** to your fork:
   ```bash
   git push origin feat/your-feature-name
   ```

2. Go to [KDeekshita/rust-tune](https://github.com/KDeekshita/rust-tune) on GitHub and click **"Compare & pull request"**.

3. Fill in the PR description with:
   - A clear **title** (e.g. `feat: add seek support for audio streaming`)
   - **What changed** and **why**
   - Screenshots or screen recordings for any UI changes
   - The issue it resolves: `Closes #<issue-number>`

4. **PR checklist before submitting:**
   - [ ] My branch is up to date with `upstream/main`
   - [ ] `cargo build` passes with no errors
   - [ ] `cargo test` passes
   - [ ] Frontend changes tested in browser
   - [ ] Relevant docs updated (README, comments, etc.)
   - [ ] Commits follow the Conventional Commits style

5. A maintainer will review your PR. Requested changes are normal — address the feedback, push your updates, and the PR will be merged once approved. 🎉

> **Heads up:** PRs that introduce breaking API changes, new dependencies, or significant architecture shifts should be discussed in a GitHub Issue before implementation.

---

## Reporting Issues

Found a bug or have a feature idea? [Open an issue](https://github.com/KDeekshita/rust-tune/issues)!

**Before opening an issue:**
- Search [existing issues](https://github.com/KDeekshita/rust-tune/issues) to avoid duplicates.
- Check if it's already fixed in the latest commit on `main`.

**For bug reports, include:**
- Clear, descriptive title
- Steps to reproduce the problem
- Expected vs. actual behavior
- Your OS, Rust version (`rustc --version`), and browser (if frontend)
- Relevant error logs from `cargo run` or browser DevTools

**For feature requests, include:**
- The problem you're solving
- Your proposed solution
- Any alternatives you considered

---

## Need Help?

- Browse [open issues](https://github.com/KDeekshita/rust-tune/issues) for context on ongoing work.
- Open a [Discussion](https://github.com/KDeekshita/rust-tune/discussions) for questions that aren't bugs or features.
- Leave a comment directly on the relevant issue or PR.

New to open source or Rust? Look for issues tagged **`good first issue`** — they're scoped to be approachable for beginners. We're happy to guide you through your first PR! 🚀

---

*This guide is open to improvement too. If something is unclear, outdated, or missing — feel free to open a PR or issue for it.*
