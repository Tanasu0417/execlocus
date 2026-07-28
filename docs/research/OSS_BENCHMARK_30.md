# 個人所有OSS 30件ベンチマーク

調査日: 2026-07-28

この調査は、ExecLocusの機能を30製品の平均へ寄せるためではない。個人アカウントから成長したCLIが、価値をどう絞り、READMEでどう伝え、どのように配布とリリースを継続しているかを比較し、ExecLocusに適用できる再現性の高い型を抽出するために実施した。

- [行単位の比較データ（CSV）](oss_benchmark_30.csv)
- [集計・書式付きワークブック（XLSX）](oss_benchmark_30.xlsx)
- [ExecLocus公開・発信プレイブック](LAUNCH_PLAYBOOK.md)

## 結論

ExecLocusの勝ち筋は「一般的なシステム情報ツール」になることではない。

> Know exactly where your AI coding agent runs, which executable it resolves, and why.

日本語では「AIコーディングエージェントが、どの実行環境で、どの実行ファイルを選び、なぜその結果になったかを証拠付きで説明する」が中核となる。

30件に直接同じ製品はなかった。一方、次の隣接需要は大きい。

- 実行環境を見えるようにする需要: `neofetch`、`bottom`、`btop`、`procs`
- WindowsとWSLの境界を扱う需要: `gsudo`、`npiperelay`
- シェルごとの差を吸収する需要: `zoxide`、`just`、`navi`
- 実行物の由来と構造を調べる需要: `binsider`
- 生データを人向けの判断へ変える需要: `duf`、`dive`
- 既存の難しいコマンドを分かりやすく置き換える需要: `ripgrep`、`bat`、`fd`、`dust`

したがって、競合不在を市場不在と決めつける必要はない。ただし、隣接市場のスター数だけではExecLocus固有の需要は証明できない。重い機能開発の前に、実際のWindows／WSL利用者10人で診断を行い、少なくとも3人から「意図しない実行環境または実行ファイルを発見した」という結果を得ることを需要検証ゲートとする。

## 調査方法

最初に50件の候補を収集し、次の条件で30件へ絞った。

1. GitHub上の所有者種別が`User`である
2. CLI、TUI、診断、開発環境、クロスプラットフォーム、Windows／WSLのいずれかで学習価値がある
3. スター、更新、リリース、READMEのいずれかに明確な成功シグナルがある
4. 似た製品だけで30件を埋めず、伝え方と配布方法を比較できる

GitHub APIからスター、fork、作成日、最終push、言語、ライセンス自動判定、Issue、PR、Releaseを取得した。READMEは同一の規則で、デモ表現、Install／Usage／Comparison／Benchmark／Contributing見出し、スポンサー表現、主要パッケージマネージャー名を観測した。

### 制約

- `User`所有は「現在個人アカウントに置かれている」ことを示すだけで、全コードを1人で作った証明ではない
- スターは認知・関心のシグナルであり、品質、利用継続、収益を直接表さない
- READMEの特徴量は表記揺れを含むため、定量値は厳密なサイト監査ではなく比較用シグナルである
- pushにはbotや文書更新も含まれ得る
- `neofetch`は継承とbus factorの教訓としてアーカイブ済みのまま含めた
- `npiperelay`はスター規模よりもWindows／WSL境界への近さを優先した
- これは競合の完全列挙ではない

## 定量サマリー

| 指標 | 結果 |
|---|---:|
| 対象プロジェクト | 30 |
| 合計スター | 782,962 |
| スター中央値 | 16,560.5 |
| 平均スター | 26,099 |
| Rustプロジェクト | 20 / 30 |
| 直近90日以内にpush | 26 / 30 |
| Release合計 | 2,063 |
| Release中央値 | 54.5 |
| READMEにデモ表現 | 19 / 30 |
| READMEにInstallセクション | 24 / 30 |
| READMEにUsage系セクション | 21 / 30 |
| 観測した配布経路の中央値 | 6 |

## 繰り返し現れた7つの型

### 1. 一文で既知の問題へ接続する

`bat`はcat、`fd`はfind、`dust`はdu、`procs`はpsという既知の概念に接続している。利用者は新しいカテゴリを学ばずに価値を理解できる。

ExecLocusも「環境情報を表示するCLI」では弱い。「Codex／Claude Codeが実際にどこで何を実行しているか説明するCLI」と表現する。

### 2. 最初の10秒を画像または端末デモにする

19件にREADME上のデモシグナルがあった。`lazygit`、`fzf`、`btop`、`gsudo`のように、出力や操作が価値そのものになる製品は特に強い。

ExecLocusでは、同じ`git`コマンドがWindowsとWSLで異なる実行ファイルへ解決される例を1つ用意する。説明動画ではなく、問題と解決が1画面で完結する端末記録を優先する。

### 3. 人向け出力と自動化向け出力を両立する

`dive`や`dua-cli`は対話・可視化だけでなく、非対話の利用経路も持つ。CLIは人が見る入口とCI／エージェントが読む契約を分けた方が広がる。

ExecLocusは次を同時に維持する。

- 既定の短いterminal要約
- 安定したschema version付きJSON
- 問題が見つかったときの非0終了コード
- 将来の詳細explore機能

### 4. 結論だけでなく証拠と限界を出す

`ripgrep`のWhy／Why not、`difftastic`のNon-goals、`hyperfine`の測定方法は、誠実な制約説明が信頼を作る例である。

ExecLocusでは、各診断に`evidence`、`confidence`、`probe version`を残す。推測を事実として表示せず、判定不能を正式な状態として扱う。

### 5. Windowsとシェル差を後付けにしない

`zoxide`、`just`、`xh`、`bottom`、`gsudo`は、複数シェルやWindowsの導入経路を主要文書に含めている。ExecLocusではWindows Native、WSL、PowerShell、cmd、Bashを最初から製品マトリクスに置く。

### 6. 配布は機能である

観測できたパッケージマネージャー記載の中央値は6だった。ただし、v0.1.0で9種類を同時に保守する必要はない。最初はGitHub Releasesの検証済みバイナリ、crates.io、WinGetまたはScoopの順で広げる。

### 7. 共有したくなる出力が自然な発信になる

`neofetch`は端末スクリーンショットそのものが共有物になった。ExecLocusでも、IssueやAIエージェントの会話へ貼れる匿名化済み診断カードを用意できる。ただしusername、home、machine名、絶対パス、環境変数値は既定で公開しない。

## 30プロジェクト

RelevanceはExecLocusへの学習近接度であり、ソフトウェア品質の点数ではない。詳細列は[CSV](oss_benchmark_30.csv)と[XLSX](oss_benchmark_30.xlsx)に収録した。

| Rank | Project | Stars | Category | Relevance | 最重要の学び |
|---:|---|---:|---|---:|---|
| 1 | [junegunn/fzf](https://github.com/junegunn/fzf) | 82,003 | 検索・ナビゲーション | 4 | 10秒デモとシェル／エディタ統合 |
| 2 | [jesseduffield/lazygit](https://github.com/jesseduffield/lazygit) | 80,807 | 開発ワークフロー | 3 | Elevator Pitchと視覚的成功体験 |
| 3 | [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep) | 66,604 | 検索・ナビゲーション | 4 | Why／Why notと再現可能な説明 |
| 4 | [sharkdp/bat](https://github.com/sharkdp/bat) | 59,898 | 診断・表示 | 4 | 既知コマンドの置換と美しい出力 |
| 5 | [wagoodman/dive](https://github.com/wagoodman/dive) | 54,371 | 診断・表示 | 3 | 可視化とCI向け非対話モード |
| 6 | [sharkdp/fd](https://github.com/sharkdp/fd) | 43,898 | 検索・ナビゲーション | 4 | 簡単な代替表現と幅広い配布 |
| 7 | [ajeetdsouza/zoxide](https://github.com/ajeetdsouza/zoxide) | 38,297 | 実行環境・シェル | 5 | 全主要シェルを第一級に扱う |
| 8 | [casey/just](https://github.com/casey/just) | 34,984 | 実行環境・シェル | 5 | Windows／Unix差を徹底して文書化 |
| 9 | [aristocratos/btop](https://github.com/aristocratos/btop) | 33,681 | システム診断 | 4 | 共有したくなる特徴的な画面 |
| 10 | [dandavison/delta](https://github.com/dandavison/delta) | 31,578 | 開発ワークフロー | 3 | 既存Gitフローへ自然に統合 |
| 11 | [sharkdp/hyperfine](https://github.com/sharkdp/hyperfine) | 28,549 | 診断・再現性 | 5 | 測定方法とエクスポートで信頼を作る |
| 12 | [Wilfred/difftastic](https://github.com/Wilfred/difftastic) | 25,697 | 開発ワークフロー | 3 | 技術的差別化とNon-goals |
| 13 | [dylanaraps/neofetch](https://github.com/dylanaraps/neofetch) | 23,711 | システム診断 | 5 | 共有可能な出力とbus factorの両方 |
| 14 | [antonmedv/fx](https://github.com/antonmedv/fx) | 20,548 | 診断・表示 | 3 | 極小READMEと完成度の高いデモ |
| 15 | [denisidoro/navi](https://github.com/denisidoro/navi) | 17,368 | 実行環境・シェル | 4 | コミュニティが拡張できるコンテンツ |
| 16 | [mikefarah/yq](https://github.com/mikefarah/yq) | 15,753 | データ・自動化 | 3 | Quick Usageから詳細文書への段階設計 |
| 17 | [muesli/duf](https://github.com/muesli/duf) | 15,216 | システム診断 | 4 | 生データを即読できる表へ圧縮 |
| 18 | [XAMPPRocky/tokei](https://github.com/XAMPPRocky/tokei) | 14,729 | 診断・再現性 | 4 | 結果例と埋め込み可能な出力 |
| 19 | [ClementTsang/bottom](https://github.com/ClementTsang/bottom) | 13,805 | システム診断 | 5 | 対応OSと支援範囲の明示 |
| 20 | [Canop/broot](https://github.com/Canop/broot) | 12,853 | 検索・ナビゲーション | 3 | 機能でなくユースケース別に説明 |
| 21 | [orhun/git-cliff](https://github.com/orhun/git-cliff) | 12,058 | リリース・発信 | 3 | 変更履歴を自動化し露出面を増やす |
| 22 | [bootandy/dust](https://github.com/bootandy/dust) | 12,042 | システム診断 | 4 | Why、Demo、Alternativesの順序 |
| 23 | [ducaale/xh](https://github.com/ducaale/xh) | 7,968 | データ・自動化 | 3 | OS別のコピー可能な導入方法 |
| 24 | [svenstaro/miniserve](https://github.com/svenstaro/miniserve) | 7,741 | データ・自動化 | 2 | 30秒で試せる約束と例題集 |
| 25 | [dalance/procs](https://github.com/dalance/procs) | 6,112 | システム診断 | 5 | Platform機能差を表で管理 |
| 26 | [Byron/dua-cli](https://github.com/Byron/dua-cli) | 6,065 | システム診断 | 4 | 非対話モードと対話モードの分離 |
| 27 | [gerardog/gsudo](https://github.com/gerardog/gsudo) | 6,010 | Windows・WSL | 5 | Windowsを主役にしWSLレシピまで提供 |
| 28 | [PaulJuliusMartinez/jless](https://github.com/PaulJuliusMartinez/jless) | 5,446 | 診断・表示 | 3 | 狭い用途と専用ブランド |
| 29 | [orhun/binsider](https://github.com/orhun/binsider) | 4,358 | 実行ファイル解析 | 5 | formatとoriginを段階的に可視化 |
| 30 | [jstarks/npiperelay](https://github.com/jstarks/npiperelay) | 812 | Windows・WSL | 5 | 狭いWSL境界問題にも具体的需要がある |

## ExecLocusで採用する型

### READMEの最初の画面

1. 問題を一文で示す
2. Windows／WSLの誤認を示す短い端末デモ
3. `execlocus`と`execlocus --json`の2コマンド
4. Local-only、read-only、redacted by default
5. 対応範囲とNon-goals

### 最初の3シナリオ

1. Codex／Claude CodeがWindows版かWSL版か確認する
2. `git`、`node`、`python`等がどの実行ファイルへ解決されたか確認する
3. `/mnt/c`上の共有プロジェクトが意図した境界か、混在事故かを証拠付きで確認する

### 維持すべきNon-goals

- PATH、シェル設定、WSL設定を自動変更しない
- AIエージェント自体を置き換えない
- 汎用システムモニターにならない
- `/mnt/c`を一律に問題扱いしない
- 根拠のない「best practice」を強制しない

## 開発ゲート

v0.1.0へ進む前に次を満たす。

- WindowsとUbuntu-24.04のCIが継続して成功する
- terminal／JSONの主要フィールドがschemaとして固定される
- username、home、machine名、絶対パス、秘密環境変数のredactionテストがある
- 実際のWindows／WSL環境10件で試し、3件以上の意図しない差異を発見する
- 3シナリオをREADMEからコピーして再現できる
- Windows／LinuxのReleaseバイナリとSHA-256 checksumを生成できる

達成後の公開順序は[Launch Playbook](LAUNCH_PLAYBOOK.md)に定義する。
