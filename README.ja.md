# ExecLocus

> AIコーディングエージェントの実行コンテキストで、何が選ばれるかを根拠付きで表示します。

ExecLocusは、現在のWindows／WSL実行コンテキスト、コマンド解決、ファイルシステム境界と根拠を表示する読み取り専用CLIです。invocationまたはprocessの証拠が得られる場合に限り、エージェント自身のruntimeも識別します。

次の疑問に、観測根拠付きで答えます。

> このコンテキストではWindows版とWSL版のどちらが選ばれ、どんな境界をまたぐのか？

> **現在の状態:** pre-alphaプロトタイプです。実行環境、Codex／Claude Codeの根拠付き判定、パス、実行ファイル由来、terminal/JSON出力、初期ルールの基盤まで実装済みですが、公開リリースはまだありません。

## 最初の画面

次はv0.1の目標を示す説明用サンプルであり、現行プロトタイプの実測画面ではありません。

```text
ExecLocus
See what your agent context resolves—and why.

CURRENT EXECUTION
  Profile       balanced                       selected
  Runtime       WSL2 / Ubuntu 24.04            observed
  User          dev                            OS account
  Shell         bash                           process ancestry
  Terminal      Windows Terminal               environment hint
  Project       /mnt/c/Users/dev/project       observed · Windows-mounted

AGENT
  Product       Claude Code                    inferred · high confidence
  Runtime       Wsl                            observed · certain confidence

TOOLCHAIN
  Git           /usr/bin/git                   Linux
  Node          /mnt/c/Program Files/node.exe  Windows

2 findings
  ENV002  WSL execution resolves Windows Node                   warning
  FS001   WSL project uses a Windows-mounted path                info
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
execlocus report --format markdown
execlocus report --format json --redact
```

ゼロ引数の`execlocus`が主な利用方法です。最初の価値を得るための設定は不要にします。

## ソースからプロトタイプを実行

```console
cargo run --
cargo run -- check
cargo run -- report --format json
cargo run -- report --format markdown
cargo run -- report --format json --redact
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

意図が異なる場合は明示できます。

```console
execlocus --profile share-first check
execlocus --profile linux-first check
```

## 事実と推測を分けます

- `observed`: OSやファイルから直接観測した事実
- `inferred`: 複数の証拠から導出し、confidenceを付けた推測
- `unavailable`: OSから取得できなかった情報
- `failed`: probeが完了しなかった情報

表示中のターミナルだけでエージェントの実行場所を断定しません。証拠がなければ`Unknown`と表示します。

現在のuserはローカルOSのprocess snapshotから取得し、ローカルOSのaccount catalogで名前を解決します。起動shellは、上限付きの親process chainに対応shellが存在するときだけ`process ancestry`として表示し、取得できない場合はallowlist済みの`SHELL`または`ComSpec`を`environment hint`へ格下げします。process snapshotが要求するのはprocess名、親ID、user IDだけで、command line、process環境変数、作業directory、root、実行file pathは要求しません。CodexのLinux／WSL sandboxがPID namespaceで親processを隠す場合に限り、Codexがtool processへ注入する`CODEX_THREAD_ID`のUUID形状をmedium-confidence fallbackとして確認します。値自体は保存・表示せず、親processの証拠を常に優先します。WSL判定はkernel releaseの証拠を優先し、kernelを読めずWSL環境変数だけがある場合は推定として表示します。distributionはWSL登録名を優先し、Linuxでは`/etc/os-release`をfallbackにします。これらの通常観測ではcommand shellの起動やnetwork接続を行いません。

## 安全性とプライバシー

- 読み取り専用で、PATHや設定を変更しない
- 通常実行時にネットワークへ接続しない
- tokenや秘密の環境変数値を収集しない
- Markdown共有レポートは常に自動匿名化し、`report --format json --redact`でもserialization前にusername、home、machine名、絶対pathを匿名化する

## 関連資料

- [追加支出0円の開発ポリシー](COST_POLICY.ja.md)
- [Zero-incremental-cost development policy](COST_POLICY.md)
- [製品仕様](PRODUCT_SPEC.md)
- [診断ルール仕様](RULES.md)
- [MVPスコープ](docs/MVP_SCOPE.md)
- [v0.1提供ロードマップ](docs/V0_1_ROADMAP.md)
- [初期ユースケース仕様](docs/USE_CASES.md)
- [現在の対応状況](docs/SUPPORT_MATRIX.md)
- [匿名化済みruntime identity実機検証](docs/validation/RUNTIME_IDENTITY_2026-07-29.md)
- [共有用匿名化の検証記録](docs/validation/SHAREABLE_REDACTION_2026-07-29.md)
- [Windows Claude Code／WSL Codex実測](docs/validation/WINDOWS_CLAUDE_WSL_CODEX_2026-07-29.md)
- [1ページ製品紹介](docs/ONE_PAGER.ja.md)
- [デモ／紹介MV制作計画](docs/DEMO_PLAN.md)
- [絵コンテ・撮影scenario](docs/demo/README.md)
- [OSS成功パターンの採用設計](docs/ADOPTION_BLUEPRINT.md)
- [代替手段・現在の回避策との比較](docs/research/ALTERNATIVES.md)
- [OSSベンチマーク・公開戦略](docs/research/README.md)
- [X需要調査の投稿戦略](docs/research/X_POST_STRATEGY.md)
- [変更履歴](CHANGELOG.md)
- [コントリビューションガイド](CONTRIBUTING.md)
- [セキュリティポリシー](SECURITY.md)

## 実装状況

- [x] Rust CLIのscaffold
- [x] Windows/WSLの観測モデル
- [x] 実行ファイルとファイルシステムの分類
- [x] terminal/JSON renderer
- [x] 初期診断ルール（`ENV002`、`PATH001`、`GIT001`）
- [x] Codex/Claude adapter
- [x] 自動匿名化Markdownレポートと`--redact` JSON
- [x] `FS001`／`FS002`と3 profileの実動作
- [ ] `ENV001`／`ENV003`／`ENV004`
- [ ] `explain <RULE_ID>`とshell固有candidate表示
- [ ] 外部prototype検証、実測demo、v0.1.0 release artifact

## 実環境での仮説検証にご協力ください

v0.1.0までに、10人以上の協力者から10環境以上のカテゴリ情報を集め、3件以上の「確認済みで役に立った事例」を得ることを目標にしています。単に差異が見つかっただけでは数えず、手作業より明確な結論または判断につながったことを確認します。

プロトタイプを試した場合は、[フィールドレポート](https://github.com/Tanasu0417/execlocus/issues/new?template=field_report.yml)を利用できます。フォームはまだコマンド出力を要求しません。メンテナー確認済みの検証で出力が必要な場合だけ、自動匿名化Markdownまたは`--redact` JSONを使用してください。raw terminal／raw JSON、credential、非公開プロジェクト情報は公開Issueへ貼らないでください。

## ライセンス

ExecLocusは[MIT License](LICENSE)で公開します。
