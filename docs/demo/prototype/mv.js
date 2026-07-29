const scenes = [
  { start: 0, end: 5, view: "inspect", kicker: "THE QUESTION", caption: "同じproject。選ばれるNodeも同じ？" },
  { start: 5, end: 12, view: "compare", kicker: "MANUAL CHECKS", caption: "確認結果は、terminalごとに分かれる。" },
  { start: 12, end: 17, view: "inspect", kicker: "ONE EVIDENCE VIEW", caption: "選ばれる候補と理由を、1画面へ。" },
  { start: 17, end: 25, view: "inspect", kicker: "READ-ONLY PROBE", caption: "設定なし。PATHを変えず、まず1回実行。" },
  { start: 25, end: 35, view: "compare", kicker: "PAIRED CONTEXT", caption: "同じsource treeでも、選ばれる実行ファイルは違う。" },
  { start: 35, end: 43, view: "explain", kicker: "EVIDENCE BEFORE CLAIMS", caption: "予測と、実行済みの証拠を混同しない。" },
  { start: 43, end: 51, view: "explain&profile=share-first", kicker: "PURPOSE-AWARE", caption: "/mnt/cは目的次第。共有のための正しい選択にもなる。" },
  { start: 51, end: 60, view: "share", kicker: "SHARE SAFELY", caption: "Windows × WSLの境界を、匿名化した根拠付きで。" }
];

const frame = document.querySelector("#demo-frame");
const caption = document.querySelector("#caption");
const kicker = document.querySelector("#frame-kicker");
const frameNumber = document.querySelector("#frame-number");
const progress = document.querySelector("#progress");
const timecode = document.querySelector("#timecode");
const select = document.querySelector("#scene-select");
let elapsed = 0;
let previous = 0;
let running = false;
let animationFrame = 0;
let activeIndex = -1;

for (const [index, scene] of scenes.entries()) {
  const option = document.createElement("option");
  option.value = String(index);
  option.textContent = `${index + 1}. ${scene.kicker}`;
  select.append(option);
}

function showScene(index) {
  if (index === activeIndex) return;
  activeIndex = index;
  const scene = scenes[index];
  frame.src = `index.html?view=${scene.view}`;
  caption.textContent = scene.caption;
  kicker.textContent = scene.kicker;
  frameNumber.textContent = String(index + 1).padStart(2, "0");
  select.value = String(index);
}

function render() {
  const clamped = Math.min(60, Math.max(0, elapsed));
  const index = Math.min(scenes.length - 1, scenes.findIndex((scene) => clamped >= scene.start && clamped < scene.end));
  showScene(index < 0 ? scenes.length - 1 : index);
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
select.addEventListener("change", () => { pause(); elapsed = scenes[Number(select.value)].start; render(); });

const params = new URLSearchParams(location.search);
if (params.get("capture") === "1") document.body.classList.add("capture");
render();
if (params.get("autoplay") === "1") play();
