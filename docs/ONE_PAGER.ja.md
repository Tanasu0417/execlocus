# ExecLocus 1ページ紹介

> 同じprojectでも、WindowsとWSLでは選ばれる実行ファイルが違う。ExecLocusは、それぞれの現在の環境で何が選ばれるかを根拠付きで表示します。

## 15秒で説明

WindowsとWSLをまたいでCodex／Claude Codeを使うと、GitやNodeがどちら側の実行ファイルへ解決されるか分かりにくくなります。ExecLocusは、selected candidate、alternative、projectのfilesystem境界と判定根拠を、local・read-onlyの1画面へまとめるCLIです。

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
- Markdownは自動匿名化し、JSON共有時は`--redact`を必須とする。raw terminal／raw JSON出力は公開Issueへ貼らない。

## 現在とv0.1までの予定

| sourceで利用可能 | v0.1までに必要 |
|---|---|
| runtime／filesystem分類 | Codex／Claudeのinvocation・process adapter |
| executable origin／PATH候補の基盤 | shellに忠実なresolution contractとscenario fixture |
| terminal／pre-alpha JSON | 秘匿化Markdownとprivacy golden test |
| 初期の決定論的rule | checksum付きWindows／Linux release artifact |

正確な実装状況は[対応表](SUPPORT_MATRIX.md)、手作業との違いは[代替手段の比較](research/ALTERNATIVES.md)を参照してください。

## 現在お願いしたいこと

公開binaryの利用募集ではなく、Windows／WSLで最近困った経験と、現在使っている確認方法を調査しています。[需要検証計画](research/X_POST_STRATEGY.md)を参照してください。自動秘匿化が完成するまで、公開Issueやreplyへ生の診断出力を貼らないでください。
