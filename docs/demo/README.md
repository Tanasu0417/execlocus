# ExecLocus demo kit

更新日: 2026-07-30

このdirectoryは、説明用mockupと実測demoを混同せずに制作するための正本である。

- [60秒紹介MVの絵コンテ](STORYBOARD.ja.md)
- [60秒ナレーション原稿](NARRATION.ja.md)
- [撮影scenario contract](RECORDING_SCENARIO.md)
- [顔なしカワウソ案内役の仕様](OTTER_GUIDE_SPEC.md)
- [Claude Design handoffと完成prompt](CLAUDE_DESIGN_HANDOFF.ja.md)
- [Claude Designへ貼るpromptだけを開く](CLAUDE_DESIGN_PROMPTS.ja.md)
- [オリジナルカワウソSVG（陸上）](assets/otter-guide.svg)
- [オリジナルカワウソSVG（泳ぎ）](assets/otter-swim.svg)
- [操作できるlocal concept demo](prototype/index.html)
- [60秒caption animatic](prototype/mv.html)
- [Windows／WSL実機確認手順](../TRY_IT.ja.md)
- [Windows/WSL try-it guide](../TRY_IT.md)
- [日本語紹介deck](ExecLocus_intro_ja.pptx)
- [Codex側のdesign制作workflow](CODEX_DESIGN_WORKFLOW.ja.md)
- [段階別の制作・公開gate](../DEMO_PLAN.md)
- [日本語1ページ紹介](../ONE_PAGER.ja.md)

現在は絵コンテ、ナレーション、動きの仕様、日英切替付きの操作デモ、60秒字幕映像、日本語紹介資料まで完成している。同じ`prototype/index.html`は、静的に開くと合成データのconcept demo、`execlocus gui`から開くと実環境を診断するlocal GUIとして動作する。実診断の起動方法と、local detail／共有用匿名化reportの境界は[実機確認手順](../TRY_IT.ja.md)を参照する。公開用の実写収録は、専用の合成環境でWindowsとWSLを別々に収録し、外部検証を通した後に置き換える。
