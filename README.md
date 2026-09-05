# 📎clipls
### Copy files on the Clipboard. The bridge of TUI and GUI.

<p align="center">
  <img src="assets/clipls_icon.png" width="160" alt="clipls icon">
</p>

[![Feature Requests](https://img.shields.io/github/issues/h2depot/clipls/enhancement.svg?style=for-the-badge&label=FEATURE%20REQUESTS&color=7c6fe8)](https://github.com/h2depot/clipls/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Bugs](https://img.shields.io/github/issues/h2depot/clipls/bug.svg?style=for-the-badge&label=BUGS&color=ef6f8e)](https://github.com/h2depot/clipls/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![License](https://img.shields.io/badge/LICENSE-AGPLv3-2fbaa3?style=for-the-badge)](https://github.com/h2depot/clipls/blob/main/LICENSE)

## 🎉 News

## ⚙️How to Use clipls

### Build with Windows
Coming soon...

### Install on Linux / macOS

Run the following command on Linux or macOS (requires `curl` and `tar`):

```sh
curl -fsSL https://raw.githubusercontent.com/h2depot/clipls/main/install.sh -o install-clipls.sh && sh install-clipls.sh
```

The installer detects your OS and CPU architecture (x86_64 or ARM64), downloads the matching `.tar.gz` from the latest GitHub Release, and extracts `clipls` into `~/.local/bin`. A published release with the matching asset is required.

Add the installation directory to your PATH for the current terminal session:

```sh
export PATH="$HOME/.local/bin:$PATH"
clipls --version
```

To keep this PATH setting in new terminals, add the `export` line to `~/.bashrc` (Bash) or `~/.zshrc` (Zsh).

### command menu

```text
clipls [OPTIONS] [PATH]
```

If `PATH` is omitted, clipls opens the current directory. After starting, select the items you want to copy in the TUI.

| Command | Description |
| --- | --- |
| `clipls` | Browse the current directory and copy the selected files to the clipboard. |
| `clipls <PATH>` | Browse the specified directory instead of the current directory. |
| `clipls --fc <FILES...>` | Directly copy specified files to the clipboard without launching the TUI (Fast Clip). |
| `clipls -r <PATH>` / `clipls --recursive <PATH>` | Include files and directories inside subdirectories. |
| `clipls -a <PATH>` / `clipls --all <PATH>` | Include hidden files and directories. |
| `clipls -m file <PATH>` | Copy the selected files themselves to the clipboard (default mode). |
| `clipls -m text <PATH>` | Copy the UTF-8 contents of the selected files as text. |
| `clipls -m path <PATH>` | Copy the paths of the selected files as newline-separated text. |
| `clipls -h` / `clipls --help` | Show the command-line help. |
| `clipls -v` / `clipls --version` | Show the installed clipls version. |

Options can be combined, for example: `clipls -ra -m path ./src`.

## 🖥️ Supported OS

- Windows (Download from winget command. Coming soon...)
- macOS (Intel / Apple Silicon; see installation command above)
- Linux (x86_64 / ARM64; see installation command above)

## 💡 Questions & Suggestions

Feel free to ask questions or make suggestions using Pull Request.

## ⚖️ License
Copyright © 2026 HelloweenHead's Depot. All rights reserved.

clipls is licensed under the [Massachusetts Institute of Technology License](https://github.com/h2depot/clipls/blob/main/LICENSE).

You are free to redistribute, use for commercial purposes, and modify this clipls.

## 💐 Acknowledgments

clipls is made possible by excellent open-source projects and the people behind them.

- [**ratatui**](https://ratatui.rs/) - Rust library for building fast, lightweight, and rich terminal user interfaces.
- [**clap**](https://docs.rs/clap/latest/clap/) - Command Line Argument Parser for Rust.
- [**arboard**](https://docs.rs/arboard/latest/arboard/) - Cross-platform Rust library for reading from and writing to the system clipboard.
- [**crossterm**](https://docs.rs/crossterm/latest/crossterm/) - terminal manipulation library that makes it possible to write cross-platform text-based interfaces.
- [**anyhow**](https://docs.rs/anyhow/latest/anyhow/) - Flexible error handling library for Rust applications with contextual error messages.

---

<div align="center">
  <img src="https://user-images.githubusercontent.com/74038190/212284100-561aa473-3905-4a80-b561-0d28506553ee.gif" width="500" alt="Animated divider">
  <br><br>
  <a href="https://github.com/h2depot/clipls">
    <img src="https://img.shields.io/badge/⭐%20STAR%20CLIPLS-1a1a2e?style=for-the-badge&logo=github&logoColor=white" alt="Star clipls on GitHub">
  </a>
  <br><br>
  <strong>⭐ Thank you for stopping by clipls! ⭐</strong>
  <br>
  <sub>Take a breath. Open a page. Let the story happen.</sub>
</div>
