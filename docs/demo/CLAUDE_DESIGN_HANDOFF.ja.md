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

```text
ExecLocusというWindows／WSL向けread-only診断CLIの、
60秒コンセプトMV用インタラクティブHTML animaticを作成してください。

視聴者:
WindowsとWSLを併用するCodex／Claude Code利用者。

目的:
同じprojectでもWindowsとWSLでは選ばれる実行ファイルが違い得ること、
ExecLocusがselected candidateと根拠を表示することを60秒で理解させる。

正本の優先順位:
1. SUPPORT_MATRIX.md
2. RECORDING_SCENARIO.md
3. STORYBOARD.ja.md
4. NARRATION.ja.md
5. OTTER_GUIDE_SPEC.md
6. DEMO_PLAN.md
7. ONE_PAGER.ja.md
矛盾した場合は上位を優先し、実装済みか不明な機能を補完しないでください。
SVGはmotion／silhouetteのreferenceであり、診断機能の正本ではありません。

ビジュアル:
・暗いサイバー作戦管制画面
・terminalと診断evidenceが主役
・WindowsとWSLを位置とlabelで明確に区別
・悪意あるhacking、侵入、skull、文字雨は使用しない
・細い接続線、控えめなglow、深い青緑を基調とする
・色だけで状態を表さない
・第三者作品のcharacter、screen、logoを再現しない

案内character:
・完全オリジナルの顔なしカワウソsilhouette
・目、鼻、口、ひげ、服、装飾品を付けない
・細長い胴、短い脚、低い頭、根元が太く先端が細い長い尾で判別可能にする
・3/4横向きの泳ぐ姿勢を基本とする
・SVG／CSSで滑らかに動かし、胴体より尾を遅れて追従させる
・発話は口パクではなく、胸の位置marker、通信波、字幕、音声波形で示す
・画面面積の15%以内を目安とし、terminalとevidenceを隠さない
・prefers-reduced-motion相当の静止版も作る

制作条件:
・1920×1080、16:9、30fps相当、60秒
・1:1 crop用の中央safe areaを表示切替可能にする
・8 frameをtimelineで再生、停止、前後移動できる
・各frameの時間、画面、字幕、ナレーションを編集可能にする
・mockup画面には左上へ常時「Concept」と表示する
・terminal内容はRECORDING_SCENARIO.mdのsynthetic pathだけを使う
・実映像を入れる場所は「REAL FOOTAGE SLOT」と明示する
・本物らしい架空の成功画面を作らない
・点滅、激しいcamera shake、読めない高速scrollを使わない
・字幕だけでも意味が成立するようにする

最初から完成版を1案に決めず、同じFrame 1、5、8について次の3方向を提示してください。
A: 精密で静かな観測室
B: カワウソがevidence経路を泳いで案内する電脳空間
C: terminal中心の硬派なdeveloper console

各方向について、可読性、信頼感、characterの目立ち方、実映像への差し替えやすさを説明してください。
```

## 方向選定後に貼るprompt

```text
方向Bを基礎にしてください。ただしterminalとevidenceを最優先とし、
カワウソは情報経路を案内する補助役に限定してください。

STORYBOARD.ja.mdとNARRATION.ja.mdに沿う8 frame・60秒のHTML animaticを作成してください。

納品物:
1. standalone HTML
2. source ZIP
3. 8 frameのPPTXまたはPDF
4. 編集可能なオリジナルのカワウソSVGとpose一覧
5. 色、書体、spacing、motion durationのdesign tokens
6. 16:9と1:1のcrop guide
7. reduced-motion版
8. REAL FOOTAGE SLOT一覧
9. 字幕timing一覧
10. 使用素材、font、license一覧

全frameについて公開gateを表にし、未通過項目は必ずConcept表示のままにしてください。
第三者作品のscreen、character、音声、logoを参照素材として埋め込まないでください。
```

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
