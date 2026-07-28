# ExecLocus 60秒MV ナレーション原稿

- Status: concept narration; factual claims remain gated by storyboard
- Updated: 2026-07-29
- Voice: 落ち着いた観測者。煽り、攻撃的なhacker表現、過度な断定を避ける

| Time | Narration |
|---:|---|
| 0–5秒 | 同じsourceでも、実行するcontextで選ばれるtoolは変わります。 |
| 5–12秒 | 今はterminalごとにcommand、PATH、file formatを突き合わせる必要があります。 |
| 12–17秒 | ExecLocusは、現在の環境で何が選ばれるかと、その根拠を一画面へまとめます。 |
| 17–25秒 | 通常実行はread-only。PATHやWSL設定を変更せず、診断内容をuploadしません。 |
| 25–35秒 | 同じprojectでも、Windowsではnode.exe、WSLではLinux版Nodeが選ばれています。 |
| 35–43秒 | 現在の解決結果と、agentが実際に使った証拠は分けて扱います。証拠がなければ断定しません。 |
| 43–51秒 | Windowsアプリとの共有が目的なら、スラッシュ・エム・エヌ・ティー・スラッシュ・シーは正しい選択にもなります。 |
| 51–60秒 | ExecLocusは現在pre-alphaです。WindowsとWSLで困った経験や、確認に使うcommandを教えてください。 |

## Character synchronization

カワウソに口は付けない。ナレーション中はsilhouette外側の位置marker、通信波、字幕横の音声波形だけを同期させる。無音版でも同じ字幕を残す。

## Recording notes

- `read-only`、`node.exe`、`pre-alpha`は英語読みでもよいが、全takeで統一する。
- `/mnt/c`は記号を画面へ表示し、音声では「スラッシュ・エム・エヌ・ティー・スラッシュ・シー」と読む。
- 60秒を超える場合はFrame 6の2文目を短縮し、再生速度を不自然に上げない。
- 合成音声を使う場合、利用条件とvoice名を制作memoへ記録する。
