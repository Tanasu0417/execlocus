# ExecLocus 1ページ紹介

> AIコーディングエージェントの実行コンテキストで、何が選ばれるかを根拠付きで表示します。

ExecLocusは、WindowsとWSLをまたいでAIコーディングエージェントを使う開発者向けの、ローカル・読み取り専用CLIです。現在の実行コンテキスト、選ばれる実行ファイル、プロジェクトの保存場所、判定根拠を1つの結果へまとめます。

**現在の状態:** pre-alphaのsource prototypeで、公開binaryはまだありません。「現在のコンテキストなら選ばれる実行ファイル」と「エージェントが過去に実行した実行ファイル」は別物です。後者はprocessまたはinvocationの証拠がある場合だけ表示します。

## 解決する問題

画面がWSL terminalに見えても、Git、Node、npm、agent executable、config、projectがすべてLinux側にあるとは限りません。一方、Windowsアプリとの共有を目的に`/mnt/c`を使う構成は、意図された正しい構成にもなります。

現在は`which`、`where.exe`、`Get-Command`、PATH確認、WSL確認、path比較を手作業で組み合わせる必要があり、情報が分断され誤解も起きます。

## 3つの質問

1. ExecLocusはどこで動き、エージェントruntimeを示す独立した証拠はあるか。
2. 現在のshell contextでは、どの実行ファイルが選ばれ、どの候補がなぜ負けるか。
3. `/mnt/c`は意図した共有か、選択したworkflow profileとの不一致か。

## 表示する結果

| 結果 | 意味 |
|---|---|
| Runtime／shell | ExecLocus runtime、terminal appearance、agent-runtime evidenceを分離 |
| Command resolution | 選ばれる候補、負ける候補、Windows／Linux／script由来、根拠 |
| Filesystem boundary | Windows native、Windows mounted、WSL native、WSL UNCを分類 |
| Finding | OS境界をまたぐ実行ファイル選択などを決定論的に説明 |
| Evidence state | `observed`、`inferred`、`unavailable`、`failed` |

## 安全性

- 読み取り専用で、PATH、WSL、shell、agent、project設定を変更しない。
- 通常実行はlocal-onlyで、診断uploadやtelemetryを行わない。
- tokenや無制限の環境変数値を収集しない。
- redaction-before-renderingを実装・検証するまで、公開Issueへcommand outputを貼らない。

## 現在とv0.1までの予定

| sourceで利用可能 | v0.1までに必要 |
|---|---|
| runtime／filesystem分類 | Codex／Claudeのinvocation・process adapter |
| executable origin／PATH候補の基盤 | shellに忠実なresolution contractとscenario fixture |
| terminal／pre-alpha JSON | 秘匿化Markdownとprivacy golden test |
| 初期の決定論的rule | checksum付きWindows／Linux release artifact |

正確な実装状況は[対応表](SUPPORT_MATRIX.md)、手作業との違いは[代替手段の比較](research/ALTERNATIVES.md)を参照してください。
