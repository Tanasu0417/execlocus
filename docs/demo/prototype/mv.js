const sceneDefinitions = [
  { start: 0, end: 5, view: "inspect&motion=swim", ja: ["問い", "同じプロジェクト。選ばれるNodeも同じ？"], en: ["THE QUESTION", "Same project. Does it resolve the same Node?"] },
  { start: 5, end: 12, view: "compare", ja: ["手作業で確認", "確認結果は、ターミナルごとに分かれる。"], en: ["MANUAL CHECKS", "Each terminal can produce a different answer."] },
  { start: 12, end: 17, view: "inspect&motion=land", ja: ["根拠を1画面へ", "選ばれる候補と理由を、1画面へ。"], en: ["ONE EVIDENCE VIEW", "Bring the selected candidate and its reasons into one view."] },
  { start: 17, end: 25, view: "inspect&motion=land", ja: ["読み取り専用", "設定なし。PATHを変えず、まず1回実行。"], en: ["READ-ONLY PROBE", "No setup. Run once without changing PATH."] },
  { start: 25, end: 35, view: "compare", ja: ["2つの実行環境", "同じソース一式でも、選ばれる実行ファイルは違う。"], en: ["PAIRED CONTEXT", "The same source tree can resolve a different executable."] },
  { start: 35, end: 43, view: "explain", ja: ["断定より根拠", "予測と、実行済みの証拠を混同しない。"], en: ["EVIDENCE BEFORE CLAIMS", "Do not confuse a prediction with evidence of past execution."] },
  { start: 43, end: 51, view: "explain&profile=share-first", ja: ["目的に合わせる", "/mnt/cは目的次第。共有のための正しい選択にもなる。"], en: ["PURPOSE-AWARE", "/mnt/c depends on intent. It can be the right choice for sharing."] },
  { start: 51, end: 60, view: "share", ja: ["安全に共有", "Windows × WSLの境界を、匿名化した根拠付きで。"], en: ["SHARE SAFELY", "Explain the Windows × WSL boundary with redacted evidence."] }
];

const uiCopy = {
  ja: {
    title: "ExecLocus 60秒コンセプト映像",
    length: "ExecLocus / 60秒",
    gate: "コンセプト映像 · 合成データ",
    play: "再生",
    pause: "一時停止",
    reset: "最初へ",
    frame: "場面",
    note: "音声なしでも意味が通る字幕版です。",
    controlsLabel: "再生操作",
    languageLabel: "Switch to English"
  },
  en: {
    title: "ExecLocus 60-second concept animatic",
    length: "ExecLocus / 60 SEC",
    gate: "CONCEPT ANIMATIC · SYNTHETIC",
    play: "Play",
    pause: "Pause",
    reset: "Reset",
    frame: "Frame",
    note: "A caption master that works without audio.",
    controlsLabel: "Playback controls",
    languageLabel: "日本語へ切り替え"
  }
};

const frame = document.querySelector("#demo-frame");
const caption = document.querySelector("#caption");
const kicker = document.querySelector("#frame-kicker");
const frameNumber = document.querySelector("#frame-number");
const progress = document.querySelector("#progress");
const timecode = document.querySelector("#timecode");
const select = document.querySelector("#scene-select");
const params = new URLSearchParams(location.search);
let language = params.get("lang") === "en" ? "en" : "ja";
let elapsed = 0;
let previous = 0;
let running = false;
let animationFrame = 0;
let activeIndex = -1;

function rebuildSceneOptions() {
  select.replaceChildren();
  for (const [index, scene] of sceneDefinitions.entries()) {
    const option = document.createElement("option");
    option.value = String(index);
    option.textContent = `${index + 1}. ${scene[language][0]}`;
    select.append(option);
  }
}

function applyLanguage(nextLanguage) {
  language = nextLanguage;
  document.documentElement.lang = language;
  document.title = uiCopy[language].title;
  document.querySelector("#mv-length").textContent = uiCopy[language].length;
  document.querySelector("#mv-gate").textContent = uiCopy[language].gate;
  document.querySelector("#play").textContent = uiCopy[language].play;
  document.querySelector("#pause").textContent = uiCopy[language].pause;
  document.querySelector("#reset").textContent = uiCopy[language].reset;
  document.querySelector("#frame-label").textContent = uiCopy[language].frame;
  document.querySelector("#mv-note").textContent = uiCopy[language].note;
  document.querySelector(".controls").setAttribute("aria-label", uiCopy[language].controlsLabel);
  const toggle = document.querySelector("#mv-language-toggle");
  toggle.textContent = language === "ja" ? "EN" : "日本語";
  toggle.setAttribute("aria-label", uiCopy[language].languageLabel);
  rebuildSceneOptions();
  activeIndex = -1;
  render();
}

function showScene(index) {
  if (index === activeIndex) return;
  activeIndex = index;
  const scene = sceneDefinitions[index];
  frame.src = `index.html?view=${scene.view}&lang=${language}`;
  caption.textContent = scene[language][1];
  kicker.textContent = scene[language][0];
  frameNumber.textContent = String(index + 1).padStart(2, "0");
  select.value = String(index);
}

function render() {
  const clamped = Math.min(60, Math.max(0, elapsed));
  const index = Math.min(sceneDefinitions.length - 1, sceneDefinitions.findIndex((scene) => clamped >= scene.start && clamped < scene.end));
  showScene(index < 0 ? sceneDefinitions.length - 1 : index);
  progress.style.width = `${(clamped / 60) * 100}%`;
  timecode.textContent = `00:${String(Math.floor(clamped)).padStart(2, "0")} / 01:00`;
}

function tick(now) {
  if (!running) return;
  if (!previous) previous = now;
  elapsed += (now - previous) / 1000;
  previous = now;
  render();
  if (elapsed >= 60) { running = false; return; }
  animationFrame = requestAnimationFrame(tick);
}

function play() {
  if (elapsed >= 60) elapsed = 0;
  if (running) return;
  running = true;
  previous = 0;
  animationFrame = requestAnimationFrame(tick);
}

function pause() { running = false; cancelAnimationFrame(animationFrame); previous = 0; }
function reset() { pause(); elapsed = 0; render(); }

document.querySelector("#play").addEventListener("click", play);
document.querySelector("#pause").addEventListener("click", pause);
document.querySelector("#reset").addEventListener("click", reset);
document.querySelector("#mv-language-toggle").addEventListener("click", () => {
  const nextLanguage = language === "ja" ? "en" : "ja";
  const url = new URL(location.href);
  url.searchParams.set("lang", nextLanguage);
  history.replaceState(null, "", url);
  applyLanguage(nextLanguage);
});
select.addEventListener("change", () => { pause(); elapsed = sceneDefinitions[Number(select.value)].start; render(); });

if (params.get("capture") === "1") document.body.classList.add("capture");
applyLanguage(language);
if (params.get("autoplay") === "1") play();
