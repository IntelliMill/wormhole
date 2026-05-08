# Wormhole

**English** | [中文](docs/README.zh.md) | [日本語](docs/README.ja.md) | [한국어](docs/README.ko.md)

A terminal-based SSH connection manager with an encrypted password vault, built with [Ratatui](https://github.com/ratatui-org/ratatui).

Wormhole lets you manage and connect to your SSH servers from a beautiful TUI interface — group hosts, store credentials securely, and connect with a single keypress.

## Features

- Encrypted password vault (AES-256-GCM + Argon2id)
- Group hosts with collapsible folders
- Reorder hosts and groups with Shift+J/K
- Full-text search across host names, addresses, and groups
- 4 built-in color themes with live preview
- Optional master password — or skip it for plain-text storage
- Nerd Font icons throughout

## Requirements

- A terminal with [Nerd Font](https://www.nerdfonts.com/) support (e.g. JetBrains Mono Nerd Font)
- [sshpass](https://sourceforge.net/projects/sshpass/) (for password-based SSH connections)
- Rust 1.85+ (edition 2024)

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
