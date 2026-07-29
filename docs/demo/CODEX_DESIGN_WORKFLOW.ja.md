# Codex側でのExecLocus design制作workflow

- Updated: 2026-07-29
- Cost policy: default incremental cost JPY 0

## 結論

今回必要な範囲はCodex側だけでも制作できる。repository-nativeなHTML、CSS、JavaScript、SVG、PPTXと、その表示・操作・overflow検査を同じcode reviewへ載せられる。Claude Designは別方向のkeyframe探索やmotion案の比較に使えるが、現在の成果物を完成させるための必須サービスではない。

## 今回Codex側で作成したもの

- [`prototype/index.html`](prototype/index.html): profile切替、診断transition、Windows／WSL比較、rule説明、匿名化共有を操作できるconcept UI。
- [`prototype/mv.html`](prototype/mv.html): 8 frame、60秒、caption、timeline、play／pause／resetを持つanimatic。
- [`assets/otter-guide.svg`](assets/otter-guide.svg): 目、鼻、口、ひげ、衣装を使わない承認済み陸上silhouette。
- [`assets/otter-swim.svg`](assets/otter-swim.svg): 同じ造形方針の泳ぎsilhouette。
- [`ExecLocus_intro_ja.pptx`](ExecLocus_intro_ja.pptx): WSL2利用者向けの7 slide紹介deck。

これらはhosted deploymentや有料音声APIを使わずに作成した。カワウソ候補だけはCodex内蔵の画像生成で比較案を作り、承認されたA（陸上）／D（泳ぎ）をローカル処理で単純化したSVG pathへ変換した。添付illustrationは一般的な体型の参考に限定し、輪郭をtraceせずrepositoryへ同梱していない。prototypeはsynthetic値だけを表示し、実際のprobeを実行しない。

## Codexで追加利用できる表現手段

- Image Generation: bitmap key visualやillustrationが必要な場合。
- Visualizations: 関係性や比較を対話的に説明する場合。
- Sites: landing pageを実装・公開する場合。

このprojectでは、subscription quotaを消費する可能性がある生成、または外部公開を伴う操作は`COST_POLICY.md`に従い、実行直前に個別承認を取る。現在のlocal prototypeとPPTXには使用していない。

公式資料:

- [Image generation](https://learn.chatgpt.com/docs/image-generation)
- [Visualizations](https://learn.chatgpt.com/docs/visualizations)
- [Sites](https://learn.chatgpt.com/docs/sites)

## Claude Designへ渡す場合

既存の[`CLAUDE_DESIGN_HANDOFF.ja.md`](CLAUDE_DESIGN_HANDOFF.ja.md)に加え、`prototype` directoryとPPTXを渡す。実装済み機能を再解釈させず、motion、字幕layout、16:9から1:1へのcropだけを比較対象にする。返却物はrepositoryへ入れる前にlicense、font、外部asset、個人情報、公開gateを再確認する。
