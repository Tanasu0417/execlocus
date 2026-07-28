# ExecLocus デモ／紹介MV制作計画

更新日: 2026-07-28

## 結論

設計は根幹機能の完成前に始め、実測素材の収録は段階的に行う。最終版の紹介MVを最初に作ると、未実装機能を完成済みに見せるか、実装変更のたびに撮り直すことになる。

| 時期 | 作るもの | 公開上の扱い |
|---|---|---|
| 今 | 1ページ紹介、絵コンテ、再現scenario、必要画面一覧 | mockupは必ずConcept／Plannedと表示 |
| UC-02・UC-03完了後 | current-context resolutionの10秒GIF、30秒technical demo | 実fixtureを使った実測だけを表示 |
| UC-01・自動秘匿化完了後 | 60秒hero MV、共有reportのbefore／after | agent-observedとcurrent-contextを画面上で区別 |
| alpha artifact検証後 | installから結果までの90秒walkthrough | download URL、version、checksumを固定 |

## Hero scenario

WSL2上のprojectで、current contextの`node`候補にLinux版とWindows版が存在し、Windows版が先に解決されるscenarioを使う。

デモが証明する内容:

- ExecLocus自身はWSLで動作している。
- current contextではWindows版Nodeが選ばれる。
- Linux版候補も存在するが優先順位で負けた。
- projectは`/mnt/c`にあり、`balanced`または`share-first`なら場所だけでerrorにしない。
- 判定はread-onlyで、変更は利用者が選ぶ。

デモが証明しない内容:

- AI agentが過去にそのNodeを実行したこと。
- `/mnt/c`が常に遅い、危険、または誤りであること。
- すべてのshellで同じresolutionになること。

## 10秒GIF

1. 0–2秒: `WSLなのにWindows Node？`という問題を1行表示。
2. 2–4秒: `execlocus`を実行。
3. 4–8秒: `Current context → node.exe (Windows)`、負けたLinux候補、evidenceを強調。
4. 8–10秒: `Local only · Read only · Evidence-backed`とrepository URL。

音声は付けず、READMEで自動再生しなくても意味が分かるcaptionを焼き込む。点滅を避け、色だけに意味を持たせない。

## 60秒紹介MV

| 時間 | 画面 | ナレーションの役割 |
|---:|---|---|
| 0–7秒 | Windows／WSLの二層と同名command | 問題を提示 |
| 7–17秒 | 手動で複数commandを確認する画面 | 現在の手間を提示 |
| 17–32秒 | `execlocus`の実測実行 | 1commandの価値を提示 |
| 32–43秒 | selected／alternatives／evidence | 結論の理由を提示 |
| 43–51秒 | `/mnt/c` profile判定 | 共有を一律否定しないことを提示 |
| 51–57秒 | redacted report | 共有可能性を提示（実装後のみ） |
| 57–60秒 | GitHub URLとalpha version | 次の行動を1つに限定 |

## 収録ゲート

- scenarioのpositive、non-triggering、missing-evidence testが通る。
- Windows CI、Ubuntu CI、MSRV、Packageが通る。
- terminalとJSONが同じnormalized reportを表す。
- 実測画面に`Concept`、`Planned`等の偽装がない。
- username、machine名、home、absolute personal path、repository private name、tokenが映らない。
- shell history、prompt、window title、notification、browser bookmarkも確認する。
- 公開用はsynthetic user、synthetic project、専用temporary environmentで撮影する。
- 第三者のlogo、音楽、font、screen素材は利用条件を記録する。

## 納品セット

- README向け10秒GIFまたは軽量MP4。
- X／記事向け字幕付き30秒版（16:9と1:1）。
- release向け60秒版（英語字幕、日本語字幕）。
- 同じscenarioの静止画1枚とalt text。
- 撮影command、fixture、version、commit SHAを記録した再現メモ。

需要検証と投稿文は[DEMAND_VALIDATION.md](research/DEMAND_VALIDATION.md)、実装順は[ADOPTION_BLUEPRINT.md](ADOPTION_BLUEPRINT.md)を正本とする。
