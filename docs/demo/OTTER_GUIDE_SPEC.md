# ExecLocus faceless otter guide specification

- Status: original concept; optional presentation layer
- Updated: 2026-07-29
- License: project MIT License

## Role

カワウソは診断を行う主体ではなく、ExecLocusが生成したevidence経路を案内する補助役である。CLI、JSON、headless環境ではcharacterなしで全機能が成立しなければならない。

## Silhouette

顔の情報を使わず、次の外形でカワウソと判別できるようにする。

- 横向き寄りの3/4姿勢。
- 小さく丸い頭と低い首。
- 細長く柔らかい胴体。
- 短い前脚と後脚。
- 根元が太く、先端へ滑らかに細くなる長い尾。
- 頭から尾まで連続するS字の泳ぐline。

目、鼻、口、ひげ、眉、服、neck accessory、既存mascotを想起させる記号は付けない。発話の表現に口パクを使わない。

## Motion states

| State | Loop／duration | Motion |
|---|---:|---|
| Idle | 4秒loop | 上下5px、回転±1.5度、胴体の小さな呼吸、尾が0.2秒遅れて追従 |
| Swim／guide | 1.2–3.6秒 | 直線でなく緩いS字軌道。胴、腰、尾の順にfollow-through |
| Point | 0.8秒 | 前脚を対象へ伸ばし、細いguide lineを表示 |
| Inspect | 2秒loop | 体を少し丸め、胸の位置markerから探索ring |
| Warning | 0.6–1.2秒 | 小さなsquash and stretch。強い点滅やcamera shakeは使わない |
| Success | 1.8秒 | 尾を大きく一度振り、対象nodeへ位置markerを残す |

動画masterは30fpsでも成立させ、可能なら60fpsでmotionを調整する。GUIではdisplay refreshに追従し、固定frame rateを前提にしない。

## Speech without a face

- 胸の位置markerを音節に合わせて弱くpulseさせる。
- silhouette外側へ2–3本の通信波を出す。
- 隣接する字幕panelに発話内容を表示する。
- 小さな音声波形を字幕panel内に表示する。
- 発話中だけoutline glowをわずかに強める。

音声は初期OFFとし、mute状態でも字幕だけで意味を保つ。音声を後から実装する場合もcloud APIを必須にせず、local／OS voiceと差し替え可能なadapterにする。

## Screen constraints

- 通常時は画面面積の15%以内。
- terminal、selected candidate、alternative、reason、公開gateを隠さない。
- Windows／WSLのnode labelと同じ視覚階層へ昇格させない。
- characterの動きだけで診断状態を表現しない。
- 1秒3回を超える点滅を使わない。
- `prefers-reduced-motion`では移動を停止し、状態は字幕とmarkerだけで示す。
- subtitleと背景のcontrast ratioは4.5:1以上を目標とする。

## 60-second placement

| Time | Character action |
|---:|---|
| 0–5秒 | 暗い画面へ泳いで入り、同じprojectを示すnodeで停止 |
| 5–17秒 | Windows／WSLの確認command間を移動し、manual workを示す |
| 17–35秒 | `execlocus`実行中は画面端で待機し、結果を隠さない |
| 35–43秒 | selected pathからevidenceまでを前脚と尾で案内 |
| 43–51秒 | `/mnt/c` nodeへ移動し、目的依存であることを字幕表示 |
| 51–57秒 | privacy／redactionのgateを通信panelで示す |
| 57–60秒 | GitHub CTAの横で静止し、motionを止める |

## Source asset

[`assets/otter-guide.svg`](assets/otter-guide.svg)は、silhouetteと基本motionを確認するためのproject-owned referenceである。完成logoや商標を意味せず、Claude DesignまたはGUI実装時に調整可能とする。
