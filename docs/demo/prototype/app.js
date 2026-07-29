const translations = {
  ja: {
    "document.title": "ExecLocus",
    "brand.subtitle": "Windows／WSL 実行環境チェッカー",
    "status.local": "このPC内だけで診断",
    "nav.label": "使い方",
    "nav.guide": "はじめに",
    "nav.inspect": "診断結果",
    "nav.compare": "Win／WSL差分",
    "nav.explain": "対応方法",
    "nav.share": "安全に共有",
    "controls.targetLabel": "診断対象",
    "controls.targetValue": "このアプリを起動したプロジェクト",
    "controls.targetHint": "設定やファイルは変更しません",
    "action.run": "Windows側を診断",
    "action.pair": "WindowsとWSLを比較",
    "action.pairNow": "WindowsとWSLを今すぐ比較",
    "action.singleNow": "まずWindows側だけ見る",
    "profile.label": "重視する使い方",
    "profile.question": "あなたは何を優先しますか？",
    "profile.shareName": "Windows共有優先",
    "profile.balancedName": "迷ったらこれ",
    "profile.linuxName": "WSL性能優先",
    "profile.meaning": "この選択の意味",
    "state.ready": "準備完了",
    "state.stale": "基準を変更しました · 再診断してください",
    "state.running": "読み取り専用で診断中…",
    "state.complete": "診断完了 · 外部送信なし",
    "state.error": "診断できませんでした",
    "guide.eyebrow": "WSL2の実行環境を見える化",
    "guide.title": "「どれが実行される？」を、実行前に見抜く。",
    "guide.lead": "同じプロジェクトでも、WindowsとWSLではGit・Node・Codex・Claude Codeの選択が変わります。ExecLocusは、その違いと影響、直し方を1画面にまとめます。",
    "guide.value1Title": "実行場所が分かる",
    "guide.value1Body": "Windows、WSL、ディストリビューション、シェル、プロジェクト境界を判定します。",
    "guide.value2Title": "選ばれるツールが分かる",
    "guide.value2Body": "Git・Node・npm・Codex・Claude Codeの候補と、実際の優先順位を確認します。",
    "guide.value3Title": "次の行動が分かる",
    "guide.value3Body": "不一致の影響、修正候補、確認コマンド、再診断手順を表示します。",
    "guide.problemEyebrow": "防げる問題",
    "guide.problemTitle": "動くけれど、思っていた環境と違う。",
    "guide.problem1": "WSLで作業しているのにWindows版Nodeが選ばれる",
    "guide.problem2": "依存関係やキャッシュが意図しない側へ作られる",
    "guide.problem3": "人によって再現結果が違い、原因調査に時間がかかる",
    "guide.trustTitle": "安全な診断",
    "guide.trustBody": "読み取り専用・外部送信なし・有料APIなし。診断結果はこのPC内だけに表示します。",
    "inspect.emptyTitle": "まだ診断していません",
    "inspect.emptyBody": "「Windows側を診断」または「WindowsとWSLを比較」を押してください。",
    "inspect.resultEyebrow": "今回わかったこと",
    "inspect.verdictOk": "重大な実行環境の不一致は見つかりませんでした。",
    "inspect.verdictWarning": "意図と異なる可能性がある選択を確認してください。",
    "inspect.verdictIncomplete": "選択を確定できていないツールがあります。",
    "inspect.verdictError": "実行前に解消したい問題が見つかりました。",
    "inspect.benefitOk": "現在の実行場所とツールの選択根拠を記録できました。Windows／WSL比較で、両側の差も確認できます。",
    "inspect.benefitWarning": "選ばれた実行ファイル、起こり得る影響、確認コマンドを下にまとめています。",
    "inspect.benefitIncomplete": "候補は見つかりましたが、現在のシェルで実際に選ばれるものを確定できていません。詳細の確認コマンドを実行してください。",
    "inspect.toolsEyebrow": "実行ファイルの選択",
    "inspect.toolsTitle": "この環境で使われる5ツール",
    "inspect.toolsHint": "行を開くと候補と確認方法を表示",
    "metric.error": "要修正",
    "metric.warning": "要確認",
    "metric.selected": "選択済み",
    "field.runtime": "実行環境",
    "field.shell": "シェル",
    "field.project": "プロジェクト境界",
    "field.agent": "AIエージェント",
    "field.selected": "選択結果",
    "field.kind": "コマンドの種類",
    "field.candidates": "見つかった候補",
    "field.why": "この候補が選ばれる理由",
    "field.verify": "自分で確認するコマンド",
    "field.unavailable": "確認できません",
    "field.none": "候補なし",
    "state.not_found": "見つかりません",
    "state.candidates_unconfirmed": "候補あり・選択未確定",
    "state.selected": "選択済み",
    "state.probe_failed": "調査失敗",
    "pair.eyebrow": "WINDOWS ↔ WSL",
    "pair.title": "同じプロジェクトを、両側から比較。",
    "pair.body": "異なる項目を先に表示し、環境差による事故の候補を短時間で絞ります。",
    "pair.refresh": "再比較",
    "pair.emptyTitle": "比較結果はまだありません",
    "pair.emptyBody": "WindowsアプリからWSL補助診断を呼び出し、同じ形式で差分を作ります。",
    "pair.availableTitle": "WindowsとWSLの両方を診断できました",
    "pair.unavailableTitle": "WSL側の診断準備が必要です",
    "pair.setup": "WSLのExecLocusフォルダで、この無料セットアップを1回実行してください。",
    "pair.tableEyebrow": "差分一覧",
    "pair.tableTitle": "Windows／WSLの選択結果",
    "pair.showAll": "同じ項目も表示",
    "pair.runtimeTitle": "実行境界",
    "pair.runtimeOk": "WindowsとWSLを別の実行環境として観測しました。",
    "pair.projectTitle": "プロジェクトの見え方",
    "pair.projectBody": "同じ起動場所を両側のパス規則で確認します。",
    "pair.toolTitle": "注意が必要なツール",
    "pair.toolOk": "WSL側でWindows由来の実行ファイルは選ばれていません。",
    "pair.toolWarn": "WSL側でWindows由来の実行ファイルが選ばれています。",
    "pair.noDifference": "重要な差分なし",
    "pair.different": "差分あり",
    "pair.windows": "Windows",
    "pair.wsl": "WSL",
    "explain.eyebrow": "影響と対応",
    "explain.title": "検出した理由と、次にすること。",
    "explain.emptyTitle": "診断後に対応方法を表示します",
    "explain.emptyBody": "診断結果から項目を選ぶと、影響と確認手順をここに表示します。",
    "explain.emptyImpact": "環境差がない場合は、問題なしと判断できる根拠を表示します。",
    "explain.detected": "何を検出したか",
    "explain.impact": "起こり得る影響",
    "explain.action": "推奨する対応",
    "explain.verify": "確認・再診断",
    "explain.evidence": "判断に使った根拠",
    "explain.noFindings": "要対応の項目はありません",
    "explain.noFindingsBody": "現在の診断基準では警告・エラーはありません。Windows／WSL比較で両側の違いも確認してください。",
    "explain.unconfirmedTitle": "ツール選択の確認が必要です",
    "explain.unconfirmedImpact": "選択を断定しないまま作業すると、想定と異なるOS側の実行ファイルを使う可能性があります。",
    "explain.unconfirmedAction": "診断結果で該当ツールの行を開き、候補と選択理由を確認してください。",
    "explain.unconfirmedVerify": "同じシェルで確認コマンドを実行してから、もう一度診断してください。",
    "share.eyebrow": "安全に共有",
    "share.title": "人には読みやすく、個人情報は伏せる。",
    "share.body": "画面は読みやすいHTMLで表示し、貼り付け用にはMarkdownをコピーします。",
    "share.badge": "個人情報を置換済み",
    "share.whatTitle": "「自動匿名化」とは？",
    "share.whatBody": "ユーザー名、PC名、ホームディレクトリ、個人の絶対パスを、共有画面を作る前に安全な表記へ置き換える機能です。",
    "share.username": "ユーザー名",
    "share.machine": "PC名",
    "share.home": "ホーム",
    "share.path": "絶対パス",
    "share.scope": "対象はこの共有画面とコピーするMarkdownです。未加工のJSONやターミナル出力をそのまま公開しないでください。",
    "share.previewTitle": "共有プレビュー",
    "share.previewBody": "現在の診断結果をHTMLで要約",
    "share.copy": "Markdownをコピー",
    "share.emptyTitle": "共有レポートはまだありません",
    "share.emptyBody": "診断後に、匿名化した結果だけをここへ表示します。",
    "share.ready": "コピー前にプレビューを確認できます。",
    "share.copied": "匿名化済みMarkdownをコピーしました。外部送信はしていません。",
    "share.failed": "クリップボードへコピーできませんでした。画面表示は維持されます。",
    "share.reportTitle": "ExecLocus 診断レポート",
    "share.pairedTitle": "ExecLocus Windows／WSL比較レポート",
    "share.safeNote": "共有用に識別情報を置換済み",
    "share.environment": "実行環境",
    "share.findings": "検出結果",
    "share.noFindings": "共有対象の警告・エラーはありません。",
    "share.unresolvedTitle": "選択の確認が必要",
    "share.unresolvedBody": "候補はありますが、現在のシェルで選ばれる実行ファイルを確定できていません。",
    "footer.local": "ローカル診断 · 読み取り専用 · 外部送信なし",
    "language.label": "Switch to English"
  },
  en: {
    "document.title": "ExecLocus",
    "brand.subtitle": "Windows / WSL execution checker",
    "status.local": "diagnostics stay on this PC",
    "nav.label": "WORKFLOW",
    "nav.guide": "Start here",
    "nav.inspect": "Results",
    "nav.compare": "Win / WSL diff",
    "nav.explain": "Actions",
    "nav.share": "Share safely",
    "controls.targetLabel": "TARGET",
    "controls.targetValue": "Project used to launch this app",
    "controls.targetHint": "No settings or files will be changed",
    "action.run": "Inspect Windows",
    "action.pair": "Compare Windows and WSL",
    "action.pairNow": "Compare Windows and WSL now",
    "action.singleNow": "Inspect Windows first",
    "profile.label": "WORKFLOW PRIORITY",
    "profile.question": "What matters most to you?",
    "profile.shareName": "Windows sharing",
    "profile.balancedName": "Not sure",
    "profile.linuxName": "WSL performance",
    "profile.meaning": "WHAT THIS MEANS",
    "state.ready": "ready",
    "state.stale": "profile changed · run again",
    "state.running": "collecting read-only evidence…",
    "state.complete": "complete · nothing uploaded",
    "state.error": "diagnostic failed",
    "guide.eyebrow": "EXECUTION CLARITY FOR WSL2",
    "guide.title": "Know what will run before it runs.",
    "guide.lead": "The same project can select different Git, Node, Codex, and Claude Code executables in Windows and WSL. ExecLocus puts the difference, impact, and next action on one screen.",
    "guide.value1Title": "Know where it runs",
    "guide.value1Body": "Identify Windows, WSL, distribution, shell, and project boundary.",
    "guide.value2Title": "Know which tool wins",
    "guide.value2Body": "Inspect candidates and effective priority for Git, Node, npm, Codex, and Claude Code.",
    "guide.value3Title": "Know what to do next",
    "guide.value3Body": "See impact, suggested fixes, verification commands, and rerun steps.",
    "guide.problemEyebrow": "PROBLEMS IT PREVENTS",
    "guide.problemTitle": "It runs, but not where you thought.",
    "guide.problem1": "WSL work unexpectedly selects Windows Node",
    "guide.problem2": "Dependencies or caches land on the unintended side",
    "guide.problem3": "Different machines reproduce different behavior",
    "guide.trustTitle": "Safe diagnostics",
    "guide.trustBody": "Read only, no upload, and no paid API. Results stay on this PC.",
    "inspect.emptyTitle": "No diagnostic yet",
    "inspect.emptyBody": "Select Inspect Windows or Compare Windows and WSL.",
    "inspect.resultEyebrow": "WHAT WE LEARNED",
    "inspect.verdictOk": "No critical execution mismatch was found.",
    "inspect.verdictWarning": "Review selections that may not match your intent.",
    "inspect.verdictIncomplete": "Some tool selections could not be confirmed.",
    "inspect.verdictError": "Resolve these issues before relying on the environment.",
    "inspect.benefitOk": "The execution context and tool-selection evidence are recorded. Compare Windows and WSL to check both sides.",
    "inspect.benefitWarning": "Selected executables, possible impact, and verification commands are summarized below.",
    "inspect.benefitIncomplete": "Candidates were found, but the executable selected by the current shell is not yet confirmed. Run the verification command in each detail row.",
    "inspect.toolsEyebrow": "EXECUTABLE SELECTION",
    "inspect.toolsTitle": "Five tools used in this environment",
    "inspect.toolsHint": "Open a row for candidates and verification",
    "metric.error": "Fix",
    "metric.warning": "Review",
    "metric.selected": "Selected",
    "field.runtime": "Runtime",
    "field.shell": "Shell",
    "field.project": "Project boundary",
    "field.agent": "AI agent",
    "field.selected": "Selected",
    "field.kind": "Command kind",
    "field.candidates": "Candidates found",
    "field.why": "Why this candidate wins",
    "field.verify": "Verification command",
    "field.unavailable": "Unavailable",
    "field.none": "No candidates",
    "state.not_found": "Not found",
    "state.candidates_unconfirmed": "Candidates found · unconfirmed",
    "state.selected": "Selected",
    "state.probe_failed": "Probe failed",
    "pair.eyebrow": "WINDOWS ↔ WSL",
    "pair.title": "Observe the same project from both sides.",
    "pair.body": "Differences appear first so environment-boundary risks take less time to find.",
    "pair.refresh": "Compare again",
    "pair.emptyTitle": "No comparison yet",
    "pair.emptyBody": "The Windows app calls the local WSL companion and produces one normalized diff.",
    "pair.availableTitle": "Windows and WSL were both observed",
    "pair.unavailableTitle": "The WSL-side diagnostic needs setup",
    "pair.setup": "Run this free one-time setup in the ExecLocus folder inside WSL.",
    "pair.tableEyebrow": "DIFFERENCES",
    "pair.tableTitle": "Windows / WSL selections",
    "pair.showAll": "Show matching rows too",
    "pair.runtimeTitle": "Runtime boundary",
    "pair.runtimeOk": "Windows and WSL were observed as separate execution contexts.",
    "pair.projectTitle": "Project view",
    "pair.projectBody": "The same launch directory is inspected through each side's path rules.",
    "pair.toolTitle": "Tools needing attention",
    "pair.toolOk": "No Windows-origin executable was selected inside WSL.",
    "pair.toolWarn": "A Windows-origin executable was selected inside WSL.",
    "pair.noDifference": "No important difference",
    "pair.different": "Different",
    "pair.windows": "Windows",
    "pair.wsl": "WSL",
    "explain.eyebrow": "IMPACT AND ACTION",
    "explain.title": "Why it matters and what to do next.",
    "explain.emptyTitle": "Actions appear after diagnosis",
    "explain.emptyBody": "Select a result to see impact and verification steps.",
    "explain.emptyImpact": "When no mismatch exists, the evidence supporting that conclusion appears here.",
    "explain.detected": "What was detected",
    "explain.impact": "Possible impact",
    "explain.action": "Suggested action",
    "explain.verify": "Verify and rerun",
    "explain.evidence": "EVIDENCE USED",
    "explain.noFindings": "No action required",
    "explain.noFindingsBody": "No warning or error exists for the selected profile. Compare Windows and WSL to verify both sides.",
    "explain.unconfirmedTitle": "Tool selection needs confirmation",
    "explain.unconfirmedImpact": "Continuing without a confirmed selection may use an executable from a different OS layer than intended.",
    "explain.unconfirmedAction": "Open the affected tool row in Results and review its candidates and selection reason.",
    "explain.unconfirmedVerify": "Run the verification command in the same shell, then diagnose again.",
    "share.eyebrow": "SHARE SAFELY",
    "share.title": "Readable for people, redacted for privacy.",
    "share.body": "The screen uses readable HTML; the copy button produces Markdown.",
    "share.badge": "identity replaced",
    "share.whatTitle": "What does automatic redaction mean?",
    "share.whatBody": "Username, machine name, home directory, and personal absolute paths are replaced before the share view is created.",
    "share.username": "Username",
    "share.machine": "Machine",
    "share.home": "Home",
    "share.path": "Absolute path",
    "share.scope": "This applies to the share screen and copied Markdown. Never publish raw JSON or terminal output unchanged.",
    "share.previewTitle": "Share preview",
    "share.previewBody": "Readable HTML summary of the current diagnosis",
    "share.copy": "Copy Markdown",
    "share.emptyTitle": "No shareable report yet",
    "share.emptyBody": "A redacted result will appear here after diagnosis.",
    "share.ready": "Review the preview before copying.",
    "share.copied": "Redacted Markdown copied. Nothing was uploaded.",
    "share.failed": "Clipboard unavailable. The screen remains visible.",
    "share.reportTitle": "ExecLocus diagnostic report",
    "share.pairedTitle": "ExecLocus Windows / WSL comparison",
    "share.safeNote": "Identity replaced for sharing",
    "share.environment": "Environment",
    "share.findings": "Findings",
    "share.noFindings": "No warning or error is included in this report.",
    "share.unresolvedTitle": "Selection needs confirmation",
    "share.unresolvedBody": "Candidates exist, but the executable selected by the current shell has not been confirmed.",
    "footer.local": "local diagnostic · read only · no upload",
    "language.label": "日本語へ切り替え"
  }
};

const profileCopy = {
  "share-first": {
    ja: { title: "Windowsから同じファイルを開きやすくする", body: "エクスプローラー、Windows側エディタ、Coworkなどとの共有を重視し、/mnt/c上の配置を意図したものとして評価します。", use: "おすすめ: WindowsとWSLの両方から同じソースを触る人" },
    en: { title: "Keep the same files easy to open from Windows", body: "Treat /mnt/c placement as intentional when Explorer, Windows editors, or Cowork interoperability matters most.", use: "Best for: people editing the same source from Windows and WSL" }
  },
  balanced: {
    ja: { title: "目的を決めつけず、明確な不一致を探す", body: "共有性とWSL側の動作特性を中立に評価します。迷った場合はこの基準から始めてください。", use: "おすすめ: 初回診断、または配置方針がまだ決まっていない人" },
    en: { title: "Find clear mismatches without assuming intent", body: "Evaluate Windows sharing and WSL behavior neutrally. Start here when you are unsure.", use: "Best for: first-time diagnosis or an undecided layout" }
  },
  "linux-first": {
    ja: { title: "WSLネイティブの性能とLinux動作を優先する", body: "/home配下のプロジェクトとLinux版ツールを期待し、WindowsマウントやWindows実行ファイルの混入を強く知らせます。", use: "おすすめ: Linux本番環境との一致、ビルド性能、ファイル監視を重視する人" },
    en: { title: "Prioritize WSL-native performance and Linux behavior", body: "Expect projects under /home and Linux-native tools; emphasize Windows mounts or Windows executables crossing into WSL.", use: "Best for: Linux parity, build performance, and file-watching behavior" }
  }
};

const impactCopy = {
  ja: {
    ENV001: "ターミナルとAIエージェントが別のOS層で動くと、同じコマンド名でも異なる設定・パス・実行ファイルが使われる可能性があります。",
    ENV002: "WSLからWindows版ツールを実行すると、依存関係、改行、パス表記、子プロセスの配置がLinux版と異なる可能性があります。",
    ENV003: "Windows版とLinux版のAIエージェントが共存すると、起動方法によって設定や履歴の保存先が変わる可能性があります。",
    ENV004: "エージェント本体と状態保存先がOS境界をまたぐと、設定の重複や意図しない共有が起きる可能性があります。",
    FS001: "Windows共有には便利ですが、Linuxネイティブ配置と比べて大量ファイルの処理やファイル監視の挙動が変わる場合があります。",
    FS002: "WSLネイティブ配置はLinux処理に適しますが、Windowsアプリから直接開く運用には追加の手順が必要です。",
    PATH001: "意図しないOS版が優先されると、パッケージの配置、設定ファイル、実行結果が別の開発者と一致しない可能性があります。",
    GIT001: "GitのOS層がプロジェクト配置と合わないと、認証情報、改行設定、フックの動作が想定と異なる可能性があります。",
    TOOL001: "Nodeとnpmの由来が分かれると、グローバルパッケージや依存関係が別の場所へ作られる可能性があります。"
  },
  en: {
    ENV001: "When the terminal and agent run in different OS layers, the same command name can use different settings, paths, and executables.",
    ENV002: "Running a Windows tool from WSL can change dependency placement, path syntax, line endings, and child processes.",
    ENV003: "When Windows and Linux agent installs coexist, launch method can change settings and state locations.",
    ENV004: "Crossing the OS boundary for agent state can duplicate settings or share data unintentionally.",
    FS001: "Windows sharing is convenient, but heavy file operations and file watching can differ from WSL-native storage.",
    FS002: "WSL-native storage suits Linux work but can need extra steps for direct Windows-app access.",
    PATH001: "An unintended OS version can place packages and settings elsewhere and produce results other developers cannot reproduce.",
    GIT001: "A Git runtime that does not match project placement can change credentials, line endings, and hooks.",
    TOOL001: "When Node and npm come from different layers, globals and dependencies can land in different places."
  }
};

const params = new URLSearchParams(location.search);
const liveMode = params.get("mode") === "live";
let language = params.get("lang") === "en" ? "en" : "ja";
let selectedProfile = ["share-first", "balanced", "linux-first"].includes(params.get("profile")) ? params.get("profile") : "balanced";
let runPhase = "ready";
let livePayload = null;
let pairedPayload = null;
let selectedFindingIndex = 0;
const stage = document.querySelector(".stage");
document.body.classList.toggle("live-mode", liveMode);

function t(key) { return translations[language][key] ?? key; }
function text(parent, tag, value, className) {
  const element = document.createElement(tag);
  if (className) element.className = className;
  element.textContent = value;
  parent.appendChild(element);
  return element;
}
function valueOrUnavailable(value) { return value ?? t("field.unavailable"); }
function runtimeLabel(value) {
  const labels = {
    windows_native: { ja: "Windowsネイティブ", en: "Windows Native" },
    wsl: { ja: "WSL2", en: "WSL2" },
    linux_native: { ja: "Linuxネイティブ", en: "Linux Native" },
    unknown: { ja: "不明", en: "Unknown" }
  };
  return labels[value]?.[language] ?? valueOrUnavailable(value);
}
function pathLabel(value) {
  const labels = {
    windows_native: { ja: "Windows側", en: "Windows native" },
    windows_mounted: { ja: "Windows共有（/mnt）", en: "Windows mounted (/mnt)" },
    wsl_native: { ja: "WSL側（/home等）", en: "WSL native (/home etc.)" },
    wsl_unc: { ja: "WSL共有（UNC）", en: "WSL share (UNC)" },
    linux_native: { ja: "Linux側", en: "Linux native" },
    unknown: { ja: "不明", en: "Unknown" }
  };
  return labels[value]?.[language] ?? valueOrUnavailable(value);
}
function stateLabel(value) { return t(`state.${value}`); }
function severityLabel(value) {
  const labels = { error: { ja: "要修正", en: "Fix" }, warning: { ja: "要確認", en: "Review" }, info: { ja: "参考情報", en: "Info" } };
  return labels[value]?.[language] ?? value;
}
function agentLabel(value) { return value === "codex" ? "Codex" : value === "claude_code" ? "Claude Code" : t("field.unavailable"); }
function roleLabel(value) { return value === "claude" ? "Claude Code" : value === "codex" ? "Codex" : value === "git" ? "Git" : value === "node" ? "Node" : value === "npm" ? "npm" : value; }
function originLabel(value) { return value === "windows" ? "Windows" : value === "linux" ? "Linux" : value === "script" ? (language === "ja" ? "スクリプト" : "Script") : valueOrUnavailable(value); }
function formatLabel(value) { return value === "pe" ? "PE" : value === "elf" ? "ELF" : value === "script" ? (language === "ja" ? "スクリプト" : "script") : valueOrUnavailable(value); }
function selectedValue(executable) { return executable?.selected?.path ?? executable?.selected_binding ?? "—"; }
function selectedDescriptor(executable) {
  if (!executable) return "—";
  const selected = executable.selected;
  if (!selected) return selectedValue(executable);
  return `${selected.path} · ${originLabel(selected.origin)} / ${formatLabel(selected.format)}`;
}

function updateRunState() {
  const element = document.querySelector("#run-state");
  element.textContent = t(`state.${runPhase}`);
  element.className = `run-state ${runPhase}`;
}
function selectView(name) {
  document.querySelectorAll(".rail-step").forEach((button) => button.classList.toggle("active", button.dataset.view === name));
  document.querySelectorAll(".view").forEach((panel) => panel.classList.toggle("active", panel.dataset.panel === name));
}
function updateProfileHelp() {
  const copy = profileCopy[selectedProfile][language];
  document.querySelector("#profile-help-title").textContent = copy.title;
  document.querySelector("#profile-help-body").textContent = copy.body;
  document.querySelector("#profile-help-use").textContent = copy.use;
  document.querySelectorAll("[data-profile]").forEach((button) => {
    const active = button.dataset.profile === selectedProfile;
    button.classList.toggle("active", active);
    button.setAttribute("aria-checked", String(active));
  });
}
function resetResults() {
  livePayload = null;
  pairedPayload = null;
  document.querySelector("#inspect-empty").hidden = false;
  document.querySelector("#live-overview").hidden = true;
  document.querySelector("#pair-empty").hidden = false;
  document.querySelector("#pair-results").hidden = true;
  runPhase = "stale";
  updateRunState();
}
function selectProfile(name) {
  if (selectedProfile !== name && (livePayload || pairedPayload)) resetResults();
  selectedProfile = name;
  const url = new URL(location.href);
  url.searchParams.set("profile", name);
  history.replaceState(null, "", url);
  updateProfileHelp();
}

function findingCounts(report) {
  return report.findings.reduce((counts, finding) => {
    counts[finding.severity] = (counts[finding.severity] ?? 0) + 1;
    return counts;
  }, { error: 0, warning: 0, info: 0 });
}
function unresolvedToolCount(report) {
  return report.executables.filter((item) => ["candidates_unconfirmed", "probe_failed"].includes(item.selection_state)).length;
}
function renderMetrics(report) {
  const counts = findingCounts(report);
  const unresolved = unresolvedToolCount(report);
  const selected = report.executables.filter((item) => item.selection_state === "selected").length;
  const container = document.querySelector("#finding-metrics");
  container.replaceChildren();
  [[counts.error, t("metric.error"), "error"], [counts.warning + unresolved, t("metric.warning"), "warning"], [`${selected}/${report.executables.length}`, t("metric.selected"), "ok"]].forEach(([count, label, kind]) => {
    const card = document.createElement("div");
    card.className = `metric ${kind}`;
    text(card, "strong", String(count));
    text(card, "span", label);
    container.appendChild(card);
  });
}
function renderContext(report) {
  const container = document.querySelector("#live-summary");
  container.replaceChildren();
  const entries = [
    [t("field.runtime"), `${runtimeLabel(report.runtime.kind)}${report.runtime.distribution ? ` · ${report.runtime.distribution}` : ""}`],
    [t("field.shell"), valueOrUnavailable(report.runtime.shell)],
    [t("field.project"), pathLabel(report.project.class)],
    [t("field.agent"), `${agentLabel(report.agent.product)} · ${runtimeLabel(report.agent.runtime)}`]
  ];
  entries.forEach(([label, value]) => {
    const card = document.createElement("article");
    card.className = "context-card";
    text(card, "span", label);
    const strong = text(card, "strong", value);
    strong.title = value;
    container.appendChild(card);
  });
}
function renderToolchain(report) {
  const container = document.querySelector("#live-candidates");
  container.replaceChildren();
  report.executables.forEach((executable) => {
    const details = document.createElement("details");
    details.className = "live-candidate-card";
    const summary = document.createElement("summary");
    const row = document.createElement("div");
    row.className = "tool-row";
    text(row, "strong", roleLabel(executable.role));
    text(row, "span", stateLabel(executable.selection_state), "tool-state");
    const selected = text(row, "code", selectedValue(executable));
    selected.title = selectedValue(executable);
    text(row, "small", `${executable.candidates.length}${language === "ja" ? "件" : " found"}`);
    text(row, "span", "›", "disclosure");
    summary.appendChild(row);
    details.appendChild(summary);
    const content = document.createElement("div");
    content.className = "candidate-content";
    const metadata = document.createElement("dl");
    metadata.className = "candidate-meta";
    [
      [t("field.selected"), selectedValue(executable)],
      [t("field.kind"), valueOrUnavailable(executable.selected_kind)],
      [t("field.candidates"), String(executable.candidates.length)],
      [t("field.why"), valueOrUnavailable(executable.selection_reason)],
      [t("field.verify"), valueOrUnavailable(executable.verification_command)]
    ].forEach(([label, value]) => { text(metadata, "dt", label); text(metadata, "dd", value); });
    content.appendChild(metadata);
    const list = document.createElement("div");
    list.className = "candidate-list";
    if (executable.candidates.length === 0) text(list, "span", t("field.none"));
    executable.candidates.forEach((candidate, index) => {
      const item = document.createElement("div");
      item.className = "candidate-item";
      text(item, "span", executable.selected?.path === candidate.path ? stateLabel("selected") : `#${index + 1}`);
      text(item, "code", candidate.path);
      text(item, "span", `${originLabel(candidate.origin)} · ${formatLabel(candidate.format)}`);
      list.appendChild(item);
    });
    content.appendChild(list);
    details.appendChild(content);
    container.appendChild(details);
  });
}
function renderDiagnosis(report) {
  const counts = findingCounts(report);
  const unresolved = unresolvedToolCount(report);
  const level = counts.error > 0 ? "Error" : counts.warning > 0 ? "Warning" : unresolved > 0 ? "Incomplete" : "Ok";
  document.querySelector("#diagnosis-verdict").textContent = t(`inspect.verdict${level}`);
  const benefitKey = level === "Ok" ? "inspect.benefitOk" : level === "Incomplete" ? "inspect.benefitIncomplete" : "inspect.benefitWarning";
  document.querySelector("#diagnosis-benefit").textContent = t(benefitKey);
  renderMetrics(report);
  renderContext(report);
  renderToolchain(report);
  document.querySelector("#inspect-empty").hidden = true;
  document.querySelector("#live-overview").hidden = false;
}

function findingImpact(finding) { return impactCopy[language][finding.id] ?? finding.summary; }
function renderFinding(report, index) {
  const findings = report.findings;
  const list = document.querySelector("#finding-list");
  const actions = document.querySelector("#rule-actions");
  const verification = document.querySelector("#rule-verification");
  const evidence = document.querySelector("#rule-evidence");
  list.replaceChildren(); actions.replaceChildren(); verification.replaceChildren(); evidence.replaceChildren();
  if (findings.length === 0) {
    const unresolved = report.executables.filter((item) => ["candidates_unconfirmed", "probe_failed"].includes(item.selection_state));
    if (unresolved.length > 0) {
      document.querySelector("#rule-title").textContent = t("explain.unconfirmedTitle");
      document.querySelector("#rule-summary").textContent = language === "ja" ? `${unresolved.length}ツールの実行ファイルを確定できていません。` : `${unresolved.length} tool selection(s) remain unconfirmed.`;
      document.querySelector("#rule-reason").textContent = t("explain.unconfirmedImpact");
      text(actions, "li", t("explain.unconfirmedAction"));
      unresolved.forEach((item) => text(verification, "li", `${roleLabel(item.role)}: ${item.verification_command ?? t("field.unavailable")}`));
      text(verification, "li", t("explain.unconfirmedVerify"));
      unresolved.forEach((item) => {
        const evidenceItem = document.createElement("li");
        text(evidenceItem, "span", roleLabel(item.role));
        text(evidenceItem, "strong", `${stateLabel(item.selection_state)} · ${item.candidates.length}`);
        evidence.appendChild(evidenceItem);
      });
      return;
    }
    document.querySelector("#rule-title").textContent = t("explain.noFindings");
    document.querySelector("#rule-summary").textContent = t("explain.noFindingsBody");
    document.querySelector("#rule-reason").textContent = t("explain.noFindingsBody");
    text(actions, "li", t("action.pairNow"));
    text(verification, "li", t("pair.refresh"));
    const runtimeEvidence = document.createElement("li");
    text(runtimeEvidence, "span", t("field.runtime"));
    text(runtimeEvidence, "strong", runtimeLabel(report.runtime.kind));
    evidence.appendChild(runtimeEvidence);
    return;
  }
  selectedFindingIndex = Math.min(index, findings.length - 1);
  findings.forEach((finding, findingIndex) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = `finding-chip${findingIndex === selectedFindingIndex ? " active" : ""}`;
    button.textContent = `${finding.id} · ${severityLabel(finding.severity)}`;
    button.addEventListener("click", () => renderFinding(report, findingIndex));
    list.appendChild(button);
  });
  const finding = findings[selectedFindingIndex];
  document.querySelector("#rule-title").textContent = `${finding.id} · ${finding.title}`;
  document.querySelector("#rule-summary").textContent = finding.summary;
  document.querySelector("#rule-reason").textContent = findingImpact(finding);
  finding.suggested_actions.forEach((value) => text(actions, "li", value));
  finding.verification_steps.forEach((value) => text(verification, "li", value));
  finding.evidence_ids.forEach((id) => {
    const item = document.createElement("li");
    text(item, "span", id);
    const observed = report.evidence.find((entry) => entry.id === id);
    text(item, "strong", observed?.value ?? observed?.claim ?? t("field.unavailable"));
    evidence.appendChild(item);
  });
}

function renderShareFact(container, label, value) {
  const card = document.createElement("div");
  card.className = "share-fact";
  text(card, "span", label);
  const strong = text(card, "strong", valueOrUnavailable(value));
  strong.title = valueOrUnavailable(value);
  container.appendChild(card);
}
function appendShareEnvironment(container, report, sideLabel) {
  const section = document.createElement("section");
  section.className = "share-section";
  text(section, "h4", sideLabel ? `${t("share.environment")} · ${sideLabel}` : t("share.environment"));
  const facts = document.createElement("div");
  facts.className = "share-facts";
  renderShareFact(facts, t("field.runtime"), `${runtimeLabel(report.runtime.kind)}${report.runtime.distribution ? ` · ${report.runtime.distribution}` : ""}`);
  renderShareFact(facts, t("field.shell"), report.runtime.shell);
  renderShareFact(facts, t("field.project"), `${report.project.path ?? "—"} · ${pathLabel(report.project.class)}`);
  renderShareFact(facts, t("field.agent"), `${agentLabel(report.agent.product)} · ${runtimeLabel(report.agent.runtime)}`);
  section.appendChild(facts);
  container.appendChild(section);
}
function appendShareFindings(container, reports) {
  const section = document.createElement("section");
  section.className = "share-section";
  text(section, "h4", t("share.findings"));
  const list = document.createElement("div");
  list.className = "share-findings";
  const findings = reports.flatMap((entry) => entry.report.findings.map((finding) => ({ finding, side: entry.side })));
  const unresolved = reports.flatMap((entry) => entry.report.executables
    .filter((item) => ["candidates_unconfirmed", "probe_failed"].includes(item.selection_state))
    .map((item) => ({ item, side: entry.side })));
  if (findings.length === 0 && unresolved.length === 0) text(list, "p", t("share.noFindings"));
  findings.forEach(({ finding, side }) => {
    const card = document.createElement("div");
    card.className = "share-finding";
    text(card, "strong", `${side ? `${side} · ` : ""}${finding.id} · ${finding.title}`);
    text(card, "p", finding.summary);
    list.appendChild(card);
  });
  unresolved.forEach(({ item, side }) => {
    const card = document.createElement("div");
    card.className = "share-finding";
    text(card, "strong", `${side ? `${side} · ` : ""}${roleLabel(item.role)} · ${t("share.unresolvedTitle")}`);
    text(card, "p", t("share.unresolvedBody"));
    list.appendChild(card);
  });
  section.appendChild(list);
  container.appendChild(section);
}
function renderShare(payload) {
  const container = document.querySelector("#share-html");
  container.replaceChildren();
  const paired = payload.peer?.status === "available" && payload.peer.shareable_report;
  const header = document.createElement("header");
  header.className = "share-report-header";
  const copy = document.createElement("div");
  text(copy, "h3", t(paired ? "share.pairedTitle" : "share.reportTitle"));
  text(copy, "p", t("share.safeNote"));
  header.appendChild(copy);
  text(header, "span", t("share.badge"), "badge-safe");
  container.appendChild(header);
  appendShareEnvironment(container, payload.shareable_report, paired ? t("pair.windows") : null);
  const reports = [{ report: payload.shareable_report, side: paired ? t("pair.windows") : null }];
  if (paired) {
    appendShareEnvironment(container, payload.peer.shareable_report, t("pair.wsl"));
    reports.push({ report: payload.peer.shareable_report, side: t("pair.wsl") });
  }
  appendShareFindings(container, reports);
}

function nativeOrigin(runtime) { return runtime === "windows_native" ? "windows" : ["wsl", "linux_native"].includes(runtime) ? "linux" : "unknown"; }
function selectedOrigin(executable) { return executable?.selected?.origin ?? "unknown"; }
function renderPair(payload) {
  const peer = payload.peer;
  const status = document.querySelector("#pair-status");
  const differences = document.querySelector("#pair-differences");
  const table = document.querySelector("#pair-table");
  status.replaceChildren(); differences.replaceChildren(); table.replaceChildren();
  document.querySelector("#pair-empty").hidden = true;
  document.querySelector("#pair-results").hidden = false;
  const statusCopy = document.createElement("div");
  text(statusCopy, "strong", t(peer.status === "available" ? "pair.availableTitle" : "pair.unavailableTitle"));
  text(statusCopy, "p", peer.message);
  status.appendChild(statusCopy);
  if (peer.status !== "available" || !peer.report) {
    if (peer.setup_command) {
      const setup = document.createElement("div");
      text(setup, "span", t("pair.setup"));
      text(setup, "code", peer.setup_command, "setup-command");
      status.appendChild(setup);
    }
    return;
  }
  const windows = payload.report;
  const wsl = peer.report;
  const wslCrossing = wsl.executables.filter((executable) => selectedOrigin(executable) !== "unknown" && selectedOrigin(executable) !== nativeOrigin(wsl.runtime.kind));
  [
    [t("pair.runtimeTitle"), `${runtimeLabel(windows.runtime.kind)} ↔ ${runtimeLabel(wsl.runtime.kind)}`, t("pair.runtimeOk"), "ok"],
    [t("pair.projectTitle"), `${pathLabel(windows.project.class)} ↔ ${pathLabel(wsl.project.class)}`, t("pair.projectBody"), "ok"],
    [t("pair.toolTitle"), String(wslCrossing.length), t(wslCrossing.length ? "pair.toolWarn" : "pair.toolOk"), wslCrossing.length ? "warning" : "ok"]
  ].forEach(([label, value, body, level]) => {
    const card = document.createElement("article");
    card.className = "difference-card";
    card.dataset.level = level;
    text(card, "span", label);
    text(card, "strong", value);
    text(card, "p", body);
    differences.appendChild(card);
  });
  const rows = [
    { role: t("field.runtime"), windows: runtimeLabel(windows.runtime.kind), wsl: runtimeLabel(wsl.runtime.kind), different: windows.runtime.kind !== wsl.runtime.kind },
    { role: t("field.shell"), windows: valueOrUnavailable(windows.runtime.shell), wsl: valueOrUnavailable(wsl.runtime.shell), different: windows.runtime.shell !== wsl.runtime.shell },
    { role: t("field.project"), windows: pathLabel(windows.project.class), wsl: pathLabel(wsl.project.class), different: windows.project.class !== wsl.project.class }
  ];
  ["codex", "claude", "git", "node", "npm"].forEach((role) => {
    const left = windows.executables.find((item) => item.role === role);
    const right = wsl.executables.find((item) => item.role === role);
    const leftValue = selectedDescriptor(left);
    const rightValue = selectedDescriptor(right);
    rows.push({ role: roleLabel(role), windows: leftValue, wsl: rightValue, different: leftValue !== rightValue });
  });
  const showAll = document.querySelector("#show-all-pair").checked;
  rows.filter((row) => showAll || row.different).forEach((row) => {
    const element = document.createElement("div");
    element.className = "pair-row";
    element.dataset.different = String(row.different);
    text(element, "strong", row.role);
    const left = document.createElement("div"); left.className = "pair-side"; text(left, "span", t("pair.windows")); const leftCode = text(left, "code", row.windows); leftCode.title = row.windows; element.appendChild(left);
    text(element, "span", row.different ? "≠" : "=", "pair-arrow");
    const right = document.createElement("div"); right.className = "pair-side"; text(right, "span", t("pair.wsl")); const rightCode = text(right, "code", row.wsl); rightCode.title = row.wsl; element.appendChild(right);
    table.appendChild(element);
  });
}

function renderPayload(payload, paired) {
  livePayload = payload;
  pairedPayload = paired ? payload : null;
  renderDiagnosis(payload.report);
  renderFinding(payload.report, selectedFindingIndex);
  renderShare(payload);
  if (paired) renderPair(payload);
}
function syntheticPayload(paired = false) {
  const report = {
    runtime: { kind: "windows_native", distribution: null, shell: "PowerShell 7" },
    project: { class: "windows_native", path: "[windows-project]" },
    agent: { product: "codex", runtime: "windows_native" },
    executables: ["codex", "claude", "git", "node", "npm"].map((role) => ({ role, selection_state: role === "claude" ? "not_found" : "selected", selected_kind: "application", selected_binding: null, selected: role === "claude" ? null : { path: `C:\\Tools\\${role}.exe`, origin: "windows", format: "pe" }, candidates: role === "claude" ? [] : [{ path: `C:\\Tools\\${role}.exe`, origin: "windows", format: "pe" }], selection_reason: "PATH priority was observed.", verification_command: `Get-Command -All ${role}` })),
    findings: [], evidence: []
  };
  const shareable = structuredClone(report); shareable.project.path = "[windows-project]";
  const payload = { report, shareable_report: shareable, shareable_markdown: "# ExecLocus shareable report\n", peer: { status: "not_requested", message: "" } };
  if (paired) {
    const peerReport = structuredClone(report);
    peerReport.runtime = { kind: "wsl", distribution: "Ubuntu-24.04", shell: "bash" };
    peerReport.project = { class: "windows_mounted", path: "/mnt/c/demo" };
    peerReport.agent = { product: "codex", runtime: "wsl" };
    peerReport.executables.forEach((item) => { if (item.selected) { item.selected.path = `/usr/bin/${item.role}`; item.selected.origin = "linux"; item.selected.format = "elf"; item.candidates = [item.selected]; } });
    payload.peer = { status: "available", report: peerReport, shareable_report: peerReport, shareable_markdown: "# WSL report", message: language === "ja" ? "合成データで両側を比較しました。" : "Compared both sides with synthetic data." };
    payload.paired_shareable_markdown = "# ExecLocus Windows / WSL comparison\n";
  }
  return payload;
}

async function requestDiagnostic(route, paired) {
  stage.classList.add("is-running");
  runPhase = "running";
  updateRunState();
  try {
    const payload = liveMode ? await fetchPayload(route) : syntheticPayload(paired);
    renderPayload(payload, paired);
    runPhase = "complete";
    updateRunState();
    selectView(paired ? "compare" : "inspect");
  } catch (error) {
    runPhase = "error";
    updateRunState();
    const message = error instanceof Error ? error.message : String(error);
    document.querySelector("#inspect-empty").hidden = false;
    document.querySelector("#inspect-empty strong").textContent = t("state.error");
    document.querySelector("#inspect-empty p").textContent = message;
    selectView("inspect");
  } finally {
    stage.classList.remove("is-running");
  }
}
async function fetchPayload(route) {
  const response = await fetch(`${route}?profile=${encodeURIComponent(selectedProfile)}&lang=${encodeURIComponent(language)}`, { method: "POST", headers: { "X-ExecLocus-Request": "diagnose" }, cache: "no-store" });
  if (!response.ok) throw new Error(`HTTP ${response.status}`);
  return response.json();
}
function runSingleDiagnostic() { return requestDiagnostic("/api/diagnose", false); }
function runPairedDiagnostic() { return requestDiagnostic("/api/diagnose-pair", true); }

function applyLanguage(nextLanguage) {
  language = nextLanguage;
  document.documentElement.lang = language;
  document.title = t("document.title");
  document.querySelectorAll("[data-i18n]").forEach((element) => { element.textContent = t(element.dataset.i18n); });
  const toggle = document.querySelector("#language-toggle");
  toggle.textContent = language === "ja" ? "EN" : "日本語";
  toggle.setAttribute("aria-label", t("language.label"));
  updateProfileHelp();
  updateRunState();
}

document.querySelectorAll(".rail-step").forEach((button) => button.addEventListener("click", () => selectView(button.dataset.view)));
document.querySelectorAll("[data-profile]").forEach((button) => button.addEventListener("click", () => selectProfile(button.dataset.profile)));
document.querySelector("#run-button").addEventListener("click", runSingleDiagnostic);
document.querySelector("#pair-button").addEventListener("click", runPairedDiagnostic);
document.querySelector("#guide-pair-button").addEventListener("click", runPairedDiagnostic);
document.querySelector("#guide-single-button").addEventListener("click", runSingleDiagnostic);
document.querySelector("#pair-refresh-button").addEventListener("click", runPairedDiagnostic);
document.querySelector("#show-all-pair").addEventListener("change", () => { if (pairedPayload) renderPair(pairedPayload); });
document.querySelector("#copy-button").addEventListener("click", async () => {
  const state = document.querySelector("#copy-state");
  try {
    const content = pairedPayload?.paired_shareable_markdown ?? livePayload?.shareable_markdown;
    if (!content) throw new Error("no report");
    await navigator.clipboard.writeText(content);
    state.textContent = t("share.copied");
  } catch { state.textContent = t("share.failed"); }
});
document.querySelector("#language-toggle").addEventListener("click", () => {
  const next = language === "ja" ? "en" : "ja";
  const hadPair = pairedPayload !== null;
  const hadSingle = livePayload !== null;
  const url = new URL(location.href); url.searchParams.set("lang", next); history.replaceState(null, "", url);
  applyLanguage(next);
  if (hadPair) void runPairedDiagnostic(); else if (hadSingle) void runSingleDiagnostic();
});
document.addEventListener("keydown", (event) => {
  if (event.key.toLowerCase() === "r" && !event.ctrlKey && !event.metaKey && !event.altKey) void runPairedDiagnostic();
  const keys = { "0": "guide", "1": "inspect", "2": "compare", "3": "explain", "4": "share" };
  if (keys[event.key]) selectView(keys[event.key]);
});

const requestedView = params.get("view");
if (["guide", "inspect", "compare", "explain", "share"].includes(requestedView)) selectView(requestedView);
applyLanguage(language);
