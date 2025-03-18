# 📖 Phrack Read CLI

![Phrack Read](https://img.shields.io/badge/Rust-CLI-blue?style=for-the-badge&logo=rust) ![Crate Version](https://img.shields.io/crates/v/phrack-read?style=for-the-badge)


A powerful and elegant command-line tool to read **Phrack Magazine** articles directly from your terminal with smooth navigation and a beautiful display format.

## 🚀 Features

✅ Fetch and display **Phrack** issues and articles using web scraping.  
✅ Styled terminal output with borders and formatting.  
✅ Smooth scrolling with **arrow keys, PageUp, and PageDown**.  
✅ **Keyboard Shortcuts** for easy navigation.  
✅ Cross-platform support (Linux, macOS, Windows).  

---

## 📦 Installation

To install **Phrack Read CLI**, you need **Rust** installed. Run:

```sh
cargo install phrack-read
```

Or clone the repository and build manually:

```sh
git clone https://github.com/PATEL96/phrack-read.git
cd phrack-read
cargo build --release
./target/release/phrack-read --help
```

---

## 🎯 Usage

Fetch and read a specific **Phrack issue** and **article**:

```sh
phrack-read <issue> <article>
```

Example:

```sh
phrack-read 69 1
```

This command fetches **Issue 69, Article 1** and displays it in the terminal.

---

## 🎨 Terminal Interface

The content is displayed with a structured **ASCII border** and allows easy scrolling.

```plaintext
╔════════════════════════════════════════════════════════════╗
║                 Phrack Issue 69 Article 1                  ║
╚════════════════════════════════════════════════════════════╝
```

### 🕹 Navigation

| Key | Action |
|-----|--------|
| ↑   | Scroll up |
| ↓   | Scroll down |
| PgUp | Page up |
| PgDn | Page down |
| q / Esc | Quit |

---

## 📜 Dependencies

This project uses the following **Rust crates**:

- **[reqwest](https://crates.io/crates/reqwest)** → Fetch webpage content
- **[scraper](https://crates.io/crates/scraper)** → Parse and extract text from HTML
- **[clap](https://crates.io/crates/clap)** → Command-line argument parsing
- **[colored](https://crates.io/crates/colored)** → Colorful terminal output
- **[crossterm](https://crates.io/crates/crossterm)** → Terminal manipulation (scrolling, clearing, cursor control)

---

## 🛠 Development

To contribute read [Contribution Guide](./CONTRIBUTION.md)

---

## 📜 License

This project is licensed under the **MIT License**. Feel free to modify and use it as you wish!

📖 Happy Hacking! 🤘

