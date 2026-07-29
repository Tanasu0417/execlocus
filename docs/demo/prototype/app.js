const translations = {
  ja: {
    "document.title": "ExecLocus 操作デモ",
    "brand.subtitle": "Windows × WSL 証拠コンソール",
    "status.concept": "操作デモ",
    "status.synthetic": "合成データ · ローカルのみ",
    "status.live": "実診断",
    "status.local": "実データ · ローカルのみ",
    "nav.label": "診断手順",
    "nav.inspect": "診断",
    "nav.compare": "比較",
    "nav.explain": "説明",
    "nav.share": "共有",
    "action.run": "診断を実行",
    "profile.label": "利用目的",
    "state.ready": "準備完了",
    "state.running": "読み取り専用で根拠を収集中…",
    "state.complete": "完了 · 外部送信 0件",
    "state.error": "診断失敗 · ターミナルを確認してください",
    "inspect.eyebrow": "現在の実行環境",
    "inspect.title": "見た目ではなく、<br>実行根拠をたどる。",
    "inspect.body": "実行環境、シェル、プロジェクト境界、エージェント根拠を分離して観測します。",
    "inspect.sameProject": "同じソース一式",
    "trust.readonly": "読み取り専用",
    "trust.telemetry": "送信なし",
    "trust.shell": "シェル変更なし",
    "runtime.windows": "Windowsネイティブ",
    "candidate.selected": "選択候補",
    "compare.title": "同じソース一式。異なる実行環境。",
    "compare.badge": "合成データによる再現",
    "compare.windows": "WINDOWS 実行環境",
    "compare.wsl": "WSL 実行環境",
    "compare.boundary": "実行境界",
    "compare.note": "2つのExecLocusプロセスを同じ形式で比較します。過去にエージェントが実行した対象とは断定しません。",
    "field.runtime": "実行環境",
    "field.distribution": "ディストリビューション",
    "field.user": "ユーザー",
    "field.shell": "シェル",
    "field.project": "プロジェクト",
    "field.agent": "エージェント",
    "field.selected": "選択結果",
    "field.kind": "種類",
    "field.candidates": "候補数",
    "field.why": "理由",
    "field.verify": "確認コマンド",
    "field.origin": "由来",
    "field.format": "形式",
    "field.unavailable": "利用不可",
    "live.toolchainEyebrow": "実データ",
    "live.toolchainTitle": "現在のツール選択",
    "live.candidatesTitle": "候補・選択根拠・確認コマンド",
    "live.candidatesBadge": "実環境の観測結果",
    "live.traceId": "LIVE-LOCAL-01",
    "live.noFindings": "現在の条件では検出結果はありません",
    "live.noFindingsSummary": "問題がない場合と、判定に必要な根拠が不足している場合があります。Toolchainの状態と確認コマンドも確認してください。",
    "live.errorTitle": "診断を完了できませんでした",
    "live.errorBody": "起動中のターミナルに表示されたエラーを確認してください。",
    "live.wait": "「診断を実行」を押すと、この端末の実データを読み取り専用で表示します。",
    "candidate.contract": "シェル規則",
    "explain.eyebrow": "ルール説明",
    "explain.rule": "FS001 · マウントされたプロジェクトの境界",
    "explain.why": "なぜ重要か",
    "explain.action": "次に確認すること",
    "explain.verify": "再検証",
    "explain.syntheticVerify": "変更後に同じ条件で再診断します。",
    "explain.evidence": "根拠の連鎖",
    "share.eyebrow": "共有用レポート",
    "share.title": "共有前に、識別情報を落とす。",
    "share.badge": "自動匿名化",
    "share.removed": "描画前に除去",
    "share.username": "利用者名",
    "share.home": "ホームディレクトリ",
    "share.machine": "マシン名",
    "share.path": "個人の絶対パス",
    "share.copy": "合成レポートをコピー",
    "share.copyLive": "匿名化レポートをコピー",
    "share.warning": "未加工のターミナル出力やJSONは公開しない",
    "share.copied": "合成レポートをコピーしました · 個人のパスは含みません",
    "share.copiedLive": "匿名化レポートをコピーしました · 外部送信していません",
    "share.failed": "クリップボードを利用できません · 画面表示は維持されます",
    "report.title": "ExecLocus 共有用レポート",
    "report.schema": "形式",
    "report.profile": "利用目的",
    "report.field": "項目",
    "report.value": "値",
    "report.user": "利用者",
    "report.finding": "検出結果",
    "report.upload": "値は外部送信されていません。",
    "copy.header": "ExecLocus 合成デモ",
    "copy.profile": "利用目的",
    "copy.user": "利用者",
    "copy.project": "プロジェクト",
    "footer.concept": "コンセプト画面 — v0.1のCLI本体ではありません",
    "footer.live": "ローカル実診断 — 127.0.0.1限定 · 読み取り専用",
    "aria.status": "デモの状態",
    "aria.rail": "診断の流れ",
    "aria.map": "WindowsとWSLの実行境界",
    "aria.profile": "診断プロファイル",
    "aria.otter": "根拠の経路を案内する、顔のないカワウソのシルエット",
    "language.label": "Switch to English"
  },
  en: {
    "document.title": "ExecLocus interactive concept demo",
    "brand.subtitle": "Windows × WSL evidence console",
    "status.concept": "INTERACTIVE CONCEPT",
    "status.synthetic": "synthetic data · local only",
    "status.live": "LIVE DIAGNOSTIC",
    "status.local": "local observations · no upload",
    "nav.label": "TRACE",
    "nav.inspect": "Inspect",
    "nav.compare": "Compare",
    "nav.explain": "Explain",
    "nav.share": "Share",
    "action.run": "Run diagnostic",
    "profile.label": "Workflow profile",
    "state.ready": "ready",
    "state.running": "collecting read-only evidence…",
    "state.complete": "complete · 0 values uploaded",
    "state.error": "diagnostic failed · check the terminal",
    "inspect.eyebrow": "CURRENT EXECUTION",
    "inspect.title": "Trace evidence,<br>not appearances.",
    "inspect.body": "Observe runtime, shell, project boundary, and agent evidence as separate claims.",
    "inspect.sameProject": "same source tree",
    "trust.readonly": "READ ONLY",
    "trust.telemetry": "NO TELEMETRY",
    "trust.shell": "NO SHELL MUTATION",
    "runtime.windows": "Windows Native",
    "candidate.selected": "SELECTED",
    "compare.title": "Same source tree. Different execution context.",
    "compare.badge": "SYNTHETIC REPRODUCTION",
    "compare.windows": "WINDOWS CONTEXT",
    "compare.wsl": "WSL CONTEXT",
    "compare.boundary": "runtime boundary",
    "compare.note": "Compare two ExecLocus processes through one schema. This does not claim what an agent executed in the past.",
    "field.runtime": "Runtime",
    "field.distribution": "Distribution",
    "field.user": "User",
    "field.shell": "Shell",
    "field.project": "Project",
    "field.agent": "Agent",
    "field.selected": "Selected",
    "field.kind": "Kind",
    "field.candidates": "Candidates",
    "field.why": "Why",
    "field.verify": "Verification command",
    "field.origin": "Origin",
    "field.format": "Format",
    "field.unavailable": "Unavailable",
    "live.toolchainEyebrow": "LIVE DATA",
    "live.toolchainTitle": "Current tool selection",
    "live.candidatesTitle": "Candidates, selection evidence, and verification",
    "live.candidatesBadge": "OBSERVED LOCALLY",
    "live.traceId": "LIVE-LOCAL-01",
    "live.noFindings": "No finding was emitted for the current conditions",
    "live.noFindingsSummary": "The condition may be absent or required evidence may be unavailable. Review the Toolchain states and verification commands too.",
    "live.errorTitle": "The diagnostic could not complete",
    "live.errorBody": "Review the error printed in the terminal that started the GUI.",
    "live.wait": "Select Run diagnostic to collect read-only observations from this machine.",
    "candidate.contract": "shell contract",
    "explain.eyebrow": "RULE EXPLANATION",
    "explain.rule": "FS001 · mounted project boundary",
    "explain.why": "Why it matters",
    "explain.action": "Read-only next action",
    "explain.verify": "Reverification",
    "explain.syntheticVerify": "Rerun under the same conditions after a change.",
    "explain.evidence": "EVIDENCE CHAIN",
    "share.eyebrow": "SHAREABLE REPORT",
    "share.title": "Remove identity before sharing.",
    "share.badge": "AUTO REDACTED",
    "share.removed": "Removed before render",
    "share.username": "username",
    "share.home": "home directory",
    "share.machine": "machine name",
    "share.path": "personal absolute path",
    "share.copy": "Copy synthetic report",
    "share.copyLive": "Copy redacted report",
    "share.warning": "never publish raw terminal / raw JSON",
    "share.copied": "synthetic report copied · no personal paths",
    "share.copiedLive": "redacted report copied · no values uploaded",
    "share.failed": "clipboard unavailable · report remains on screen",
    "report.title": "ExecLocus shareable report",
    "report.schema": "Schema",
    "report.profile": "Profile",
    "report.field": "Field",
    "report.value": "Value",
    "report.user": "User",
    "report.finding": "Finding",
    "report.upload": "No values were uploaded.",
    "copy.header": "ExecLocus synthetic demo",
    "copy.profile": "profile",
    "copy.user": "user",
    "copy.project": "project",
    "footer.concept": "Concept UI — not the v0.1 CLI surface",
    "footer.live": "Local diagnostic · loopback only · read only",
    "aria.status": "Demo status",
    "aria.rail": "Diagnostic flow",
    "aria.map": "Windows and WSL execution boundary",
    "aria.profile": "Diagnostic profile",
    "aria.otter": "A faceless river otter silhouette guiding the evidence path",
    "language.label": "日本語へ切り替え"
  }
};

const profiles = {
  "share-first": {
    severityClass: "ok",
    ja: {
      severity: "想定内",
      state: "警告としては未発火",
      summary: "Windows側との共有を優先する利用目的に、Windowsマウント上のプロジェクトが一致しています。",
      reason: "エクスプローラー、Windows側のエディタ、Coworkから同じファイルへ直接アクセスする場合、この配置は意図された選択です。",
      action: "共有場所は維持できます。容量の大きいLinuxのビルド出力やキャッシュだけ、実測後に移動を検討してください。",
      finding: "FS001 · 想定内"
    },
    en: {
      severity: "EXPECTED",
      state: "Not triggered as a warning",
      summary: "The Windows-mounted project matches a workflow that prioritizes Windows interoperability.",
      reason: "The location is intentional when Explorer, Windows editors, or Cowork need direct access to the same files.",
      action: "Keep the shared location. Move caches or heavy Linux build outputs only if measurements justify it.",
      finding: "FS001 · expected context"
    }
  },
  balanced: {
    severityClass: "",
    ja: {
      severity: "情報",
      state: "状況説明として発火",
      summary: "ExecLocusがWSL内で動作し、プロジェクトはWindowsマウント上にあります。",
      reason: "Windowsとの相互運用は意図的な場合があります。性能とファイル監視の特性差は利用目的によって変わります。",
      action: "共有場所を維持し、移動を決める前に実際の作業負荷を測定してください。",
      finding: "FS001 · 情報"
    },
    en: {
      severity: "INFO",
      state: "Triggered as context",
      summary: "Project is on a Windows-mounted filesystem while ExecLocus runs inside WSL.",
      reason: "Windows interoperability may be intentional. Performance and file-watching tradeoffs depend on the selected workflow profile.",
      action: "Keep the shared location, then measure your own workload before relocating the project.",
      finding: "FS001 · Info"
    }
  },
  "linux-first": {
    severityClass: "warn",
    ja: {
      severity: "警告",
      state: "利用目的との不一致として発火",
      summary: "Linuxネイティブの性能と動作特性を優先する設定に対し、プロジェクトはWindowsマウント上にあります。",
      reason: "ファイルシステムをまたぐメタデータやファイル監視は、WSLネイティブのプロジェクトと異なる場合があります。",
      action: "移動を決める前に、/home/demo/projectでも同じ作業負荷を比較してください。",
      finding: "FS001 · 警告"
    },
    en: {
      severity: "WARNING",
      state: "Triggered for profile mismatch",
      summary: "The project is Windows-mounted while this profile prioritizes Linux-native tool performance and semantics.",
      reason: "Cross-filesystem metadata and file-watching behavior can differ from a WSL-native project.",
      action: "Compare the same workload under /home/demo/project before deciding whether to relocate.",
      finding: "FS001 · Warning"
    }
  }
};

const params = new URLSearchParams(location.search);
const liveMode = params.get("mode") === "live";
let language = params.get("lang") === "en" ? "en" : "ja";
let selectedProfile = profiles[params.get("profile")] ? params.get("profile") : "balanced";
let runPhase = "ready";
let livePayload = null;
let lastLiveError = null;
const stage = document.querySelector(".stage");
const command = document.querySelector("#command");
const runState = document.querySelector("#run-state");
const otterGuide = document.querySelector("#otter-guide");
const requestedOtterMotion = params.get("motion") === "swim" ? "swim" : "land";

function t(key) {
  return translations[language][key] ?? key;
}

function updateRunState() {
  runState.textContent = t(`state.${runPhase}`);
  runState.className = `run-state${runPhase === "running" ? " running" : runPhase === "complete" ? " complete" : runPhase === "error" ? " error" : ""}`;
}

function setOtterMotion(motion) {
  const swimming = motion === "swim";
  otterGuide.classList.toggle("is-swimming", swimming);
  otterGuide.classList.toggle("is-idle", !swimming);
}

function selectView(name) {
  document.querySelectorAll(".rail-step").forEach((button) => button.classList.toggle("active", button.dataset.view === name));
  document.querySelectorAll(".view").forEach((panel) => panel.classList.toggle("active", panel.dataset.panel === name));
}

function enumLabel(group, value) {
  const labels = {
    runtime: {
      windows_native: { ja: "Windowsネイティブ", en: "Windows Native" },
      wsl: { ja: "WSL", en: "WSL" },
      linux_native: { ja: "Linuxネイティブ", en: "Linux Native" },
      unknown: { ja: "不明", en: "Unknown" }
    },
    path: {
      windows_native: { ja: "Windowsネイティブ", en: "Windows Native" },
      windows_mounted: { ja: "Windowsマウント", en: "Windows mounted" },
      wsl_native: { ja: "WSLネイティブ", en: "WSL native" },
      wsl_unc: { ja: "WSL UNC", en: "WSL UNC" },
      linux_native: { ja: "Linuxネイティブ", en: "Linux native" },
      unknown: { ja: "不明", en: "Unknown" }
    },
    state: {
      not_found: { ja: "見つかりません", en: "Not found" },
      candidates_unconfirmed: { ja: "候補あり／選択未確定", en: "Candidates found / selection unconfirmed" },
      selected: { ja: "選択済み", en: "Selected" },
      probe_failed: { ja: "調査失敗", en: "Probe failed" }
    },
    kind: {
      alias: { ja: "エイリアス", en: "alias" },
      function: { ja: "関数", en: "function" },
      cmdlet: { ja: "コマンドレット", en: "cmdlet" },
      builtin: { ja: "組み込み", en: "builtin" },
      external_script: { ja: "外部スクリプト", en: "external script" },
      application: { ja: "アプリケーション", en: "application" }
    },
    severity: {
      info: { ja: "情報", en: "Info" },
      warning: { ja: "警告", en: "Warning" },
      error: { ja: "エラー", en: "Error" }
    },
    origin: {
      windows: { ja: "Windows", en: "Windows" },
      linux: { ja: "Linux", en: "Linux" },
      script: { ja: "スクリプト", en: "Script" },
      unknown: { ja: "不明", en: "Unknown" }
    },
    format: {
      pe: { ja: "PE", en: "PE" },
      elf: { ja: "ELF", en: "ELF" },
      script: { ja: "スクリプト", en: "script" },
      unknown: { ja: "不明", en: "unknown" }
    }
  };
  return labels[group]?.[value]?.[language] ?? value ?? t("field.unavailable");
}

function appendTextElement(parent, tagName, text, className) {
  const element = document.createElement(tagName);
  if (className) element.className = className;
  element.textContent = text;
  parent.appendChild(element);
  return element;
}

function renderLiveSummary(report) {
  const summary = document.querySelector("#live-summary");
  summary.replaceChildren();
  const agent = report.agent.product === "claude_code" ? "Claude Code" : report.agent.product === "codex" ? "Codex" : t("field.unavailable");
  const entries = [
    [t("field.runtime"), enumLabel("runtime", report.runtime.kind)],
    [t("field.distribution"), report.runtime.distribution ?? t("field.unavailable")],
    [t("field.user"), report.runtime.user ?? t("field.unavailable")],
    [t("field.shell"), report.runtime.shell ?? t("field.unavailable")],
    [t("field.project"), `${report.project.path ?? t("field.unavailable")} · ${enumLabel("path", report.project.class)}`],
    [t("field.agent"), `${agent} · ${enumLabel("runtime", report.agent.runtime)}`]
  ];
  entries.forEach(([label, value]) => {
    const card = document.createElement("article");
    appendTextElement(card, "span", label);
    appendTextElement(card, "strong", value);
    summary.appendChild(card);
  });
}

function selectedValue(executable) {
  return executable.selected?.path ?? executable.selected_binding ?? "—";
}

function renderLiveToolchain(report) {
  const summary = document.querySelector("#live-toolchain-table");
  const details = document.querySelector("#live-candidates");
  summary.replaceChildren();
  details.replaceChildren();
  report.executables.forEach((executable) => {
    const row = document.createElement("div");
    row.className = "live-table-row";
    appendTextElement(row, "strong", executable.role);
    appendTextElement(row, "span", enumLabel("state", executable.selection_state));
    appendTextElement(row, "code", selectedValue(executable));
    appendTextElement(row, "small", `${executable.candidates.length} ${language === "ja" ? "件" : "candidate(s)"}`);
    summary.appendChild(row);

    const article = document.createElement("article");
    const header = document.createElement("div");
    header.className = "live-candidate-header";
    appendTextElement(header, "h3", executable.role);
    appendTextElement(header, "span", enumLabel("state", executable.selection_state), "live-candidate-state");
    article.appendChild(header);
    const metadata = document.createElement("dl");
    metadata.className = "live-candidate-meta";
    [
      [t("field.selected"), selectedValue(executable)],
      [t("field.kind"), executable.selected_kind ? enumLabel("kind", executable.selected_kind) : "—"],
      [t("field.candidates"), String(executable.candidates.length)],
      [t("field.why"), executable.selection_reason],
      [t("field.verify"), executable.verification_command]
    ].forEach(([label, value]) => {
      appendTextElement(metadata, "dt", label);
      appendTextElement(metadata, "dd", value);
    });
    article.appendChild(metadata);
    const list = document.createElement("div");
    list.className = "candidate-list";
    executable.candidates.forEach((candidate, index) => {
      const item = document.createElement("div");
      const selected = executable.selected?.path === candidate.path;
      appendTextElement(item, "span", selected ? enumLabel("state", "selected") : `#${index + 1}`);
      appendTextElement(item, "code", candidate.path);
      appendTextElement(item, "span", `${enumLabel("origin", candidate.origin)} · ${enumLabel("format", candidate.format)}`);
      list.appendChild(item);
    });
    if (executable.candidates.length === 0) appendTextElement(list, "span", t("field.unavailable"));
    article.appendChild(list);
    details.appendChild(article);
  });
}

function renderLiveFinding(report) {
  const finding = report.findings[0];
  const title = document.querySelector("#rule-title");
  const severity = document.querySelector("#severity");
  const state = document.querySelector("#rule-state");
  const summary = document.querySelector("#rule-summary");
  const reason = document.querySelector("#rule-reason");
  const actions = document.querySelector("#rule-actions");
  const verification = document.querySelector("#rule-verification");
  const evidence = document.querySelector("#rule-evidence");
  actions.replaceChildren();
  verification.replaceChildren();
  evidence.replaceChildren();
  if (!finding) {
    title.textContent = t("live.noFindings");
    severity.textContent = language === "ja" ? "情報" : "INFO";
    severity.className = "severity ok";
    state.textContent = t("live.noFindings");
    summary.textContent = t("live.noFindingsSummary");
    reason.textContent = t("live.noFindingsSummary");
    appendTextElement(actions, "li", t("live.noFindingsSummary"));
    return;
  }
  title.textContent = `${finding.id} · ${finding.title}`;
  severity.textContent = enumLabel("severity", finding.severity);
  severity.className = `severity${finding.severity === "warning" ? " warn" : finding.severity === "info" ? " ok" : " warn"}`;
  state.textContent = language === "ja" ? "現在の実データで検出" : "Triggered by current observations";
  summary.textContent = finding.summary;
  reason.textContent = finding.summary;
  finding.suggested_actions.forEach((action) => appendTextElement(actions, "li", action));
  finding.verification_steps.forEach((step) => appendTextElement(verification, "li", step));
  finding.evidence_ids.forEach((id) => {
    const item = document.createElement("li");
    appendTextElement(item, "span", id);
    const observed = report.evidence.find((entry) => entry.id === id);
    appendTextElement(item, "strong", observed?.value ?? observed?.claim ?? t("field.unavailable"));
    evidence.appendChild(item);
  });
}

function renderLivePayload(payload) {
  livePayload = payload;
  lastLiveError = null;
  document.querySelector(".runtime-map").hidden = true;
  document.querySelector("#live-overview").hidden = false;
  document.querySelector("#synthetic-comparison").hidden = true;
  document.querySelector("#synthetic-comparison-note").hidden = true;
  document.querySelector("#live-candidates").hidden = false;
  document.querySelector("#share-report").textContent = payload.shareable_markdown;
  document.querySelector("#copy-button").textContent = t("share.copyLive");
  renderLiveSummary(payload.report);
  renderLiveToolchain(payload.report);
  renderLiveFinding(payload.report);
}

function renderLiveError() {
  document.querySelector(".runtime-map").hidden = true;
  document.querySelector("#live-overview").hidden = false;
  const summary = document.querySelector("#live-summary");
  summary.replaceChildren();
  const card = document.createElement("article");
  appendTextElement(card, "span", t("live.errorTitle"));
  appendTextElement(card, "strong", lastLiveError || t("live.errorBody"));
  summary.appendChild(card);
}

function selectProfile(name) {
  selectedProfile = name;
  const profile = profiles[name];
  const copy = profile[language];
  document.querySelectorAll("[data-profile]").forEach((button) => button.classList.toggle("active", button.dataset.profile === name));
  command.textContent = `execlocus --lang ${language} --profile ${name} ${liveMode ? "gui" : "check"}`;
  if (liveMode && livePayload) return;
  document.querySelector("#severity").textContent = copy.severity;
  document.querySelector("#severity").className = `severity ${profile.severityClass}`.trim();
  document.querySelector("#rule-state").textContent = copy.state;
  document.querySelector("#rule-summary").textContent = copy.summary;
  document.querySelector("#rule-reason").textContent = copy.reason;
  document.querySelector("#rule-action").textContent = copy.action;
  const evidenceProfile = document.querySelector("#evidence-profile");
  const reportProfile = document.querySelector("#report-profile");
  const reportFinding = document.querySelector("#report-finding");
  if (evidenceProfile) evidenceProfile.textContent = name;
  if (reportProfile) reportProfile.textContent = name;
  if (reportFinding) reportFinding.textContent = copy.finding;
}

function applyLanguage(nextLanguage) {
  language = nextLanguage;
  document.documentElement.lang = language;
  document.title = t("document.title");
  document.querySelectorAll("[data-i18n]").forEach((element) => {
    element.textContent = t(element.dataset.i18n);
  });
  document.querySelectorAll("[data-i18n-html]").forEach((element) => {
    element.innerHTML = t(element.dataset.i18nHtml);
  });
  const toggle = document.querySelector("#language-toggle");
  toggle.textContent = language === "ja" ? "EN" : "日本語";
  toggle.setAttribute("aria-label", t("language.label"));
  document.querySelector("#top-status").setAttribute("aria-label", t("aria.status"));
  document.querySelector("#diagnostic-rail").setAttribute("aria-label", t("aria.rail"));
  document.querySelector(".runtime-map").setAttribute("aria-label", t("aria.map"));
  document.querySelector(".profile-row").setAttribute("aria-label", t("aria.profile"));
  otterGuide.setAttribute("aria-label", t("aria.otter"));
  if (!livePayload) document.querySelector("#rule-title").textContent = t("explain.rule");
  if (liveMode) {
    const badge = document.querySelector("#mode-badge");
    badge.textContent = t("status.live");
    badge.className = "badge badge-live";
    document.querySelector("#data-mode").textContent = t("status.local");
    document.querySelector("#footer-mode").textContent = t("footer.live");
    document.querySelector("#copy-button").textContent = t("share.copyLive");
    document.querySelector("#view-compare .section-heading .eyebrow").textContent = t("live.traceId");
    document.querySelector("#view-compare .section-heading h2").textContent = t("live.candidatesTitle");
    document.querySelector("#view-compare .section-heading .badge").textContent = t("live.candidatesBadge");
  }
  selectProfile(selectedProfile);
  updateRunState();
}

async function runDiagnostic() {
  stage.classList.remove("is-running");
  void stage.offsetWidth;
  stage.classList.add("is-running");
  setOtterMotion("swim");
  runPhase = "running";
  updateRunState();
  if (!liveMode) {
    window.setTimeout(() => {
      runPhase = "complete";
      updateRunState();
      selectView("compare");
      if (requestedOtterMotion !== "swim") setOtterMotion("land");
    }, 720);
    return;
  }
  try {
    const response = await fetch(`/api/diagnose?profile=${encodeURIComponent(selectedProfile)}&lang=${encodeURIComponent(language)}`, {
      method: "POST",
      headers: { "X-ExecLocus-Request": "diagnose" },
      cache: "no-store"
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const payload = await response.json();
    renderLivePayload(payload);
    runPhase = "complete";
    updateRunState();
    selectView("compare");
  } catch (error) {
    lastLiveError = error instanceof Error ? error.message : String(error);
    runPhase = "error";
    updateRunState();
    renderLiveError();
    selectView("inspect");
  } finally {
    if (requestedOtterMotion !== "swim") setOtterMotion("land");
  }
}

document.querySelectorAll(".rail-step").forEach((button) => button.addEventListener("click", () => selectView(button.dataset.view)));
document.querySelectorAll("[data-profile]").forEach((button) => button.addEventListener("click", () => selectProfile(button.dataset.profile)));
document.querySelector("#run-button").addEventListener("click", runDiagnostic);
document.querySelector("#language-toggle").addEventListener("click", () => {
  const refreshLiveReport = liveMode && livePayload !== null;
  const nextLanguage = language === "ja" ? "en" : "ja";
  const url = new URL(location.href);
  url.searchParams.set("lang", nextLanguage);
  history.replaceState(null, "", url);
  applyLanguage(nextLanguage);
  if (refreshLiveReport) void runDiagnostic();
});
document.querySelector("#copy-button").addEventListener("click", async () => {
  const state = document.querySelector("#copy-state");
  state.removeAttribute("data-i18n");
  try {
    const content = liveMode && livePayload
      ? livePayload.shareable_markdown
      : `${t("copy.header")}\n${t("copy.profile")}=${selectedProfile}\n${t("copy.user")}=[redacted-user]\n${t("copy.project")}=[windows-project]`;
    await navigator.clipboard.writeText(content);
    state.textContent = t(liveMode ? "share.copiedLive" : "share.copied");
  } catch {
    state.textContent = t("share.failed");
  }
});
document.addEventListener("keydown", (event) => {
  if (event.key.toLowerCase() === "r" && !event.ctrlKey && !event.metaKey) runDiagnostic();
  if (["1", "2", "3", "4"].includes(event.key)) selectView(["inspect", "compare", "explain", "share"][Number(event.key) - 1]);
});

const requestedView = params.get("view");
if (["inspect", "compare", "explain", "share"].includes(requestedView)) selectView(requestedView);
setOtterMotion(requestedOtterMotion);
applyLanguage(language);
if (liveMode) {
  document.querySelector(".runtime-map").hidden = true;
  document.querySelector("#live-overview").hidden = false;
  document.querySelector("#synthetic-comparison").hidden = true;
  document.querySelector("#synthetic-comparison-note").hidden = true;
  document.querySelector("#live-candidates").hidden = false;
  const summary = document.querySelector("#live-summary");
  summary.replaceChildren();
  const card = document.createElement("article");
  appendTextElement(card, "span", t("state.ready"));
  appendTextElement(card, "strong", t("live.wait"));
  summary.appendChild(card);
}
