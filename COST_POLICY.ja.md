# 追加支出0円の開発ポリシー

ExecLocusは、プロジェクトの**追加支出を0円**にすることを既定方針とします。リポジトリ所有者が利用前に明示承認しない限り、新たな請求、従量課金クレジットの消費、有料機能の有効化につながる操作を行いません。

これは開発手順と設定のポリシーです。開発者がすでに負担しているインターネット、端末、OS、既存サブスクリプションの料金まで0円と保証するものではありません。

## 既定の分類

### 費用承認なしで利用可能

- ライセンス条件を守ったうえで、導入・利用に従量料金がないローカルツールと依存関係
- ローカルかつオフラインで完結するビルド、テスト、lint、資料生成
- GitHubの現行条件で無料対象である間の、公開リポジトリ向けstandard GitHub-hosted runner
- 公開資料を読むだけの調査
- 予定する操作で請求が発生しないことを、アカウントとプロジェクト双方の請求画面で確認できるサービス

### 利用前に明示承認が必要

- 従量課金API、hosted AI model、cloud compute、有料runner、有料Marketplace Actionなど、pay-as-you-go請求が可能なサービス
- プロジェクト自動化で使用するAPI key、cloud credential、billing account、usage credit、支払い方法
- trial、promotional credit、有料サブスクリプションに含まれる利用枠
- 無料枠を超える可能性があるartifact、cache、package、release storage
- 現在の料金または課金動作を確認できない製品

無料clientのインストールは、有料backendの利用許可を意味しません。たとえばagent CLI自体は無料でダウンロードできても、model呼び出しで有料subscription枠や従量課金API creditを消費する場合があります。

## 例外承認の記録

例外を利用する前に、秘密情報を含まない非公開の承認記録、または公開Issueへ次を記録します。

1. 実施内容と必要な理由
2. serviceと認証経路
3. subscription枠、credit、pay-as-you-go残高のどれを消費するか
4. 発生し得る追加請求額の上限
5. 端末外へ送信されるdata
6. 承認範囲と有効期限

1回の承認は、後続の呼び出しや別serviceを許可しません。credentialや個人の請求情報はcommitしません。

## CIとリポジトリの制約

- workflowはstandard runnerを使用します。GitHub-hosted larger runnerには例外承認が必要です。
- workflowからhosted AI modelや有料外部APIを呼び出しません。
- workflowへ課金credentialや有料serviceのsecretを要求しません。
- cache、artifact、package、releaseの保存設定を変える場合は費用確認を行います。
- Pull Requestでは、プロジェクトの費用発生可能性を変えるか明記します。

料金または認証方法が不明な場合は、操作前に停止してリポジトリ所有者へ判断を求めます。
