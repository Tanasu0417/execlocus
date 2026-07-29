# ExecLocus demo kit

更新日: 2026-07-29

このdirectoryは、説明用mockupと実測demoを混同せずに制作するための正本である。

- [60秒紹介MVの絵コンテ](STORYBOARD.ja.md)
- [60秒ナレーション原稿](NARRATION.ja.md)
- [撮影scenario contract](RECORDING_SCENARIO.md)
- [顔なしカワウソ案内役の仕様](OTTER_GUIDE_SPEC.md)
- [Claude Design handoffと完成prompt](CLAUDE_DESIGN_HANDOFF.ja.md)
- [Claude Designへ貼るpromptだけを開く](CLAUDE_DESIGN_PROMPTS.ja.md)
- [オリジナルカワウソSVG](assets/otter-guide.svg)
- [操作できるlocal concept demo](prototype/index.html)
- [60秒caption animatic](prototype/mv.html)
- [日本語紹介deck](ExecLocus_intro_ja.pptx)
- [Codex側のdesign制作workflow](CODEX_DESIGN_WORKFLOW.ja.md)
- [段階別の制作・公開gate](../DEMO_PLAN.md)
- [日本語1ページ紹介](../ONE_PAGER.ja.md)

現在は絵コンテ、ナレーション、character motion reference、操作できるconcept demo、60秒caption animatic、日本語紹介deckまで完成している。shell-specific resolution、candidate表示、profile、agent evidence、自動匿名化の実装gateは通過した。公開用の実写captureは、専用synthetic環境でWindowsとWSLを別々に収録し、外部prototype検証を通した後に置き換える。
