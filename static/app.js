// ========================
// 实验注册表（核心）
// ========================
const experiments = {
  textAnalyze: {
    title: "文本分析",
    endpoint: "/api/text/analyze",
    placeholder: "输入文本进行分析",
  },
    baseConvert: {
    title: "进制转换",
    endpoint: "/api/base/convert",
    placeholder: "输入数字（如 ff、1011、-42）",
    params: ["from", "to"],
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

let current = null;
let worker = null;
let workerReady = false;
let nextWorkerId = 1;
const pending = new Map();

function initWorker() {
  try {
    worker = new Worker('/wasm_worker.js');
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
function selectExperiment(exp) {
  current = exp;
  title.textContent = exp.title;
  input.placeholder = exp.placeholder;
  input.value = "";
  output.textContent = "等待输入…";

  paramsDiv.classList.toggle("hidden", !exp.params);
}

// ========================
// 运行实验
// ========================
const fromInput = document.getElementById("from");
const toInput = document.getElementById("to");
async function runExperiment() {
  if (!current) {
    output.textContent = "❗ 请先选择一个实验";
    return;
  }

  output.textContent = "⏳ 运行中…";

  let body = { text: input.value };

  if (current.params) {
    body = {
      value: input.value,
      from: Number(fromInput.value),
      to: Number(toInput.value),
    };
  }
  try {
    // For experiments with params (like base convert) run both WASM (local) and API (server)
    if (current.params) {
      const compare = {};

      // number of runs (avg)
      const runs = Math.max(1, Math.min(1000, Number(runsInput?.value) || 1));

      // WASM run via worker (if available) - average over `runs`
      if (worker && workerReady) {
        try {
          let total_us = 0;
          let lastResult = null;
          for (let i = 0; i < runs; i++) {
            const id = nextWorkerId++;
            const promise = new Promise((resolve) => pending.set(id, resolve));
            worker.postMessage({ type: 'convert', id, value: body.value, from: body.from, to: body.to });
            const m = await promise;
            if (m.error) {
              compare.wasm = { error: m.error };
              break;
            }
            lastResult = m.result;
            total_us += Number(m.elapsed_us || 0);
          }
          if (!compare.wasm) {
            compare.wasm = { result: lastResult, runs, avg_elapsed_us: Math.round(total_us / runs) };
          }
        } catch (e) {
          console.warn('WASM worker failed', e);
          compare.wasm = { error: String(e) };
        }
      } else {
        compare.wasm = { error: 'wasm worker not ready' };
      }

      // API run
      try {
        // API run repeated `runs` times to get averages
        let total_roundtrip = 0;
        let total_compute = 0;
        let lastResult = null;
        let computeAvailable = false;
        for (let i = 0; i < runs; i++) {
          const s = performance.now();
          const res = await fetch(current.endpoint, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body),
          });
          const roundtrip_us = Math.round((performance.now() - s) * 1000);
          const data = await res.json();
          if (!res.ok) {
            compare.api = { error: data.error };
            break;
          }
          lastResult = data.result;
          if (data.elapsed_us != null) {
            computeAvailable = true;
            total_compute += Number(data.elapsed_us);
          }
          total_roundtrip += roundtrip_us;
        }
        if (!compare.api) {
          compare.api = {
            result: lastResult,
            runs,
            avg_roundtrip_us: Math.round(total_roundtrip / runs),
            avg_compute_us: computeAvailable ? Math.round(total_compute / runs) : null,
          };
        }
      } catch (e) {
        console.warn("API call failed", e);
        compare.api = { error: String(e) };
      }

      output.textContent = JSON.stringify(compare, null, 2);
      return;
    }

    // Non-param experiments: fallback to existing API call
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
