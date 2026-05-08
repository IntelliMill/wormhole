# Wormhole

[English](../README.md) | **中文** | [日本語](README.ja.md) | [한국어](README.ko.md)

基于终端的 SSH 连接管理器，内置加密密码保险库，使用 [Ratatui](https://github.com/ratatui-org/ratatui) 构建。

Wormhole 让你通过精美的 TUI 界面管理和连接 SSH 服务器——对主机进行分组、安全存储凭证，一键即可连接。

## 功能特性

- 加密密码保险库（AES-256-GCM + Argon2id）
- 支持可折叠文件夹的主机分组
- 使用 Shift+J/K 重新排列主机和分组
- 跨主机名、地址和分组的全文搜索
- 4 种内置配色主题，支持实时预览
- 可选主密码——也可以跳过，使用明文存储
- 全面使用 Nerd Font 图标

## 系统要求

- 支持 [Nerd Font](https://www.nerdfonts.com/) 的终端（如 JetBrains Mono Nerd Font）
- [sshpass](https://sourceforge.net/projects/sshpass/)（用于基于密码的 SSH 连接）
- Rust 1.85+（edition 2024）

## 安装

```bash
git clone https://github.com/wormhole-ssh/wormhole.git
cd wormhole
cargo build --release
cp target/release/wormhole /usr/local/bin/
```

## 使用方法

在终端中运行 `wormhole`。

### 首次运行

系统将提示你设置主密码。该密码用于加密本地保险库（`~/.wormhole/vault.enc`）中的 SSH 凭证。你可以跳过此步骤，改为以明文形式存储密码（`~/.wormhole/passwords.json`）。

### 快捷键

| 按键 | 操作 |
|-----|------|
| `j` / `↓` | 向下移动 |
| `k` / `↑` | 向上移动 |
| `h` / `l` | 切换面板焦点 |
| `Enter` | 连接到主机 |
| `e` | 编辑选中项 |
| `d` | 删除选中项 |
| `n` | 新建主机 |
| `g` | 新建分组 |
| `/` | 搜索 |
| `Tab` | 切换详情标签页 |
| `t` | 切换主题 |
| `Shift+J/K` | 重新排序 |
| `?` | 帮助 |
| `q` | 退出 |
| `Esc` | 取消 / 返回 |

### 数据存储

所有数据存储在 `~/.wormhole/` 目录下：

| 文件 | 说明 |
|------|------|
| `config.toml` | 主机、分组和设置 |
| `vault.enc` | 加密密码保险库（设置主密码时生成） |
| `passwords.json` | 明文密码（跳过主密码时生成） |

## 安全性

设置主密码后，Wormhole 使用以下安全措施：
- **Argon2id** 用于密钥派生（m=65536, t=3, p=4）
- **AES-256-GCM** 用于认证加密
- 每次安装使用随机盐值加密保险库
- 每个密码使用随机盐值进行主密码哈希

> **注意：** 基于密码的 SSH 连接使用 `sshpass`，它会将密码作为命令行参数传递。系统上的其他用户可以通过 `ps aux` 看到该密码。为了更好的安全性，建议使用密钥认证。

## 主题

Wormhole 内置 4 种主题：
- **Catppuccin Mocha**（默认）
- **Tokyo Night**
- **Dracula**
- **Gruvbox Dark**

按 `t` 打开主题选择器。使用方向键预览，按 Enter 保存。

## 许可证

[MIT](LICENSE)
