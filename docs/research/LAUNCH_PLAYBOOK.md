# ExecLocus launch playbook

更新日: 2026-07-28

この文書は、[30件の個人所有OSS分析](OSS_BENCHMARK_30.md)から得た発信・配布パターンを、ExecLocusの初回リリースへ変換する実行計画である。

リポジトリ構成、開発単位、受入ゲートへの反映は[OSS adoption blueprint](../ADOPTION_BLUEPRINT.md)を正本とする。

## 発信の原則

### 製品ではなく問題から始める

悪い例:

> A cross-platform environment diagnostic CLI written in Rust.

推奨:

> In your AI coding agent's current context, will `git` resolve to Windows Git or Linux Git? ExecLocus shows which candidate would win and the evidence behind the answer.

日本語:

> AIコーディングエージェントの現在のコンテキストで、`git`はWindows版とLinux版のどちらに解決されるか。ExecLocusは、どの候補が選ばれるかと判定根拠を表示します。

### AIを主語にしすぎない

CodexやClaude Codeは入口として有効だが、製品の本体はローカル実行環境の再現可能な診断である。特定AI製品のAPIや仕様変更に依存しない価値を保つ。

### `/mnt/c`を否定しない

Windows側のCowork系ツールとWSL側のCLIから同じプロジェクトを扱えることは利点でもある。ExecLocusは場所を善悪で分類せず、性能、権限、実行ファイル混在、意図との不一致を個別に説明する。

## リリース段階

### Stage 0: private proof

対象: 開発者本人と協力者

- 10人以上・10件以上のWindows／WSL実環境を診断する
- 検証済み、誤検知、判定不能を分けて記録する
- 従来の手作業とExecLocusのtime-to-conclusionを比較する
- 再利用意向と後日の実利用を別々に記録する
- terminal出力とJSONを固定する
- 共有出力のredactionをテストする
- 10秒デモに使う実例を1件選ぶ

終了条件: 3件以上で、確認可能な結論または判断につながり、従来の手作業より役に立ったと確認する。差異が存在しただけでは完了に数えない。

### Stage 1: `v0.1.0-alpha.1`

対象: GitHubから試す初期利用者

- GitHub prereleaseとして公開
- Windows x86_64とLinux x86_64のバイナリ
- 各ファイルのSHA-256 checksum
- 第三者ライセンスnoticeとSBOM
- 既知の制約とNon-goals
- コピー可能な3シナリオ
- 変更履歴

GitHub Releasesは、リリースノートとバイナリを同じ場所で配布できる。最初は必ずDraft releaseを作り、全assetとchecksumが揃ってから公開する。

参考: [GitHub: Managing releases](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)

### Stage 2: `v0.1.0`

対象: 一般のWindows／WSL利用者

- alpha利用者の重大問題を解消
- CI、CodeQL、checksum、第三者ライセンスnotice、SBOM生成を自動化
- `cargo package --list`と`cargo publish --dry-run`を確認
- crates.ioへ公開する場合は、公開物に秘密・不要なassetがないか再監査
- WinGetまたはScoopの少なくとも1経路を追加

crates.ioの公開versionは上書きも削除もできないため、名前・metadata・同梱ファイルを先に確定する。API tokenはリポジトリやIssueへ貼らない。

参考: [Cargo Book: Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)

### Stage 3: repeatable releases

- Conventional Commitsまたは同等の変更分類
- tagからRelease assetを生成
- checksumとSBOMを生成
- changelogを人が最終編集
- Windows／Linuxのsmoke test
- 将来macOSを追加する場合は、対応表と実機検証を同時に追加

## README公開セット

README上部は次の順序にする。

1. 一文の価値提案
2. 10秒以内の端末GIFまたは静止画
3. `Install`と`Run`
4. 3つの実例
5. 実際の出力
6. Privacy and safety
7. Supported environments
8. How it works
9. Non-goals
10. Contributing、Security、License

長いschema、全rule、設計判断はREADMEへ詰めず、既存の`PRODUCT_SPEC.md`、`RULES.md`、`docs/`へ分離する。

## 必須の公開素材

| 素材 | 目的 | 完了条件 |
|---|---|---|
| 端末デモ | 最初の10秒で価値を伝える | Windows GitとWSL Gitの差が1画面で分かる |
| 共有用診断例 | 実利用を想像させる | 個人情報がredactされている |
| Before／After | 手作業との差を示す | 手動コマンド列とExecLocus 1回を比較 |
| 対応表 | 期待値を管理する | Windows Native、WSL、Linuxを別行で示す |
| Non-goals | scope creepを防ぐ | 自動修復しないことを明記 |
| Release notes | 継続利用を支える | Added／Changed／Fixed／Known limitationsを含む |
| Checksums | 配布物の検証 | 全Release assetにSHA-256がある |

## 公開チャネルの順序

### 1. GitHub Release

最初のsource of truth。tag、release notes、バイナリ、checksumを揃える。READMEのinstall URLは`latest`へ無条件に向けず、バージョン固定または検証可能な導線を用意する。

### 2. crates.io

Rust利用者が`cargo install execlocus`で試せる。ただしversionは恒久的に残るため、最初に`cargo publish --dry-run`と同梱ファイル一覧を確認する。

### 3. WinGetまたはScoop

ExecLocusはWindowsが主役なので、一般的なRust CLIよりWindows導入経路の優先度が高い。WinGetはmanifest提出後に自動検証と安全性確認が行われる。

参考: [Microsoft: Create a WinGet package manifest](https://learn.microsoft.com/en-us/windows/package-manager/package/manifest)

### 4. 問題別の記事

リリース告知だけでなく、次の3本を個別記事にする。

1. 「Codex／Claude CodeはWindows版とWSL版のどちらを実行しているか」
2. 「WSLで`which git`だけでは足りない理由」
3. 「`/mnt/c`共有は悪ではない。意図しない混在だけを診断する」

日本語版はZenn、Qiita等のうち普段利用する場所を1つ選び、同じ本文を無差別に重複投稿しない。英語版はGitHub Discussionまたは個人ブログをsource of truthにし、各コミュニティには要約とリンクを投稿する。

### 5. Show HN

実際にダウンロードして試せる段階だけで投稿する。タイトルは`Show HN: ExecLocus – explain whether your coding agent is running Windows or WSL`のようにする。登録要求やランディングページだけの状態では投稿しない。作者本人が背景と理由を説明し、投稿後の質問へ回答できる日に行う。

参考: [Show HN Guidelines](https://news.ycombinator.com/showhn.html)

## 初回投稿テンプレート

### Short English

```text
I use AI coding agents across Windows and WSL, and I kept losing time to a simple question:
which runtime and executable would this context select?

ExecLocus is a local, read-only Rust CLI that shows the runtime, distro, user, shell,
filesystem boundary, executable resolution, and the evidence behind each conclusion.

It does not upload diagnostics or rewrite PATH. Shareable output will be available only after redaction-before-rendering is implemented and tested.

Try: <release URL>
Source: https://github.com/Tanasu0417/execlocus
```

### Short Japanese

```text
WindowsとWSLをまたいでAIコーディングエージェントを使っていると、
「このコンテキストでは、どちらの実行ファイルが選ばれるのか」が分からなくなることがあります。

ExecLocusは、runtime、distro、user、shell、ファイルシステム境界、
実行ファイルの解決結果と根拠をローカルで表示する読み取り専用Rust CLIです。

診断情報はアップロードしません。共有用出力は、serialization前の秘匿化を実装・検証してから提供します。
```

## 計測

スターだけを目標にしない。最初の30日では次を追う。

| 指標 | 初期目標 | 理由 |
|---|---:|---|
| unique collaborator／環境 | 30／30 | 同一人物の複数実行を需要として過大計上しない |
| 確認済みで役立った結論 | 10 | 差異でなく判断価値を検証する |
| 誤検知／判定不能 | 全件分類 | 精度と限界を確認する |
| 手作業比のtime-to-conclusion | 全件記録 | 時間価値を確認する |
| 再現可能なIssue | 5 | 診断情報の品質を見る |
| 再利用意向／後日の実利用 | 分けて記録 | 意向とretentionを混同しない |
| 外部contributor | 2 | ルール拡張可能性を見る |
| privacyに関する事故 | 0 | 最優先の安全指標 |

ダウンロード数はGitHub API、Issueはlabel、再利用は匿名の任意フィードバックで確認する。通常実行時のtelemetryは導入しない。

## やらないこと

- スター購入、相互スター、投票依頼
- 同じ宣伝文の大量投稿
- 未検証の性能または安全性主張
- AI生成文を人の確認なしで公開
- token、メール、ローカル絶対パスをデモへ含める
- crates.io tokenや署名鍵をGitHub Actionsのログへ出す
- v0.1.0で全パッケージマネージャーへ同時対応する

## 実装順と需要検証

実装順の正本は[OSS adoption blueprint](../ADOPTION_BLUEPRINT.md)のGate A〜Dとし、この文書には重複して定義しない。紹介資料・デモ・Xでの需要検証は[デモ制作計画](../DEMO_PLAN.md)と[需要検証計画](DEMAND_VALIDATION.md)に従う。
