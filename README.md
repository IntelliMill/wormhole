<div align="center">
  <img src="assets/logo.png" alt="Wormhole Logo" width="256" />
</div>

# Wormhole

**English** | [中文](docs/README.zh.md) | [日本語](docs/README.ja.md) | [한국어](docs/README.ko.md)

A fast, secure, and beautiful terminal SSH connection manager written in Rust.

Wormhole lets you manage and connect to your SSH servers from a stunning TUI interface — organize hosts into groups, encrypt credentials with AES-256-GCM, and connect with a single keypress.

## Screenshots

<div align="center">
  <img src="assets/login.png" alt="Login" width="49%" />
  <img src="assets/host_list.png" alt="Host List" width="49%" />
</div>
<div align="center">
  <img src="assets/new_group.png" alt="New Group" width="49%" />
  <img src="assets/new_host.png" alt="New Host" width="49%" />
</div>

## Features

- Encrypted password vault (AES-256-GCM + Argon2id)
- Group hosts with collapsible folders
- Reorder hosts and groups with Shift+J/K
- Full-text search across host names, addresses, and groups
- 4 built-in color themes with live preview
- Optional master password — or skip it for plain-text storage
- Nerd Font icons throughout

## Requirements

- A [Nerd Font](https://www.nerdfonts.com/) installed and set as your terminal font (e.g. JetBrains Mono Nerd Font)
- [sshpass](https://sourceforge.net/projects/sshpass/) (for password-based SSH connections)
- Rust 1.85+ (edition 2024)

### Installing a Nerd Font

Wormhole uses Nerd Font icons throughout the interface. If icons appear as boxes or question marks, install a Nerd Font:

```bash
# macOS
brew install --cask font-jetbrains-mono-nerd-font

# Linux
mkdir -p ~/.local/share/fonts && cd ~/.local/share/fonts
curl -fLO https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.zip
unzip JetBrainsMono.zip && rm JetBrainsMono.zip
fc-cache -fv
```

Then set "JetBrainsMono Nerd Font" as your terminal font.

## Installation

```bash
git clone https://github.com/wormhole-ssh/wormhole.git
cd wormhole
cargo build --release
cp target/release/wormhole /usr/local/bin/
```

## Usage

Run `wormhole` in your terminal.

### First Run

You will be prompted to set a master password. This password encrypts your SSH credentials in a local vault (`~/.wormhole/vault.enc`). You can skip this to store passwords in plain text instead (`~/.wormhole/passwords.json`).

### Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `l` | Switch panel focus |
| `Enter` | Connect to host |
| `e` | Edit selected |
| `d` | Delete selected |
| `n` | New host |
| `g` | New group |
| `/` | Search |
| `Tab` | Switch detail tab |
| `t` | Switch theme |
| `Shift+J/K` | Reorder items |
| `?` | Help |
| `q` | Quit |
| `Esc` | Cancel / back |

### Data Storage

All data is stored in `~/.wormhole/`:

| File | Description |
|------|-------------|
| `config.toml` | Hosts, groups, and settings |
| `vault.enc` | Encrypted password vault (if master password set) |
| `passwords.json` | Plain-text passwords (if master password skipped) |

## Security

When a master password is set, Wormhole uses:
- **Argon2id** for key derivation (m=65536, t=3, p=4)
- **AES-256-GCM** for authenticated encryption
- Per-installation random salt for vault encryption
- Per-password random salt for master password hashing

> **Note:** Password-based SSH connections use `sshpass`, which passes the password as a command-line argument. This is visible to other users on the system via `ps aux`. For better security, use key-based authentication.

## Themes

Wormhole ships with 4 themes:
- **Catppuccin Mocha** (default)
- **Tokyo Night**
- **Dracula**
- **Gruvbox Dark**

Press `t` to open the theme selector. Use arrow keys to preview, Enter to save.

## License

[MIT](LICENSE)
