# WSL Project Workbench 製品仮説

- 更新日: 2026-07-30
- 費用方針: 追加費用0円、ローカル実行を既定とする

## 結論

「プロジェクトごとに正しいWindows／WSL環境を選び、必要なシェルと開発ツールを1回で開く」という課題には日常的な利用価値があります。ただし、ターミナルやWSL設定画面をExecLocusが一から作り直すことは勝ち筋ではありません。

Windows TerminalはPowerShellやWSLディストリビューションのプロファイルを自動生成し、`wt.exe`から指定プロファイルを開けます。新しいWSLには公式のWSL Settingsもあり、`.wslconfig`の主要設定をGUIで扱えます。ExecLocusはこれらを置き換えず、**プロジェクト、起動先、権限、診断、開発コマンドを結び付ける作業台**になります。

## 利用者が解決したいこと

- このプロジェクトはPowerShell、コマンドプロンプト、Ubuntuのどれで開くべきか迷う。
- 同じ名前のGit、Node、Codex、Claude Codeが複数あり、起動後の環境が意図と合っているか分からない。
- 管理者シェルが必要な作業と、通常権限で行う開発を混同したくない。
- 複数のWSLディストリビューション、ユーザー、作業ディレクトリを覚えたくない。
- WSLのメモリ、CPU、swapを変更した影響と、反映に必要な再起動を安全に把握したい。
- エラーが起きたとき、環境を探し直さず、その場でDoctorと匿名共有を使いたい。

## MVPの画面

| 画面 | 役割 | MVPで行うこと |
|---|---|---|
| Projects | 日常の入口 | フォルダ、期待OS層、WSLディストリビューション、シェル、通常／管理者、開発コマンドを登録する |
| Launch | 正しい環境を開く | Windows TerminalのPowerShell、CMD、指定WSLプロファイルを正しい作業フォルダで開く |
| Preflight | 起動事故を止める | 実際に選ばれるGit・Node・npm・Codex・Claude Codeを確認し、開始可否を1結論で示す |
| Tasks | 開発を始める | 登録済みのdev、test、buildを選択した環境で起動し、ポートと終了状態を表示する |
| WSL | 状態を把握する | 導入済みディストリビューション、WSLバージョン、停止／実行状態、既定ディストリビューションを読み取り専用で表示する |
| Resources | 設定事故を避ける | 現在のグローバル設定と影響範囲を説明し、公式WSL Settingsを開く。直接編集は後段で検討する |
| Doctor | 問題を解く | 診断シナリオ、実機診断、before／after、匿名共有を1か所へまとめる |

## 既存機能との分担

- **Windows Terminal**: タブ、ペイン、文字描画、シェルプロファイルを担当する。ExecLocusは `wt.exe -p <profile>` などの構造化された引数で起動する。
- **WSL CLI**: `wsl --list --online`、`wsl --install`、`wsl --status`、`wsl --update`、`wsl --shutdown`などの公式操作を担当する。
- **WSL Settings**: `.wslconfig`のGUI編集を担当する。ExecLocusはプロジェクトへの影響、現在値、再起動の必要性を説明して公式画面へ渡す。
- **ExecLocus**: プロジェクトの期待値、実測された実行根拠、起動先、権限、開発タスク、診断履歴を結び付ける。

## 権限と安全設計

| 操作 | 権限・影響 | 製品上の扱い |
|---|---|---|
| 通常のPowerShell／CMD／WSLを開く | 通常権限 | 1クリックで実行可能 |
| 管理者PowerShell／CMDを開く | UAC確認が必要 | 「管理者」バッジを分離し、OSのUACを迂回しない |
| 新しいディストリビューションを表示 | 読み取り専用 | `wsl --list --online`から取得し、一覧をハードコードしない |
| ディストリビューションを導入 | ダウンロード、容量、場合により管理者権限・再起動 | サイズ・操作内容・コマンドを先に示し、明示確認後だけ実行する |
| `.wslconfig`を変更 | 全WSL2ディストリビューションへ影響 | MVPでは現在値と公式Settingsへの導線を提供する |
| `wsl --shutdown` | 全ディストリビューションと実行中処理を停止 | 実行中環境と影響を表示し、明示確認を必須にする |
| `wsl --terminate` | 指定ディストリビューションを停止 | 対象名を表示し、明示確認を必須にする |
| `wsl --unregister` | データを恒久削除 | MVP対象外。通常のGUIには置かない |

`.wslconfig`はWindowsユーザー単位でWSL2全体に作用します。ディストリビューション固有の設定は `/etc/wsl.conf` であり、プロジェクト単位のメモリ割り当てではありません。この違いをGUI上で明記します。

## 最初に作る縦の流れ

1. プロジェクトフォルダを登録する。
2. 「Windows」「Ubuntu-24.04」など、期待する実行先を選ぶ。
3. Preflightで実測し、「開始してよい／確認が必要／開始しない」を表示する。
4. 問題がなければ、正しいTerminalプロファイルと作業ディレクトリで開く。
5. dev／test／buildを起動する。
6. 失敗したらDoctorを開き、前回との差分と匿名化済み共有情報を作る。

この一連の流れなら、ExecLocusは「たまに開く情報画面」ではなく、開発開始時に毎日使うランチャー兼ガードになります。

## 実装しないもの

- 独自ターミナルエミュレーター
- パスワードや管理者資格情報の保存
- UACの回避
- 未確認のPATH、シェル設定、WSL設定の自動修正
- WSLディストリビューションの無確認削除
- ホスト型AIや有料クラウドを前提とする提案機能

## 無料で構成できる範囲

Tauri、Rust、Windows Terminal、WSL CLI、WSL Settingsをローカルで連携する範囲では、ExecLocus開発による追加利用料は0円です。ディストリビューションのダウンロードには通信量と保存容量が必要ですが、ExecLocusが課金APIを呼ぶことはありません。

## 参考となる公式仕様

- [Windows Terminalの動的プロファイル](https://learn.microsoft.com/en-us/windows/terminal/dynamic-profiles)
- [Windows Terminalのコマンドライン引数](https://learn.microsoft.com/en-us/windows/terminal/command-line-arguments)
- [WSLの基本コマンド](https://learn.microsoft.com/en-us/windows/wsl/basic-commands)
- [WSLの高度な設定（.wslconfigとwsl.conf）](https://learn.microsoft.com/en-us/windows/wsl/wsl-config)
- [MicrosoftによるWSL Settings GUIの紹介](https://devblogs.microsoft.com/commandline/whats-new-in-the-windows-subsystem-for-linux-in-may-2024/)
