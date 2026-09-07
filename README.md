# Atom One Pro for Obsidian (v2.0.0)

A sleek, modern, and high-performance theme for [Obsidian](https://obsidian.md), rewritten from the ground up with a native **Rust-powered compilation pipeline**.

---

## 🚀 What's New in v2.0.0

Version `2.0.0` marks a complete architectural rewrite of the theme engine. The codebase has been fully migrated from a monolithic CSS file into a clean, modular SCSS structure compiled natively using Rust.

* **Native Rust Build Engine:** Blazing-fast compilation with zero reliance on Node.js or heavy `npm` dependencies.
* **Modular SCSS Architecture:** Organized codebase split into clean, maintainable SCSS components.
* **Lightweight & Cross-Platform:** Minimalist build pipeline optimized for Linux, macOS, and Windows.
* **Core Refactoring:** Completely cleaned legacy CSS debt, improved variable scoping, and enhanced DOM rendering performance.

---

## 🛠️ Building from Source

This project utilizes a native Rust build system. To compile the theme locally:

### Prerequisites
* [Rust & Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### Build Steps
1. Clone the repository:
   ```bash
   git clone [https://github.com/DevAbdallahAbdelrahim/atom-one-pro-obsidian.git](https://github.com/DevAbdallahAbdelrahim/atom-one-pro-obsidian.git)
   cd atom-one-pro-obsidian
