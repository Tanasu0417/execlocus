const translations = {
  ja: {
    "document.title": "ExecLocus 操作デモ",
    "brand.subtitle": "Windows × WSL 証拠コンソール",
    "status.concept": "操作デモ",
    "status.synthetic": "合成データ · ローカルのみ",
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
    "inspect.eyebrow": "現在の実行環境",
    "inspect.title": "見た目ではなく、<br>実行根拠をたどる。",
    "inspect.body": "実行環境、シェル、プロジェクト境界、エージェント根拠を分離して観測します。",
    "inspect.sameProject": "同じソース一式",
    "trust.readonly": "読み取り専用",
    "trust.telemetry": "送信なし",
    "trust.shell": "シェル実行なし",
    "runtime.windows": "Windowsネイティブ",
    "candidate.selected": "選択候補",
    "compare.title": "同じソース一式。異なる実行環境。",
    "compare.badge": "合成データによる再現",
    "compare.windows": "WINDOWS 実行環境",
    "compare.wsl": "WSL 実行環境",
    "compare.boundary": "実行境界",
    "compare.note": "2つのExecLocusプロセスを同じ形式で比較します。過去にエージェントが実行した対象とは断定しません。",
    "field.runtime": "実行環境",
    "field.shell": "シェル",
    "field.project": "プロジェクト",
    "candidate.contract": "シェル規則",
    "explain.eyebrow": "ルール説明",
    "explain.rule": "FS001 · マウントされたプロジェクトの境界",
    "explain.why": "なぜ重要か",
    "explain.action": "次に確認すること",
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
    "share.warning": "未加工のターミナル出力やJSONは公開しない",
    "share.copied": "合成レポートをコピーしました · 個人のパスは含みません",
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
    "inspect.eyebrow": "CURRENT EXECUTION",
    "inspect.title": "Trace evidence,<br>not appearances.",
    "inspect.body": "Observe runtime, shell, project boundary, and agent evidence as separate claims.",
    "inspect.sameProject": "same source tree",
    "trust.readonly": "READ ONLY",
    "trust.telemetry": "NO TELEMETRY",
    "trust.shell": "NO SHELL EXEC",
    "runtime.windows": "Windows Native",
    "candidate.selected": "SELECTED",
    "compare.title": "Same source tree. Different execution context.",
    "compare.badge": "SYNTHETIC REPRODUCTION",
    "compare.windows": "WINDOWS CONTEXT",
    "compare.wsl": "WSL CONTEXT",
    "compare.boundary": "runtime boundary",
    "compare.note": "Compare two ExecLocus processes through one schema. This does not claim what an agent executed in the past.",
    "field.runtime": "Runtime",
    "field.shell": "Shell",
    "field.project": "Project",
    "candidate.contract": "shell contract",
    "explain.eyebrow": "RULE EXPLANATION",
    "explain.rule": "FS001 · mounted project boundary",
    "explain.why": "Why it matters",
    "explain.action": "Read-only next action",
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
    "share.warning": "never publish raw terminal / raw JSON",
    "share.copied": "synthetic report copied · no personal paths",
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
let language = params.get("lang") === "en" ? "en" : "ja";
let selectedProfile = profiles[params.get("profile")] ? params.get("profile") : "balanced";
let runPhase = "ready";
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
  runState.className = `run-state${runPhase === "running" ? " running" : runPhase === "complete" ? " complete" : ""}`;
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

function selectProfile(name) {
  selectedProfile = name;
  const profile = profiles[name];
  const copy = profile[language];
  document.querySelectorAll("[data-profile]").forEach((button) => button.classList.toggle("active", button.dataset.profile === name));
  command.textContent = `execlocus --profile ${name} check`;
  document.querySelector("#severity").textContent = copy.severity;
  document.querySelector("#severity").className = `severity ${profile.severityClass}`.trim();
  document.querySelector("#rule-state").textContent = copy.state;
  document.querySelector("#rule-summary").textContent = copy.summary;
  document.querySelector("#rule-reason").textContent = copy.reason;
  document.querySelector("#rule-action").textContent = copy.action;
  document.querySelector("#evidence-profile").textContent = name;
  document.querySelector("#report-profile").textContent = name;
  document.querySelector("#report-finding").textContent = copy.finding;
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
  document.querySelector("#rule-title").textContent = t("explain.rule");
  selectProfile(selectedProfile);
  updateRunState();
}

function runDiagnostic() {
  stage.classList.remove("is-running");
  void stage.offsetWidth;
  stage.classList.add("is-running");
  setOtterMotion("swim");
  runPhase = "running";
  updateRunState();
  window.setTimeout(() => {
    runPhase = "complete";
    updateRunState();
    selectView("compare");
    if (requestedOtterMotion !== "swim") setOtterMotion("land");
  }, 720);
}

document.querySelectorAll(".rail-step").forEach((button) => button.addEventListener("click", () => selectView(button.dataset.view)));
document.querySelectorAll("[data-profile]").forEach((button) => button.addEventListener("click", () => selectProfile(button.dataset.profile)));
document.querySelector("#run-button").addEventListener("click", runDiagnostic);
document.querySelector("#language-toggle").addEventListener("click", () => {
  const nextLanguage = language === "ja" ? "en" : "ja";
  const url = new URL(location.href);
  url.searchParams.set("lang", nextLanguage);
  history.replaceState(null, "", url);
  applyLanguage(nextLanguage);
});
document.querySelector("#copy-button").addEventListener("click", async () => {
  const state = document.querySelector("#copy-state");
  state.removeAttribute("data-i18n");
  try {
    await navigator.clipboard.writeText(`${t("copy.header")}\n${t("copy.profile")}=${selectedProfile}\n${t("copy.user")}=[redacted-user]\n${t("copy.project")}=[windows-project]`);
    state.textContent = t("share.copied");
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
