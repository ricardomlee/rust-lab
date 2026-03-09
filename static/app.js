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
    btn.textContent = `${example.label}: ${example.value} (${example.from}→${example.to})`;
    btn.onclick = () => {
      input.value = example.value;
      fromInput.value = String(example.from);
      toInput.value = String(example.to);
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

  return {
    result: lastData.result ?? {
      chars: lastData.chars,
      words: lastData.words,
      lines: lastData.lines,
      sha256: lastData.sha256,
    },
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

  if (current.params) {
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
