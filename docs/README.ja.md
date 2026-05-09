<div align="center">
  <img src="../assets/logo.png" alt="Wormhole Logo" width="256" />
</div>

# Wormhole

[English](../README.md) | [中文](README.zh.md) | **日本語** | [한국어](README.ko.md)

Rust で書かれた、高速で安全、美しいターミナル SSH 接続マネージャー。

Wormhole を使えば、洗練された TUI インターフェースから SSH サーバーを管理・接続できます。ホストのグループ化、AES-256-GCM による認証情報の暗号化、ワンキーでの接続が可能です。

## スクリーンショット

<div align="center">
  <img src="../assets/login.png" alt="ログイン" width="49%" />
  <img src="../assets/host_list.png" alt="ホスト一覧" width="49%" />
</div>
<div align="center">
  <img src="../assets/new_group.png" alt="新規グループ" width="49%" />
  <img src="../assets/new_host.png" alt="新規ホスト" width="49%" />
</div>

## 機能

- 暗号化パスワード保管庫（AES-256-GCM + Argon2id）
- 折りたたみ可能なフォルダーによるホストのグループ化
- Shift+J/K でホストとグループの並べ替え
- ホスト名、アドレス、グループ全体の全文検索
- ライブプレビュー付き 4 種類の内蔵カラーテーマ
- オプションのマスターパスワード — スキップしてプレーンテキストでの保存も可能
- Nerd Font アイコンの全面採用

## 動作要件

- [Nerd Font](https://www.nerdfonts.com/) がインストールされ、ターミナルのフォントに設定されていること（例：JetBrains Mono Nerd Font）
- [sshpass](https://sourceforge.net/projects/sshpass/)（パスワードベースの SSH 接続用）
- Rust 1.85+（edition 2024）

### Nerd Font のインストール

Wormhole のインターフェースでは Nerd Font アイコンを多用しています。アイコンが四角や疑問符で表示される場合は、Nerd Font をインストールしてください：

```bash
# macOS
brew install --cask font-jetbrains-mono-nerd-font

# Linux
mkdir -p ~/.local/share/fonts && cd ~/.local/share/fonts
curl -fLO https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.zip
unzip JetBrainsMono.zip && rm JetBrainsMono.zip
fc-cache -fv
```

インストール後、ターミナルの設定でフォントを「JetBrainsMono Nerd Font」に変更してください。

## インストール

```bash
git clone https://github.com/wormhole-ssh/wormhole.git
cd wormhole
cargo build --release
cp target/release/wormhole /usr/local/bin/
```

## 使い方

ターミナルで `wormhole` を実行してください。

### 初回起動

マスターパスワードの設定を求められます。このパスワードは、ローカル保管庫（`~/.wormhole/vault.enc`）内の SSH 認証情報を暗号化するために使用されます。スキップすると、パスワードはプレーンテキストで保存されます（`~/.wormhole/passwords.json`）。

### キーバインド

| キー | アクション |
|-----|-----------|
| `j` / `↓` | 下に移動 |
| `k` / `↑` | 上に移動 |
| `h` / `l` | パネルフォーカスの切替 |
| `Enter` | ホストに接続 |
| `e` | 選択項目を編集 |
| `d` | 選択項目を削除 |
| `n` | 新規ホスト作成 |
| `g` | 新規グループ作成 |
| `/` | 検索 |
| `Tab` | 詳細タブの切替 |
| `t` | テーマ切替 |
| `Shift+J/K` | 項目の並べ替え |
| `?` | ヘルプ |
| `q` | 終了 |
| `Esc` | キャンセル / 戻る |

### データストレージ

すべてのデータは `~/.wormhole/` に保存されます：

| ファイル | 説明 |
|---------|------|
| `config.toml` | ホスト、グループ、設定 |
| `vault.enc` | 暗号化パスワード保管庫（マスターパスワード設定時） |
| `passwords.json` | プレーンテキストのパスワード（マスターパスワードスキップ時） |

## セキュリティ

マスターパスワードを設定すると、Wormhole は以下を使用します：
- キー導出に **Argon2id**（m=65536, t=3, p=4）
- 認証付き暗号化に **AES-256-GCM**
- 保管庫暗号化用のインストールごとのランダムソルト
- マスターパスワードハッシュ用のパスワードごとのランダムソルト

> **注意：** パスワードベースの SSH 接続は `sshpass` を使用します。`sshpass` はパスワードをコマンドライン引数として渡すため、システム上の他のユーザーが `ps aux` で確認できる場合があります。より高いセキュリティのためには、鍵ベースの認証の使用を推奨します。

## テーマ

Wormhole には 4 種類のテーマが組み込まれています：
- **Catppuccin Mocha**（デフォルト）
- **Tokyo Night**
- **Dracula**
- **Gruvbox Dark**

`t` キーでテーマセレクターを開きます。矢印キーでプレビュー、Enter で保存します。

## ライセンス

[MIT](LICENSE)
