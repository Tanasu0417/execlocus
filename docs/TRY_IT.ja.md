# ExecLocusを自分のWindows／WSLで試す

この手順は、完成済みのRust CLIを自分の環境で動かし、「表示が正しいか」「判断に役立つか」を確認するためのものです。個人情報を公開せずに試せるよう、補助スクリプトは常に自動匿名化Markdownを`target/user-validation/`へ保存します。このディレクトリはGitの管理対象外です。

> 現在はプレアルファ版です。配布バイナリはまだないため、ソースからビルドします。Rust、依存ライブラリ、Microsoft Build Toolsはいずれも無料で利用できます。初回ビルド時は無料のOSS依存ライブラリを取得するためネットワークを使いますが、ExecLocus本体の通常診断はネットワークへ接続しません。

## まず知っておくこと

次の2つは役割が異なります。

| 確認対象 | できること | 実データ |
|---|---|---|
| [操作デモ](demo/prototype/index.html) | 画面構成、日英切替、診断→比較→説明→共有の操作感を確認 | すべて合成データ |
| Rust CLI | Windows／WSL、ディストリビューション、利用者、シェル、候補実行ファイル、ファイルシステム境界を観測 | 自分のローカル環境 |

操作デモは診断を実行しません。実用性の判断には、以下のCLI確認も行ってください。

## 0. 操作デモで理解しやすさを確認

リポジトリを取得したら、`docs/demo/prototype/index.html`をブラウザで開きます。右上の`EN`／`日本語`で表示言語を切り替えられます。

1. `診断`画面で「どの根拠を分けて表示するか」を確認する。
2. `診断を実行`または`R`キーを押し、カワウソが泳ぎ姿へ切り替わってから`比較`へ移る流れを確認する。
3. `share-first`、`balanced`、`linux-first`を切り替える。
4. `説明`画面で、同じ`/mnt/c`でも重要度と提案が変わることを確認する。
5. `共有`画面で、公開前に何を匿名化するか確認する。
6. `docs/demo/prototype/mv.html`を開き、60秒版の再生、場面選択、日英切替、陸上／泳ぎ姿の使い分けを確認する。

ファイルを直接開いた場合、ブラウザの制限で「コピー」だけ失敗することがあります。表示確認には影響しません。Pythonが入っている場合は、リポジトリ直下で次を実行するとローカル配信できます。

```console
python -m http.server 8765 --directory docs/demo
```

その後、`http://127.0.0.1:8765/prototype/index.html`を開きます。ここで30秒以内に「何を診断するツールか」を説明できなければ、機能ではなく紹介画面の改善点として記録してください。

## 必要なもの

- Git
- Rust 1.85以降（[rustup](https://rustup.rs/)から無料で導入）
- Windows: Microsoft C++ Build ToolsとWindows SDK
- WSL: Ubuntu 24.04を主対象とし、`cc`などのCリンカー

リポジトリは`C:\WINDOWS\system32`ではなく、自分が書き込みできる通常の作業フォルダへ取得します。`git clone`は`execlocus`フォルダを新しく作るため、**cloneの後に**そのフォルダへ移動してください。

Windows PowerShellの場合:

```powershell
Set-Location ([Environment]::GetFolderPath("MyDocuments"))
git clone https://github.com/Tanasu0417/execlocus.git
Set-Location .\execlocus
git rev-parse --show-toplevel
if (-not (Test-Path .\scripts\try-execlocus.ps1)) { throw "リポジトリ直下ではないか、取得内容が古い状態です" }
```

WSLの場合:

```bash
cd ~
git clone https://github.com/Tanasu0417/execlocus.git
cd execlocus
git rev-parse --show-toplevel
test -f scripts/try-execlocus.sh || { pwd; echo "リポジトリ直下ではないか、取得内容が古い状態です"; exit 1; }
```

すでにclone済みの場合は、cloneを繰り返さず、そのリポジトリ直下へ移動して`git pull --ff-only`を実行します。`git rev-parse --show-toplevel`の出力と現在地が一致し、`scripts`フォルダが見えることを確認してください。

このプレアルファ版を再現可能に確認する場合は、試したコミットIDも控えてください。

## 1. Windows PowerShellで確認

リポジトリ直下のPowerShellで実行します。

```powershell
& .\scripts\try-execlocus.ps1
```

利用目的を選ぶ場合:

```powershell
& .\scripts\try-execlocus.ps1 -Profile share-first
& .\scripts\try-execlocus.ps1 -Profile linux-first
```

`&`で現在のPowerShellセッションから実行すると、`Get-Command -All`でalias／function／cmdletも確認してから診断します。出力先は`target/user-validation/windows-<profile>.md`と`windows-<profile>.redacted.json`です。候補の詳細は匿名化MarkdownとJSONの両方で確認できます。未加工の詳細も手元だけで確認したい場合は`-ShowLocalDetails`を加えます。詳細表示には個人の絶対パスが含まれる場合があるため、画面共有やIssueへの貼り付けはしないでください。

## 2. WSLで確認

同じリポジトリをWSLのbashから開いて実行します。

```bash
source ./scripts/try-execlocus.sh
```

利用目的を選ぶ場合:

```bash
source ./scripts/try-execlocus.sh share-first
source ./scripts/try-execlocus.sh linux-first
```

出力先は`target/user-validation/wsl-<profile>.md`と`wsl-<profile>.redacted.json`です。未加工の詳細を手元だけで見る場合:

```bash
(SHOW_LOCAL_DETAILS=1 source ./scripts/try-execlocus.sh balanced)
```

`source`で現在のbashセッションから実行すると、alias／function／builtinとPATH候補を同じセッションの根拠として診断へ渡します。alias展開やfunction本体は保存せず、種類と固定コマンド名だけを一時JSONへ記録します。一時JSONは実行終了時に削除されます。

`not recognized as the name of a script file`または`No such file or directory`になった場合、診断処理はまだ始まっていません。次を確認してください。

- `git rev-parse --show-toplevel`が現在のExecLocusリポジトリを示している
- Windowsでは`Test-Path .\scripts\try-execlocus.ps1`が`True`になる
- WSLでは`test -f scripts/try-execlocus.sh && echo OK`が`OK`を表示する
- clone済みの古い作業コピーでは`git pull --ff-only`を実行済みである

WindowsのDocumentsにある同じリポジトリをWSLから開く場合は、次のようにDocumentsの実パスを取得できます。

```bash
documents_win="$(powershell.exe -NoProfile -Command "[Environment]::GetFolderPath('MyDocuments')" | tr -d '\r')"
cd "$(wslpath "$documents_win")/execlocus"
source ./scripts/try-execlocus.sh balanced
```

この方法でWindows側の`windows-balanced.*`とWSL側の`wsl-balanced.*`を同じ`target/user-validation/`へ出力し、同じソースに対する差を比較できます。WSLネイティブ配置も確認する場合は、個人情報を含まない別のテスト用コピーをWSLのホーム側へ置いて比較してください。

## 3. 何を確認するか

Markdownと匿名化JSONをローカルで開き、次を確認します。

| 項目 | 確認すること |
|---|---|
| 実行環境 | Windows実行ではWindows、WSL実行ではWSLとして表示されるか |
| ディストリビューション | WSLでUbuntuの名前と版が妥当か |
| シェル | PowerShell／bashなどの根拠と確度が妥当か |
| エージェント | Codex／Claude Codeの親プロセス根拠がある時だけ推定されるか。通常のターミナルからの実行で`Unknown`でも異常ではない |
| 実行ファイル | `Not found`、`Candidates found / selection unconfirmed`、`Selected`、`Probe failed`が状況に合うか。Git、Node、npm、エージェントの候補数、Windows／Linux由来、PE／ELF／scriptが独立確認と一致するか |
| プロジェクト | `/mnt/c`かWSLネイティブかが正しいか |
| 診断 | `share-first`、`balanced`、`linux-first`で説明が利用目的に合うか |
| 匿名化 | 利用者名、マシン名、ホーム、個人の絶対パスが共有用Markdownに残っていないか |

CLI単体では親シェルのエイリアスや関数を安全に復元できません。補助スクリプトは現在のセッションから必要最小限の種類だけを渡します。補助スクリプトを使わずsession根拠が不足する場合、外部候補を表示しても`Candidates found / selection unconfirmed`のままにします。

## 4. 役に立つかを判定する

次の4問に答えてください。

1. 手作業では気づかなかったWindows／WSLの境界が見つかったか。
2. 結論を`Get-Command`、`where.exe`、`command -v`などで独立確認できたか。
3. 何を変更すべきか、または現状を維持すべきか判断できたか。
4. 同じ疑問が起きた時に再利用したいか。

3問以上が「はい」なら、初期仮説に対して強い有用性シグナルです。誤判定、説明不足、理解に2分以上かかった箇所も重要な改善材料です。

## 5. 安全にフィードバックする

[フィールドレポート](https://github.com/Tanasu0417/execlocus/issues/new?template=field_report.yml)には、環境カテゴリ、確認結果、所要時間、判断だけを記入します。

- トークン、パスワード、APIキー、認証情報を貼らない
- 未加工のターミナル出力やJSONを貼らない
- 利用者名、マシン名、ホームディレクトリ、個人の絶対パス、非公開リポジトリ名を貼らない
- 匿名化Markdownも公開前に自分の目で再確認する

問題が起きた場合は、公開して安全な範囲で「OS種別」「WSLディストリビューション」「実行したプロファイル」「期待した結論と実際の結論」だけを報告してください。
