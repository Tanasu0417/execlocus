# Claude Design handoff — ExecLocus 60秒MV

- Status: ready for design exploration; real-footage slots remain gated
- Updated: 2026-07-29
- Goal: Claude Designで3方向のkeyframeを比較し、選定後に60秒HTML animaticを制作する

## Claude Designに任せる範囲

Claude Designには、画面設計、keyframe、オリジナルSVGの調整、字幕layout、motion timing、HTML animaticまでを依頼する。実terminal収録、音声収録、最終MP4 encoding、公開操作は別工程とする。

Claude Designはcodebase／fileのimportとHTML、PPTX、PDFへのexportに対応している。MP4を直接の必須納品物にせず、HTML／SVG／keyframeを動画編集工程へ渡す。

公式資料:

- [Claude Design](https://claude.com/product/design)
- [Get started with Claude Design](https://support.claude.com/en/articles/14604416-get-started-with-claude-design)

## 渡すfileと優先順位

次をuploadまたはcodebaseから参照させる。内容が矛盾した場合は番号が小さいfileを優先する。

1. [`SUPPORT_MATRIX.md`](../SUPPORT_MATRIX.md) — 公開可能な機能表現
2. [`RECORDING_SCENARIO.md`](RECORDING_SCENARIO.md) — terminal、path、expected output
3. [`STORYBOARD.ja.md`](STORYBOARD.ja.md) — 8 frame、画面、字幕、ナレーション、gate
4. [`NARRATION.ja.md`](NARRATION.ja.md) — 読み上げ原稿
5. [`OTTER_GUIDE_SPEC.md`](OTTER_GUIDE_SPEC.md) — 顔なしカワウソの造形とmotion
6. [`otter-guide.svg`](assets/otter-guide.svg) — 完全オリジナルのmotion reference
7. [`DEMO_PLAN.md`](../DEMO_PLAN.md) — Conceptから実測版への段階
8. [`ONE_PAGER.ja.md`](../ONE_PAGER.ja.md) — audience、価値、CTA

第三者作品のscreen captureやcharacter imageは渡さない。雰囲気は文章で指定する。

## 最初に貼るprompt

[コピー専用prompt集](CLAUDE_DESIGN_PROMPTS.ja.md)の「Prompt 1」を、そのまま貼り付ける。

## 方向選定後に貼るprompt

[コピー専用prompt集](CLAUDE_DESIGN_PROMPTS.ja.md)の「Prompt 2」を、3案から方向Bを選んだ後に貼り付ける。

## 戻ってきた成果物の確認

- terminal文字が停止して読める。
- Windows／WSLを色だけで区別していない。
- カワウソがterminal、path、evidenceを隠さない。
- 顔、目、鼻、口、ひげ、衣装が追加されていない。
- `Concept`と`REAL FOOTAGE SLOT`が消されていない。
- `SUPPORT_MATRIX.md`でPlannedの機能が実装済みとして描かれていない。
- synthetic path以外のusername、machine名、home、token、private repository名がない。
- reduced-motion版と字幕だけの視聴で意味が保たれる。
- source、font、素材、licenseの一覧がある。

## Conceptから公開版まで

1. Claude Designで3方向のkeyframeを作る。
2. 方向Bを選び、Concept付きHTML animaticを作る。
3. HTML、SVG、PPTX／PDF、design tokensをrepository側でreviewする。
4. UC-02、UC-03、UC-01、自動秘匿化を順次実装する。
5. synthetic fixtureでWindowsとWSLを別々に実録する。
6. `REAL FOOTAGE SLOT`だけを実映像へ置き換える。
7. ナレーション、字幕、必要ならBGMを外部editorで合成する。
8. gate、privacy、version、commit SHAを確認して公開する。

## 公開gate要約

| 表現 | 公開条件 |
|---|---|
| Frame 1–3 mockup | 常時`Concept`なら制作可能 |
| `execlocus`実行 | source prototype、version、commitを表示 |
| Windows／WSL selected path比較 | UC-02 shell contractとpaired golden test完了後 |
| agentが過去に実行した対象 | UC-01 invocation／process evidence完了後のみ |
| `/mnt/c` profile判定 | UC-03 positive／non-triggering test完了後 |
| redacted report | 自動秘匿化とprivacy golden test完了後 |
| 60秒hero MV | UC-01と自動秘匿化完了後 |
| download／release案内 | alpha artifact、checksum、実環境検証後 |
