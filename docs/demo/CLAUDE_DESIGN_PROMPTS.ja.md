# Claude Design copy-paste prompts — ExecLocus 60秒MV

- Status: ready to paste
- Updated: 2026-07-29
- Usage: 参照fileを渡した後、Prompt 1、選定、Prompt 2の順で貼る

## Prompt 1 — 3方向を比較する

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
・丸い小さな頭を胴へ直接つなぎ、小さな丸耳、横長で丸い胴、先端が丸い4本の短い脚、胴体より短く根元が太い尾でカワウソと判別可能にする
・3/4横向きの泳ぐ姿勢を基本とする
・通信markerはsilhouetteの外へ置き、目や体内器官に見せない
・SVG／CSSで滑らかに動かし、production rigでは胴体より尾を遅れて追従させる
・発話は口パクではなく、silhouette外側の位置marker、通信波、字幕、音声波形で示す
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

## Prompt 2 — 方向Bを60秒animaticへ展開する

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

## Claude Designから戻すもの

HTML、ZIP、SVG、PPTXまたはPDFをdownloadし、このtaskへ添付する。公開操作はせず、まずrepository側で機能表現、privacy、素材licenseを確認する。
