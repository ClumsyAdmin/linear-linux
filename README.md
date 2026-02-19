<p align="center">
<img alt="Linear for Linux" src="icon.png" width="100px">
<h1 align="center">Linear for Linux</h1>

<p align="center">
A Linux desktop client for <a href="https://linear.app/">linear.app</a>, built with <a href="https://v2.tauri.app/">Tauri v2</a>.
</p>

<p align="center">
<img alt="GitHub release" src="https://img.shields.io/github/v/release/zacharyftw/linear-linux?color=%23f5304c">
</p>

# Install

Requires `sudo` (for desktop entry and icons).

```bash
wget -qO- https://raw.githubusercontent.com/zacharyftw/linear-linux/refs/heads/master/installer.sh | bash
```

The installer:
- Downloads and installs the `.deb` package.
- Installs the Linear icon into the system icon cache.
- Accepts overrides: `VERSION=0.2.3 DEB_URL=<url> ./installer.sh`

# Development

Requires [Rust](https://rustup.rs/) and system dependencies:

```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libappindicator3-dev librsvg2-dev
```

```bash
npm install     # install Tauri CLI
npm run dev     # launch dev build
npm run build   # produce AppImage and .deb in src-tauri/target/release/bundle/
```

# Having an issue?

Describe your issue [here](https://github.com/zacharyftw/linear-linux/issues)

# Disclaimer

This is an unofficial Linux desktop client for Linear.
Linear is a trademark of Linear Orbit, Inc.
This project is not affiliated with or endorsed by Linear.
