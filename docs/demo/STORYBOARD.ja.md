# ExecLocus 60秒紹介MV 絵コンテ

- Status: production-ready storyboard; screen content marked `Concept` until its gate passes
- Updated: 2026-07-29
- Audience: WindowsとWSLを併用するCodex／Claude Code利用者
- Primary action: GitHubでpre-alphaの目的と制約を理解し、困った経験を共有する

## Story promise

> 同じprojectでも、WindowsとWSLでは選ばれる実行ファイルが違う。ExecLocusは、それぞれの現在の環境と解決結果を根拠付きで並べる。

「agentが過去に実行した対象」は、processまたはinvocation evidenceがある場合だけ別項目として扱う。

## Frames

| # | 時間 | 画面・動き | 画面上の言葉 | ナレーション | 公開gate |
|---:|---:|---|---|---|---|
| 1 | 0–5秒 | 顔なしカワウソsilhouetteが同じprojectを示すnodeへ泳ぎ、WindowsとWSLの両側に`node`を表示 | `同じproject。選ばれるNodeも同じ？` | 同じsourceでも、実行するcontextで選ばれるtoolは変わります。 | `Concept`表示 |
| 2 | 5–12秒 | PowerShellとbashの確認commandを並べ、カワウソが両terminal間の経路を移動 | `確認結果はterminalごとに分かれる` | 今はterminalごとにcommand、PATH、file formatを突き合わせる必要があります。 | synthetic環境のみ |
| 3 | 12–17秒 | 製品名と一文だけを表示。カワウソは画面端で静止 | `選ばれる候補と理由を、1画面へ` | ExecLocusは、現在の環境で何が選ばれるかと、その根拠を一画面へまとめます。 | 過去のagent実行と表現しない |
| 4 | 17–25秒 | 実terminalで`execlocus`を実行。カワウソは結果を隠さず端で待機 | `設定なし。まず1回実行` | 通常実行はread-only。PATHやWSL設定を変更せず、診断内容をuploadしません。 | 実測速度を加工しない |
| 5 | 25–35秒 | WindowsとWSLのselected path／formatを左右に表示し、尾と前脚でevidence経路を案内 | `Same project · Different context` | 同じprojectでも、Windowsではnode.exe、WSLではLinux版Nodeが選ばれています。 | paired UC-02 scenario test完了 |
| 6 | 35–43秒 | current-contextとagent-observedを二段表示。胸の位置markerと字幕panelで発話 | `予測と実行済み証拠を混同しない` | 現在の解決結果と、agentが実際に使った証拠は分けて扱います。証拠がなければ断定しません。 | UC-01 evidence state完了 |
| 7 | 43–51秒 | `/mnt/c/demo/project`と`balanced`を表示し、カワウソがproject nodeへ移動 | `/mnt/cは目的次第。場所だけでerrorにしない` | Windowsアプリとの共有が目的なら、`/mnt/c`は正しい選択にもなります。 | UC-03 profile test完了 |
| 8 | 51–60秒 | GitHub URL、pre-alpha、privacy注意。カワウソはCTA横でmotionを止める | `Windows × WSLの境界を、証拠付きで` | ExecLocusは現在pre-alphaです。WindowsとWSLで困った経験や、確認に使うcommandを教えてください。 | release可能とは表現しない |

## 10秒版への切り出し

Frame 1を2秒、Frame 4を2秒、Frame 5を4秒、Frame 8を2秒に再編集する。説明を詰め込まず、次の4点だけを残す。

1. 同じprojectでもWindowsとWSLの解決結果が違う。
2. 両contextで`execlocus`を実行する。
3. selected path／format／evidenceを比較できる。
4. read-only／local-onlyのpre-alphaである。

## Visual direction

- terminalとevidenceを主役にし、人物、AI robot、stock image、第三者作品のcharacterは使わない。
- 完全オリジナルの顔なしカワウソsilhouetteを案内役として使える。通常は画面面積の15%以内とし、terminal、path、evidenceを隠さない。
- カワウソに目、鼻、口、ひげ、服を付けない。発話は胸の位置marker、通信波、字幕、音声波形で示す。
- motionは泳ぐS字軌道、胴から尾へのfollow-through、滑らかな加減速を使う。`prefers-reduced-motion`版も作る。
- WindowsとWSLの違いはlabelと位置で示し、色だけへ依存しない。
- subtitleは1画面2行以内、1行24全角文字程度を目安にする。
- 16:9をmasterとし、中央の安全領域だけで1:1へ切り出せる構図にする。
- cursor点滅、通知、clock、bookmark、shell historyは映さない。
- mockupには左上へ常時`Concept`、実測にはversionとcommit SHAを記録する。

## Asset list

| Asset | 仕様 | Status |
|---|---|---|
| Master | 1920×1080、60秒以内、30fps | 撮影待ち |
| README demo | 10秒以内、字幕のみ、loopしても意味が切れない | UC-02待ち |
| X landscape | 16:9、字幕付き、30秒 | UC-02／03待ち |
| X square | 1:1、中央crop、30秒 | UC-02／03待ち |
| Still | selected／alternative／reasonが読める1枚 | UC-02待ち |
| Alt text | 結論、候補、境界、privacy状態を文章化 | capture時作成 |
| Otter SVG | 顔なしsilhouette、基本motion、reduced-motion | Concept source作成済み |
| HTML animatic | 8 frame、timeline、REAL FOOTAGE SLOT | Claude Designへhandoff可能 |

実際のfixtureとexpected outputは[撮影scenario contract](RECORDING_SCENARIO.md)に固定する。読み上げは[ナレーション原稿](NARRATION.ja.md)、characterは[カワウソ仕様](OTTER_GUIDE_SPEC.md)、Claude Designへの受け渡しは[handoff資料](CLAUDE_DESIGN_HANDOFF.ja.md)を正本とする。
