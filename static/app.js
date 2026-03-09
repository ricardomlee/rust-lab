// ========================
// 实验注册表（核心）
// ========================
const experiments = {
  textAnalyze: {
    title: "文本分析",
    endpoint: "/api/text/analyze",
    placeholder: "输入文本进行分析",
    sampleInput: "Rust + WASM makes browsers surprisingly capable.\n\nTry me.",
    help: "同一输入同时走 Rust API 与 WASM worker，统计字符、单词、行数，并计算 SHA-256。",
    supportsWasmCompare: true,
  },
  baseConvert: {
    title: "进制转换",
    endpoint: "/api/base/convert",
    placeholder: "输入数字（如 ff、1011、-42）",
    params: ["from", "to"],
    sampleInput: "ff",
    defaults: { from: 16, to: 10, runs: 5 },
    help: "同一输入同时走 Rust API 与 WASM worker，便于比较结果和耗时。支持 2–36 进制与负数。",
    quickExamples: [
      { label: "Hex → Dec", value: "ff", from: 16, to: 10 },
      { label: "Bin → Hex", value: "101101", from: 2, to: 16 },
      { label: "Negative", value: "-42", from: 10, to: 2 },
      { label: "Base36", value: "zz", from: 36, to: 10 },
    ],
  },
  bytesFormat: {
    title: "字节大小格式化",
    endpoint: "/api/bytes/format",
    placeholder: "输入字节数（如 1536、1048576、5368709120）",
    sampleInput: "5368709120",
    defaults: { runs: 20 },
    help: "同一输入同时走 Rust API 与 WASM worker，输出 binary（KiB/MiB）/ decimal（KB/MB）大小文案，以及 B/s、KiB/s、MB/s 等速率文案，适合文件大小、下载速度、存储面板等工具原型。",
    quickExamples: [
      { label: "1.5 KiB", value: "1536" },
      { label: "1 MiB", value: "1048576" },
      { label: "5 GiB", value: "5368709120" },
      { label: "Tiny", value: "999" },
    ],
    supportsWasmCompare: true,
    kind: "bytes",
  },
  durationFormat: {
    title: "时长格式化",
    endpoint: "/api/duration/format",
    placeholder: "输入毫秒数（如 250、3726045、183845000）",
    sampleInput: "3726045",
    defaults: { runs: 20 },
    help: "同一输入同时走 Rust API 与 WASM worker，输出紧凑写法、时钟写法与中文可读文案，适合任务耗时、倒计时、作业面板、日志展示等小工具。",
    quickExamples: [
      { label: "250 ms", value: "250" },
      { label: "1h 2m 6s", value: "3726045" },
      { label: "2d 3h 4m 5s", value: "183845000" },
      { label: "Zero", value: "0" },
    ],
    supportsWasmCompare: true,
    kind: "duration",
  },
  frameTimeFormat: {
    title: "帧时间换算",
    endpoint: "/api/frame-time/format",
    placeholder: "输入 FPS（如 24、60、144、23.976）",
    sampleInput: "60",
    defaults: { runs: 20 },
    help: "同一输入同时走 Rust API 与 WASM worker，把 FPS 换算成每帧耗时并附带可读标签，适合动画、游戏、渲染与性能面板类小工具。",
    quickExamples: [
      { label: "Cinema", value: "24" },
      { label: "Smooth", value: "60" },
      { label: "High Refresh", value: "144" },
      { label: "NTSC", value: "23.976" },
    ],
    supportsWasmCompare: true,
    kind: "frameTime",
  },
  countdownFormat: {
    title: "倒计时拆解",
    endpoint: "/api/countdown/format",
    placeholder: "输入秒数（如 45、3726、-30、900610）",
    sampleInput: "3726",
    defaults: { runs: 20 },
    help: "同一输入同时走 Rust API 与 WASM worker，把倒计时秒数拆成 d/h/m/s、时钟文案和状态提示，适合活动倒计时、任务到期提醒、排程面板等小工具。",
    quickExamples: [
      { label: "Due now", value: "0" },
      { label: "In 45s", value: "45" },
      { label: "1h 2m 6s", value: "3726" },
      { label: "Expired", value: "-30" },
      { label: "10d 10:10:10", value: "900610" },
    ],
    supportsWasmCompare: true,
    kind: "countdown",
  },
  percentFormat: {
    title: "百分比格式化",
    endpoint: "/api/percent/format",
    placeholder: "输入 0–1 之间的小数（如 0.0325、0.875、1）",
    sampleInput: "0.875",
    defaults: { runs: 20 },
    help: "同一输入同时走 Rust API 与 WASM worker，把比例值格式化为百分比、ratio 文案和健康提示，适合仪表盘、进度条、监控卡片等小工具。",
    quickExamples: [
      { label: "Tiny", value: "0.0325" },
      { label: "Healthy", value: "0.875" },
      { label: "Full", value: "1" },
      { label: "Overflow clamp", value: "1.42" },
    ],
    supportsWasmCompare: true,
    kind: "percent",
  },
};

// ========================
// DOM 引用
// ========================
const list = document.getElementById("experiments");
const title = document.getElementById("title");
const input = document.getElementById("input");
const output = document.getElementById("output");
const runBtn = document.getElementById("run");
const runsInput = document.getElementById("runs");
const helpText = document.getElementById("help");
const quickExamples = document.getElementById("quick-examples");

let current = null;
let worker = null;
let workerReady = false;
let nextWorkerId = 1;
const pending = new Map();

function initWorker() {
  try {
    worker = new Worker('/wasm_worker.js', { type: 'module' });
    worker.onmessage = (ev) => {
      const m = ev.data;
      if (m && m.type === 'ready') {
        workerReady = true;
        console.log('wasm worker ready');
      }
      if (m && m.type === 'error') {
        console.warn('worker init error', m.error);
      }
      if (m && m.id) {
        const cb = pending.get(m.id);
        if (cb) {
          pending.delete(m.id);
          cb(m);
        }
      }
    };
  } catch (e) {
    console.warn('Failed to start worker', e);
    worker = null;
  }
}

initWorker();

function requestWorker(message) {
  return new Promise((resolve) => {
    const id = nextWorkerId++;
    pending.set(id, resolve);
    worker.postMessage({ ...message, id });
  });
}

// ========================
// 渲染实验列表
// ========================
function renderExperiments() {
  for (const exp of Object.values(experiments)) {
    const li = document.createElement("li");
    li.textContent = exp.title;
    li.className =
      "cursor-pointer hover:text-emerald-400 transition";

    li.onclick = () => selectExperiment(exp);

    list.appendChild(li);
  }
}

// ========================
// 选择实验
// ========================
const paramsDiv = document.getElementById("params");
const fromInput = document.getElementById("from");
const toInput = document.getElementById("to");

function renderQuickExamples(exp) {
  quickExamples.innerHTML = "";

  if (!exp.quickExamples?.length) {
    quickExamples.classList.add("hidden");
    return;
  }

  quickExamples.classList.remove("hidden");

  for (const example of exp.quickExamples) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "rounded border border-emerald-700 px-3 py-1 text-xs text-emerald-300 hover:bg-emerald-900/40 transition";
    btn.textContent = exp.params
      ? `${example.label}: ${example.value} (${example.from}→${example.to})`
      : `${example.label}: ${example.value}`;
    btn.onclick = () => {
      input.value = example.value;
      if (exp.params) {
        fromInput.value = String(example.from);
        toInput.value = String(example.to);
      }
    };
    quickExamples.appendChild(btn);
  }
}

function selectExperiment(exp) {
  current = exp;
  title.textContent = exp.title;
  input.placeholder = exp.placeholder;
  input.value = exp.sampleInput ?? "";
  output.textContent = "等待输入…";
  helpText.textContent = exp.help ?? "";

  paramsDiv.classList.toggle("hidden", !exp.params);

  if (exp.defaults) {
    fromInput.value = String(exp.defaults.from ?? "");
    toInput.value = String(exp.defaults.to ?? "");
    runsInput.value = String(exp.defaults.runs ?? 1);
  } else {
    fromInput.value = "";
    toInput.value = "";
    runsInput.value = "1";
  }

  renderQuickExamples(exp);
}

async function runWorkerBenchmark(runs, buildRequest) {
  if (!worker || !workerReady) {
    return { error: 'wasm worker not ready' };
  }

  let total_us = 0;
  let lastResult = null;
  for (let i = 0; i < runs; i++) {
    const m = await requestWorker(buildRequest());
    if (m.error) {
      return { error: m.error };
    }
    lastResult = m.result;
    total_us += Number(m.elapsed_us || 0);
  }

  return {
    result: lastResult,
    runs,
    avg_elapsed_us: Math.round(total_us / runs),
  };
}

async function runApiBenchmark(runs, endpoint, body) {
  let total_roundtrip = 0;
  let total_compute = 0;
  let lastData = null;
  let computeAvailable = false;

  for (let i = 0; i < runs; i++) {
    const s = performance.now();
    const res = await fetch(endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    const roundtrip_us = Math.round((performance.now() - s) * 1000);
    const data = await res.json();
    if (!res.ok) {
      return { error: data.error };
    }
    lastData = data;
    if (data.elapsed_us != null) {
      computeAvailable = true;
      total_compute += Number(data.elapsed_us);
    }
    total_roundtrip += roundtrip_us;
  }

  let result = lastData.result;
  if (result == null) {
    if (lastData.binary != null && lastData.decimal != null) {
      result = {
        bytes: lastData.bytes,
        binary: lastData.binary,
        decimal: lastData.decimal,
        binary_per_second: lastData.binary_per_second,
        decimal_per_second: lastData.decimal_per_second,
      };
    } else if (lastData.compact != null && lastData.clock != null) {
      result = {
        milliseconds: lastData.milliseconds,
        compact: lastData.compact,
        clock: lastData.clock,
        verbose_zh: lastData.verbose_zh,
      };
    } else if (lastData.frame_ms != null && lastData.label != null) {
      result = {
        fps: lastData.fps,
        frame_ms: lastData.frame_ms,
        label: lastData.label,
        quality_hint: lastData.quality_hint,
      };
    } else if (lastData.total_seconds != null && lastData.clock != null && lastData.status != null) {
      result = {
        total_seconds: lastData.total_seconds,
        sign: lastData.sign,
        days: lastData.days,
        hours: lastData.hours,
        minutes: lastData.minutes,
        seconds: lastData.seconds,
        clock: lastData.clock,
        compact: lastData.compact,
        status: lastData.status,
      };
    } else {
      result = {
        chars: lastData.chars,
        words: lastData.words,
        lines: lastData.lines,
        sha256: lastData.sha256,
      };
    }
  }

  return {
    result,
    runs,
    avg_roundtrip_us: Math.round(total_roundtrip / runs),
    avg_compute_us: computeAvailable ? Math.round(total_compute / runs) : null,
  };
}

// ========================
// 运行实验
// ========================
async function runExperiment() {
  if (!current) {
    output.textContent = "❗ 请先选择一个实验";
    return;
  }

  output.textContent = "⏳ 运行中…";

  const runs = Math.max(1, Math.min(1000, Number(runsInput?.value) || 1));

  let body = { text: input.value };

  if (current.kind === "bytes") {
    const bytes = Number(input.value.trim());
    if (!Number.isInteger(bytes) || bytes < 0) {
      output.textContent = "❌ 请输入非负整数的字节数";
      return;
    }
    body = { bytes };
  } else if (current.kind === "duration") {
    const milliseconds = Number(input.value.trim());
    if (!Number.isInteger(milliseconds) || milliseconds < 0) {
      output.textContent = "❌ 请输入非负整数的毫秒数";
      return;
    }
    body = { milliseconds };
  } else if (current.kind === "frameTime") {
    const fps = Number(input.value.trim());
    if (!Number.isFinite(fps) || fps <= 0) {
      output.textContent = "❌ 请输入大于 0 的 FPS 数值";
      return;
    }
    body = { fps };
  } else if (current.kind === "countdown") {
    const total_seconds = Number(input.value.trim());
    if (!Number.isInteger(total_seconds)) {
      output.textContent = "❌ 请输入整数秒数";
      return;
    }
    body = { total_seconds };
  } else if (current.kind === "percent") {
    const value = Number(input.value.trim());
    if (!Number.isFinite(value)) {
      output.textContent = "❌ 请输入有效数字";
      return;
    }
    body = { value };
  } else if (current.params) {
    body = {
      value: input.value,
      from: Number(fromInput.value),
      to: Number(toInput.value),
    };
  }

  try {
    if (current.params) {
      const compare = {};
      compare.wasm = await runWorkerBenchmark(runs, () => ({
        type: 'convert',
        value: body.value,
        from: body.from,
        to: body.to,
      }));
      compare.api = await runApiBenchmark(runs, current.endpoint, body);
      output.textContent = JSON.stringify(compare, null, 2);
      return;
    }

    if (current.kind === "bytes") {
      const compare = {};
      compare.wasm = await runWorkerBenchmark(runs, () => ({
        type: 'formatBytes',
        bytes: body.bytes,
      }));
      compare.api = await runApiBenchmark(runs, current.endpoint, body);
      output.textContent = JSON.stringify(compare, null, 2);
      return;
    }

    if (current.kind === "duration") {
      const compare = {};
      compare.wasm = await runWorkerBenchmark(runs, () => ({
        type: 'formatDuration',
        milliseconds: body.milliseconds,
      }));
      compare.api = await runApiBenchmark(runs, current.endpoint, body);
      output.textContent = JSON.stringify(compare, null, 2);
      return;
    }

    if (current.kind === "frameTime") {
      const compare = {};
      compare.wasm = await runWorkerBenchmark(runs, () => ({
        type: 'formatFrameTime',
        fps: body.fps,
      }));
      compare.api = await runApiBenchmark(runs, current.endpoint, body);
      output.textContent = JSON.stringify(compare, null, 2);
      return;
    }

    if (current.kind === "countdown") {
      const compare = {};
      compare.wasm = await runWorkerBenchmark(runs, () => ({
        type: "formatCountdown",
        total_seconds: body.total_seconds,
      }));
      compare.api = await runApiBenchmark(runs, current.endpoint, body);
      output.textContent = JSON.stringify(compare, null, 2);
      return;
    }

    if (current.kind === "percent") {
      const compare = {};
      compare.wasm = await runWorkerBenchmark(runs, () => ({
        type: 'formatPercent',
        value: body.value,
      }));
      compare.api = await runApiBenchmark(runs, current.endpoint, body);
      output.textContent = JSON.stringify(compare, null, 2);
      return;
    }

    if (current.supportsWasmCompare) {
      const compare = {};
      compare.wasm = await runWorkerBenchmark(runs, () => ({
        type: 'textAnalyze',
        text: body.text,
      }));
      compare.api = await runApiBenchmark(runs, current.endpoint, body);
      output.textContent = JSON.stringify(compare, null, 2);
      return;
    }

    const res = await fetch(current.endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });

    const data = await res.json();

    if (!res.ok) {
      output.textContent = "❌ 错误：" + data.error;
      return;
    }

    output.textContent = JSON.stringify(data, null, 2);
  } catch (err) {
    console.warn(err);
    output.textContent = "❌ 网络错误";
  }
}

// ========================
// 事件绑定
// ========================
runBtn.onclick = runExperiment;

// ========================
// 启动
// ========================
renderExperiments();
selectExperiment(experiments.baseConvert);
