# ExecLocus

> AIコーディングエージェントが、実際にどこで実行されているかを表示します。

ExecLocusは、Windows、WSL、シェル、ファイルシステム、ツールチェーンにまたがるAIコーディングエージェントの実行トポロジーを表示する、読み取り専用CLIです。

次の疑問に、観測根拠付きで答えます。

> エージェントはWindows版、WSL版、それとも両方が混在した状態で動いているのか？

> **現在の状態:** pre-alphaプロトタイプです。実行環境、パス、実行ファイル由来、terminal/JSON出力、初期ルールの基盤まで実装済みですが、公開リリースはまだありません。

## 最初の画面

```text
ExecLocus
See where your agent actually executes.

CURRENT EXECUTION
  Runtime       WSL2 / Ubuntu 24.04            observed
  Shell         /usr/bin/bash                  environment hint
  Project       /mnt/c/Users/dev/project       observed · Windows-mounted

AGENT
  Product       Claude Code                    inferred · high confidence
  Executable    /usr/local/bin/claude          observed · Linux

TOOLCHAIN
  Git           /usr/bin/git                   Linux
  Node          /mnt/c/Program Files/node.exe  Windows

1 finding
  ENV002  WSL execution resolves Windows Node                   warning
```

## なぜ必要か

画面がWSLのターミナルに見えても、エージェント、Git、Node、設定、プロジェクトがすべてLinux側で動いている証明にはなりません。

例えば、次の状態が起こり得ます。

- WSLのbashからWindows版Node.jsを参照している
- 同じエージェントのWindows版とWSL版が両方存在する
- Gitとプロジェクトが異なるOS層にある
- エージェントの書き込み可能な状態ファイルをWindowsとWSLで共有している
- WindowsアプリやCoworkから参照するため、意図的に`/mnt/c`へ配置している

ExecLocusは、混在状態をすべてエラーにせず、目的と証拠に基づいて説明します。

## v0.1で予定しているコマンド

```console
execlocus
execlocus check
execlocus explain ENV002
execlocus report --format json
execlocus report --format markdown --redact
```

ゼロ引数の`execlocus`が主な利用方法です。最初の価値を得るための設定は不要にします。

## ソースからプロトタイプを実行

```console
cargo run --
cargo run -- check
cargo run -- report --format json
```

開発時の確認コマンド:

```console
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Windowsの`x86_64-pc-windows-msvc`ツールチェーンでは、Rust本体に加えてMicrosoft C++ linkerとWindows SDKが必要です。WSL/Linuxでは`cc`などのC linkerが必要です。

## `/mnt/c`を一律否定しません

| プロファイル | 優先事項 | `/mnt/c`の評価 |
|---|---|---|
| `share-first` | Windowsアプリ、Explorer、Coworkとの共有 | 許容し、注意点だけを説明 |
| `linux-first` | Linux互換性とI/O性能 | WSL-nativeを推奨 |
| `balanced` | ソース共有と重い生成物の分離 | ソースを許容し、cache/buildの分離を提案 |

既定値は`balanced`です。

## 事実と推測を分けます

- `observed`: OSやファイルから直接観測した事実
- `inferred`: 複数の証拠から導出し、confidenceを付けた推測
- `unavailable`: OSから取得できなかった情報
- `failed`: probeが完了しなかった情報

表示中のターミナルだけでエージェントの実行場所を断定しません。証拠がなければ`Unknown`と表示します。

## 安全性とプライバシー

- 読み取り専用で、PATHや設定を変更しない
- 通常実行時にネットワークへ接続しない
- tokenや秘密の環境変数値を収集しない
- 共有用レポートではusername、home、machine名、個人の絶対パスを秘匿化する

## 関連資料

- [製品仕様](PRODUCT_SPEC.md)
- [診断ルール仕様](RULES.md)
- [MVPスコープ](docs/MVP_SCOPE.md)
- [OSSベンチマーク・公開戦略](docs/research/README.md)
- [コントリビューションガイド](CONTRIBUTING.md)
- [セキュリティポリシー](SECURITY.md)

## 実装状況

- [x] Rust CLIのscaffold
- [x] Windows/WSLの観測モデル
- [x] 実行ファイルとファイルシステムの分類
- [x] terminal/JSON renderer
- [x] 初期診断ルール（`ENV002`、`PATH001`、`GIT001`）
- [ ] Codex/Claude adapter
- [ ] 秘匿化Markdownレポート

## ライセンス

ExecLocusは[MIT License](LICENSE)で公開します。
