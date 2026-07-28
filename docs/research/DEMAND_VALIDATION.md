# ExecLocus 需要検証計画

更新日: 2026-07-29

実際の投稿順、投稿copy、reply運用は[X post strategy](X_POST_STRATEGY.md)を使う。

## 結論

X等での調査は根幹機能と並行して始める。ただし、今確認するのは「困りごとの頻度」「現在の回避策」「理解される表現」であり、未完成品の利用意向を需要の証明にしない。実際の製品価値は、prototypeを使った確認済み事例で別に検証する。

## 検証する仮説

1. Windows／WSL利用者は、agent contextのcommand resolutionを確認するため複数commandを使っている。
2. 困りごとはinstall場所ではなく、選択結果・候補・project境界をまとめて判断できないことにある。
3. `/mnt/c`利用者には、性能優先だけでなくWindowsアプリとの共有を優先する層がいる。
4. 1commandで確認可能な結論が得られれば、同様の不確実性で再利用される。

## 3段階の証拠

| 段階 | 集めるもの | 需要の証明に数えるか |
|---|---|---|
| Problem discovery | X poll、reply、短いinterviewで最近の経験と回避策 | 課題候補。製品需要の証明にはしない |
| Message test | one-pagerを読んで正しく言い換えられるか | 表現の理解度。利用需要にはしない |
| Prototype validation | 実環境でverified conclusion、manual comparison、reuse | 確認済みで役に立った事例だけ数える |

## 最小記録項目

- unique collaborator数とunique environment数を別々に数える。
- Windows native、WSL `/mnt/c`、WSL nativeの区分。
- 直近の具体的な困りごとと、それまでの手動手順。
- ExecLocusの結論がverified、false positive、unresolvedのどれか。
- 手動のtime-to-conclusionとExecLocusのtime-to-conclusion。
- 結論が設定確認、修正、現状維持、Issue作成等のdecisionにつながったか。
- 「また使いたい」というintentと、後日実際に使ったretentionを別項目にする。
- raw output、username、machine名、personal path、private project、credentialは収集しない。

## Xで今行う調査

### Poll A: 最近の確認経験

投稿例:

```text
Windows＋WSLでCodex/Claude Codeを使う方へ。
「git/nodeがWindows版かLinux版か」を直近1か月で確認しましたか？
よければ、使った確認方法も返信で教えてください（pathや出力は貼らないでください）。
```

選択肢:

- 何度も確認した
- 1回確認した
- 困ったが未確認
- 困っていない

### Question B: 現在の回避策

```text
WSLでcommandの出所を疑ったとき、最初に何を確認しますか？
which / type -a / where.exe / Get-Command / PATH確認など、実際の順番を知りたいです。
個人pathや診断出力は貼らず、command名だけで教えてください。
```

### Message test C

```text
ExecLocus（開発中）は、現在のWindows/WSL contextで、どの実行ファイルが選ばれるかと根拠を1画面にまとめるread-only CLIです。
この説明から「できること／できないこと」をどう理解したか、一言で教えてください。
```

`実際にagentが過去に実行したfileを必ず特定する`と解釈された場合は、messageを修正する。

## 運用ルール

- pollは方向性を見る手段で、母集団を代表する統計として扱わない。
- 同じ文章の反復投稿、無関係なthreadへの宣伝reply、回答依頼の大量送信をしない。
- 1回のpoll後に、任意のqualitative replyまたは短いinterviewで理由を確認する。
- 実装前のscreenは`Concept`または`Planned`と明示し、real outputとして投稿しない。
- public replyに診断出力を求めない。必要な実機検証は、automatic redaction完成後の専用formへ分離する。
- 否定的回答、`困っていない`、既存commandで十分という回答も残す。

X Pollsは最大4選択肢、各選択肢25文字までで、投票期間を5分から7日まで設定できる。最初の調査は3日間を標準とし、replyの文脈も合わせて読む。Xのauthenticity policyに従い、重複投稿やunsolicitedな大量接触を避ける。

参考:

- [X Help: How to create X polls](https://help.x.com/en/using-x/x-polls)
- [X Rules](https://help.x.com/en/rules-and-policies/x-rules)
- [X Authenticity policy](https://help.x.com/en/rules-and-policies/authenticity)

## 最初の判定ゲート

10人以上・10環境以上のprototype validationのうち、3件以上で次をすべて満たしたらGate Bの価値仮説を通過とする。

1. 結論がindependent checkでverified。
2. 結論またはdecisionが利用者にとって有用。
3. 従来手順と比べて同等以下のtime-to-conclusion、または従来は調査不能。
4. privacy incidentがない。

Xの投票数、like、impression、GitHub starは認知signalとして記録してよいが、この3件を代替しない。
