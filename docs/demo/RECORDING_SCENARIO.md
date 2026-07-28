# ExecLocus recording scenario contract

- Scenario ID: `DEMO-UC02-PAIRED-01`
- Status: approved design; not yet recordable
- Updated: 2026-07-29

## User story

同じWindows filesystem上のprojectをWindows PowerShellとWSL2 bashから開き、それぞれのcurrent contextで`node`がどの実行ファイルへ解決されるかを比較する。

このscenarioは2つのcurrent-context resolutionを示す。AI agentが過去にどちらかの`node`を実行した証拠として使わない。

## Synthetic paired environment

| Input | Windows side | WSL side |
|---|---|---|
| Runtime | Windows 11 x86_64 | WSL2 / Ubuntu 24.04 |
| Shell contract | PowerShell | bash |
| Same project | `C:\demo\execlocus-sample` | `/mnt/c/demo/execlocus-sample` |
| Profile | `balanced` | `balanced` |
| Requested command | `node` | `node` |
| Selected candidate | `C:\Tools\node\node.exe` | `/usr/bin/node` |
| Format／origin | PE／Windows | ELF／Linux |
| Personal identity | none; synthetic `demo` paths only | none; synthetic `demo` paths only |

PATHは各撮影shell processだけへ固定し、system PATHを恒久変更しない。撮影終了時にprocessごと破棄する。

## Expected comparison

```text
WINDOWS CONTEXT                         WSL CONTEXT
Runtime   Windows 11                    Runtime   WSL2 / Ubuntu 24.04
Project   C:\demo\execlocus-sample      Project   /mnt/c/demo/execlocus-sample
Node      C:\Tools\node\node.exe · PE   Node      /usr/bin/node · ELF

Same source tree · different execution context
```

これは編集上のcomparisonであり、1つのExecLocus processが両OSを同時観測するという意味ではない。Windows版とWSL版を別々に実行し、同じschemaの結果を左右へ配置する。

## Independent comparison

### Windows PowerShell

```powershell
Get-Command node -All
where.exe node
execlocus report --format json
```

### WSL bash

```console
command -v node
type -a node
file /usr/bin/node
execlocus report --format json
```

実機確認では、bashの`command -v node`は`node.exe`を自動補完しなかった。したがって、`/mnt/c/.../node.exe`と`/usr/bin/node`を同じbash commandの競合候補として演出しない。

## Required tests before recording

1. Injectable resolverでWindowsとWSLのPATH、filesystem、file headerを別fixtureとして与えられる。
2. PowerShell contractとbash contractが、それぞれのselected candidateを決定する。
3. Windows terminal／JSONとWSL terminal／JSONが同じschema fieldsを使う。
4. paired golden testがproject identityの対応とNode originの違いを表す。
5. control characterがterminalへ生で出力されない。
6. synthetic path以外がgolden outputへ混入しない。

## Follow-up scenario — Windows-mounted shim

`DEMO-UC02-SHIM-01`では、WSL bashがWindows filesystem上のextensionless `npm` shimを解決するcaseを扱う。script location、shebang interpreter、downstream `node.exe`を別々に表現できるまでは撮影しない。

このscenarioはcross-layer chainを説明できるが、最初のdemoより実装と説明が複雑なため後回しにする。

## Control scenarios

### `DEMO-UC03-01` — intentional `/mnt/c`

projectは`/mnt/c`、Nodeとbuild cacheはWSL-native、profileは`share-first`または`balanced`。filesystem locationだけを理由にwarningを出さない。

### `DEMO-MISSING-01` — no command evidence

Node candidateが存在しない。`unavailable`を表示し、成功や安全を推測しない。

## Capture checklist

- clean VMまたは専用local accountを使う。
- terminal title、prompt、history、notification、clockをsynthetic化または非表示にする。
- username、machine名、home、private repository、token、credentialを映さない。
- application version、commit SHA、OS／distribution versionを撮影memoへ残す。
- speed-up、cut、overlayを使った箇所を編集memoへ残す。
- WindowsとWSLのJSON fixtureが期待値と一致するまで公開しない。
