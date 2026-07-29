const profiles = {
  "share-first": {
    severity: "EXPECTED",
    severityClass: "ok",
    state: "Not triggered as a warning",
    summary: "The Windows-mounted project matches a workflow that prioritizes Windows interoperability.",
    reason: "The location is intentional when Explorer, Windows editors, or Cowork need direct access to the same files.",
    action: "Keep the shared location. Move caches or heavy Linux build outputs only if measurements justify it.",
    finding: "FS001 · expected context"
  },
  balanced: {
    severity: "INFO",
    severityClass: "",
    state: "Triggered as context",
    summary: "Project is on a Windows-mounted filesystem while ExecLocus runs inside WSL.",
    reason: "Windows interoperability may be intentional. Performance and file-watching tradeoffs depend on the selected workflow profile.",
    action: "Keep the shared location, then measure your own workload before relocating the project.",
    finding: "FS001 · Info"
  },
  "linux-first": {
    severity: "WARNING",
    severityClass: "warn",
    state: "Triggered for profile mismatch",
    summary: "The project is Windows-mounted while this profile prioritizes Linux-native tool performance and semantics.",
    reason: "Cross-filesystem metadata and file-watching behavior can differ from a WSL-native project.",
    action: "Compare the same workload under /home/demo/project before deciding whether to relocate.",
    finding: "FS001 · Warning"
  }
};

let selectedProfile = "balanced";
const stage = document.querySelector(".stage");
const command = document.querySelector("#command");
const runState = document.querySelector("#run-state");

function selectView(name) {
  document.querySelectorAll(".rail-step").forEach((button) => button.classList.toggle("active", button.dataset.view === name));
  document.querySelectorAll(".view").forEach((panel) => panel.classList.toggle("active", panel.dataset.panel === name));
}

function selectProfile(name) {
  selectedProfile = name;
  const profile = profiles[name];
  document.querySelectorAll("[data-profile]").forEach((button) => button.classList.toggle("active", button.dataset.profile === name));
  command.textContent = `execlocus --profile ${name} check`;
  document.querySelector("#severity").textContent = profile.severity;
  document.querySelector("#severity").className = `severity ${profile.severityClass}`.trim();
  document.querySelector("#rule-state").textContent = profile.state;
  document.querySelector("#rule-summary").textContent = profile.summary;
  document.querySelector("#rule-reason").textContent = profile.reason;
  document.querySelector("#rule-action").textContent = profile.action;
  document.querySelector("#evidence-profile").textContent = name;
  document.querySelector("#report-profile").textContent = name;
  document.querySelector("#report-finding").textContent = profile.finding;
}

function runDiagnostic() {
  stage.classList.remove("is-running");
  void stage.offsetWidth;
  stage.classList.add("is-running");
  runState.textContent = "collecting read-only evidence…";
  runState.className = "run-state running";
  window.setTimeout(() => {
    runState.textContent = "complete · 0 values uploaded";
    runState.className = "run-state complete";
    selectView("compare");
  }, 720);
}

document.querySelectorAll(".rail-step").forEach((button) => button.addEventListener("click", () => selectView(button.dataset.view)));
document.querySelectorAll("[data-profile]").forEach((button) => button.addEventListener("click", () => selectProfile(button.dataset.profile)));
document.querySelector("#run-button").addEventListener("click", runDiagnostic);
document.querySelector("#copy-button").addEventListener("click", async () => {
  const state = document.querySelector("#copy-state");
  try {
    await navigator.clipboard.writeText(`ExecLocus synthetic demo\nprofile=${selectedProfile}\nuser=[redacted-user]\nproject=[windows-project]`);
    state.textContent = "synthetic report copied · no personal paths";
  } catch {
    state.textContent = "clipboard unavailable · report remains on screen";
  }
});
document.addEventListener("keydown", (event) => {
  if (event.key.toLowerCase() === "r" && !event.ctrlKey && !event.metaKey) runDiagnostic();
  if (["1", "2", "3", "4"].includes(event.key)) selectView(["inspect", "compare", "explain", "share"][Number(event.key) - 1]);
});

const requestedView = new URLSearchParams(location.search).get("view");
if (["inspect", "compare", "explain", "share"].includes(requestedView)) selectView(requestedView);
selectProfile(new URLSearchParams(location.search).get("profile") || "balanced");
