# Wormhole

[English](../README.md) | [中文](README.zh.md) | **日本語** | [한국어](README.ko.md)

[Ratatui](https://github.com/ratatui-org/ratatui) で構築された、暗号化パスワード保管庫付きターミナルベースの SSH 接続マネージャー。

Wormhole を使えば、美しい TUI インターフェースから SSH サーバーを管理・接続できます。ホストのグループ化、認証情報の安全な保存、ワンキーでの接続が可能です。

## 機能

- 暗号化パスワード保管庫（AES-256-GCM + Argon2id）
- 折りたたみ可能なフォルダーによるホストのグループ化
- Shift+J/K でホストとグループの並べ替え
- ホスト名、アドレス、グループ全体の全文検索
- ライブプレビュー付き 4 種類の内蔵カラーテーマ
- オプションのマスターパスワード — スキップしてプレーンテキストでの保存も可能
- Nerd Font アイコンの全面採用

## 動作要件

- [Nerd Font](https://www.nerdfonts.com/) に対応したターミナル（例：JetBrains Mono Nerd Font）
- [sshpass](https://sourceforge.net/projects/sshpass/)（パスワードベースの SSH 接続用）
- Rust 1.85+（edition 2024）

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
