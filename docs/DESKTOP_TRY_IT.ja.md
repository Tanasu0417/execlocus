# ExecLocus Windowsデスクトップ開発版を試す

この手順では、ブラウザではなくWindowsアプリのウィンドウで、同じ読み取り専用診断GUIを試します。現在はプレアルファの**署名なし開発版**です。インストーラーや自動更新はなく、一般配布用バイナリではありません。

## 費用と通信

- Tauri、Rust、WebView2、Microsoft C++ Build Toolsは、このローカルビルドで追加の利用料金を発生させません。
- 初回ビルドだけ、無料OSS依存ライブラリを取得するためにcrates.ioへ接続します。
- 起動後の通常診断は外部API、AIモデル、テレメトリ、クラウドへ接続しません。
- 画面と診断APIは、同じプロセスが開くランダムな`127.0.0.1`ポートだけを使います。

## 必要なもの

- Windows 10 1803以降またはWindows 11
- Rust 1.88.0以降（CLI本体だけを使う場合のMSRVは1.85.0）
- Microsoft C++ Build ToolsとWindows SDK
- Microsoft Edge WebView2 Runtime（通常は対応Windowsに導入済み）

PowerShellでリポジトリ直下へ移動し、現在地を確認します。

```powershell
git rev-parse --show-toplevel
Test-Path .\scripts\build-desktop.ps1
```

2つ目が`True`なら、開発版をビルドします。

```powershell
& .\scripts\build-desktop.ps1 -Configuration Debug
```

初回は依存取得とコンパイルに時間がかかります。以降は差分だけを再ビルドします。完成したexeはGit管理外の次の場所に生成されます。

```text
src-tauri\target\debug\execlocus-desktop.exe
```

## 署名状態を確認して起動

```powershell
$desktop = Resolve-Path .\src-tauri\target\debug\execlocus-desktop.exe
Get-AuthenticodeSignature -LiteralPath $desktop | Select-Object Status
& $desktop
```

開発版では`Status`が`NotSigned`になるのが想定どおりです。Windowsの警告が出た場合は、自分がこのリポジトリの確認済みソースからビルドしたexeかを確認してください。確認できないexeは実行しないでください。コード署名やStore配布は費用と本人確認を伴う可能性があるため、別途判断するリリース項目です。

## 操作確認

画面の目的、3つの診断基準、自動匿名化、各ページの読み方は[GUI操作ガイド](GUI_MANUAL.ja.md)にまとめています。

1. `診断を実行`を押し、`完了・外部送信 0件`になることを確認する。
2. `比較`で5ツールの概要を先に確認する。
3. 必要な行だけ展開し、候補、由来、PE／ELF／script、選択理由、確認コマンドを確認する。
4. `すべて展開`と`すべて閉じる`を試す。
5. `説明`で影響、推奨対応、再確認手順を確認する。
6. `EN`／`日本語`で実診断結果も切り替わることを確認する。
7. `共有`では匿名化Markdownだけが表示されることを確認する。
8. ウィンドウを閉じ、同じプロセスのloopback待受も終了することを確認する。

`診断`、`比較`、`説明`にはローカル絶対パスが表示される場合があります。公開用スクリーンショットには使わず、共有候補は`共有`画面だけにしてください。

## ブラウザ版との違い

デスクトップ版はブラウザのタブやアドレスバーを表示しませんが、診断ロジック、日英切替、匿名化境界は同じです。WebViewからの画面遷移は、起動時に割り当てた`127.0.0.1`の同一ポートだけに制限します。

デスクトップアプリ単体では、すでに開いているPowerShellセッションのaliasやfunctionを復元できません。そのセッションで実際に選ばれるコマンドを厳密に確認したい場合は、従来のwrapperを使います。

```powershell
& .\scripts\try-execlocus.ps1 -Gui -Language ja -Profile balanced
```

Windows／WSL自動比較を試す場合は、WSL側のExecLocusリポジトリで`bash scripts/install-wsl-companion.sh`を1回実行します。Windowsアプリは同じ起動ディレクトリを両側から読み取り専用で観測し、異なる項目を先に表示します。外部送信や自動修正は行いません。
