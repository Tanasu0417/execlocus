# ExecLocus X post strategy

- Status: ready for problem discovery; product promotion is gated
- Updated: 2026-07-29
- Goal: Windows／WSL利用者の具体的な困りごと、現在の回避策、伝わる言葉を確認する

## Public repository policy

このfileは公開repositoryへ置ける調査方針と投稿templateである。Xのcredential、API token、account recovery情報、回答者handle、個人別の回答、未公開の生集計はcommitしない。具体的な実施日時と未公開集計は運用中に公開版へ追記せず、調査終了後にaggregateと判断根拠だけを公開する。

## Strategy

最初の2週間は、製品linkを拡散するよりも「直近の経験」と「実際の確認手順」を集める。poll、自由回答、message testを1回ずつ行い、同じ文章を繰り返さない。

X Pollは最大4択、各選択肢25文字まで、5分から7日まで設定できる。最初のpollは3日間とする。pollには画像を添付できないため、one-pagerの画像投稿とは分ける。

通常postの280文字以内に収める。現在の本文はA 132文字、B 138文字、C 90文字、D 128文字、E 235文字（改行を含む、URL差し替え前）で検証済み。

## Posting sequence

| Day | Post | 目的 | Link／media | 判定 |
|---:|---|---|---|---|
| 1 | A: recent experience poll | 問題頻度と言葉を知る | なし | vote内訳＋reply内容 |
| 4 | B: workaround question | 実際の手動手順を知る | なし | unique workflow数 |
| 8 | C: `/mnt/c` intent poll | sharing-first需要を分ける | なし | intent別内訳 |
| 12 | D: message comprehension | one-pagerの誤解を検出 | GitHub上のone-pager | 正しい言い換え率 |
| UC-02後 | E: prototype recruitment | 実利用者を募る | real 10秒demo＋Issue | verified case数 |

Day 1–12は認知campaignではなくresearchである。impression、like、repostは補助signalに留め、prototype需要の成功件数へ足さない。

## A — recent experience poll

本文:

```text
Windows＋WSLでCodex／Claude Codeを使う方へ。

「gitやnodeがWindows版かLinux版か」を直近1か月で確認しましたか？

よければ、確認に使ったコマンド名だけ返信で教えてください。個人pathや診断出力は貼らないでください。
```

選択肢:

1. 何度も確認した
2. 1回確認した
3. 困ったが未確認
4. 困っていない

期間: 3日。結果投稿は「最多回答」だけでなく、回答数と選択肢別比率を示す。

## B — workaround question

```text
WSLでコマンドの出所を疑ったとき、どの順番で確認しますか？

例: command -v / type -a / where.exe / Get-Command / PATH / file

実際に使うコマンド名だけ知りたいです。個人pathや診断出力は貼らないでください。
```

集計category: 1 command、2–3 commands、4 commands以上、調べ方が分からない、既存toolで十分。

## C — `/mnt/c` intent poll

本文:

```text
WSLのprojectを`/mnt/c`に置く最大の理由は何ですか？

場所の良し悪しを決める調査ではなく、Windowsアプリとの共有とLinux側の使い方を分けて知りたいです。
```

選択肢:

1. Windowsアプリと共有
2. Explorerから扱いたい
3. WSL側の性能を優先
4. 特に意識していない

## D — message comprehension

```text
開発中のExecLocusを一文で説明します。

「現在のWindows／WSL環境で、どの実行ファイルが選ばれるかと根拠を1画面にまとめるread-only CLI」

この説明から、できること／できないことをどう理解しましたか？ 一言で教えてください。
```

添付link: `https://github.com/Tanasu0417/execlocus/blob/main/docs/ONE_PAGER.ja.md`

このpostは`ONE_PAGER.ja.md`が`main`へmergeされ、linkを未ログインbrowserで確認してから実施する。

次の誤解が1件でも出たらone-pagerを修正する。

- agentが過去に実行したfileを必ず特定できる。
- PATHやWSL設定を自動修復する。
- `/mnt/c`を常にerrorにする。
- 完成済みbinaryを今すぐdownloadできる。

## E — prototype recruitment template

UC-02とprivacy gateを通過するまで投稿しない。

```text
ExecLocusのpre-alpha testerを募集します。

Windows／WSLの現在のcontextで、gitやnodeのselected candidate、alternative、理由をread-onlyで確認するCLIです。

対象: Windows 11＋WSL2
お願い: 生の診断出力は公開replyへ貼らないでください

Demo: <real demo URL>
Test: <release or source instructions>
```

## Reply handling

- 事実確認が必要なreplyへ、公開の生outputを要求しない。
- 有用な回答には個別に礼を述べ、追加質問は1件につき1つまでにする。
- 無関係なthread、trending hashtag、有名accountへの宣伝replyを行わない。
- 同内容の自動投稿、複数accountでの相互boost、like／repost交換を行わない。
- 否定的回答と「既存commandで十分」を削除せず、alternative analysisへ反映する。

## Measurement sheet

個人を特定する一覧は作らず、次のaggregateだけを記録する。

| Field | Example |
|---|---|
| Post ID | A |
| Posted at | YYYY-MM-DD |
| Duration | 3 days |
| Votes / replies | aggregate count |
| Unique workaround categories | 5 |
| Misunderstanding categories | agent history, auto-fix |
| Follow-up decision | revise wording / keep / stop |

Prototype validationではGitHubのstructured field-report formを使い、X replyのhandleと診断結果を結び付けない。

## Stop／continue rules

- Continue: 10件以上の回答で、最近確認した／困ったが未確認が複数存在し、具体的workaroundが得られる。
- Revise: 回答の中心が別問題、またはmessage testで中核能力が誤解される。
- Stop expansion: 既存commandで十分という具体的回答が大半で、prototype比較でもtime-to-conclusionが改善しない。

参考:

- [X Help: About X Polls](https://help.x.com/en/using-x/x-polls)
- [X Help: How to post](https://help.x.com/en/using-x/how-to-post)
- [X Help: Authenticity](https://help.x.com/en/rules-and-policies/authenticity)
