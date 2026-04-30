// =====================================================================
// Rust Lab — front-end controller
// All experiments run via WASM in a Web Worker. When a backend `/api`
// is reachable (e.g. `cargo run`), an additional API benchmark is shown
// next to the WASM result for comparison. On GitHub Pages the API is
// silently skipped.
// =====================================================================

// ---------- Worker plumbing ----------
let worker = null;
let workerReady = false;
let workerError = null;
let nextWorkerId = 1;
const pendingMsgs = new Map();

function initWorker() {
  try {
    const workerUrl = new URL('./wasm_worker.js', document.baseURI);
    worker = new Worker(workerUrl, { type: 'module' });
    worker.onmessage = (ev) => {
      const m = ev.data;
      if (m && m.type === 'ready') {
        workerReady = true;
        updateStatus();
      }
      if (m && m.type === 'error') {
        workerError = m.error;
        updateStatus();
      }
      if (m && m.id) {
        const cb = pendingMsgs.get(m.id);
        if (cb) {
          pendingMsgs.delete(m.id);
          cb(m);
        }
      }
    };
  } catch (e) {
    workerError = String(e);
    updateStatus();
  }
}

function requestWorker(message) {
  return new Promise((resolve) => {
    const id = nextWorkerId++;
    pendingMsgs.set(id, resolve);
    worker.postMessage({ ...message, id });
  });
}

// ---------- API auto-detect ----------
let apiAvailable = false;
let apiChecked = false;

async function probeApi() {
  try {
    const res = await fetch('./api/health', { method: 'GET' });
    if (res.ok) {
      apiAvailable = true;
    }
  } catch {
    apiAvailable = false;
  }
  apiChecked = true;
  updateStatus();
  if (current) renderModePill();
}

// ---------- Status indicators ----------
const statusEls = {
  apiDot: document.getElementById('api-dot'),
  apiTxt: document.getElementById('api-status'),
  wasmDot: document.getElementById('wasm-dot'),
  wasmTxt: document.getElementById('wasm-status'),
};

function updateStatus() {
  if (workerReady) {
    statusEls.wasmDot.className = 'dot dot-on';
    statusEls.wasmTxt.textContent = '已就绪';
  } else if (workerError) {
    statusEls.wasmDot.className = 'dot dot-warn';
    statusEls.wasmTxt.textContent = '加载失败';
    statusEls.wasmTxt.title = workerError;
  } else {
    statusEls.wasmDot.className = 'dot dot-off';
    statusEls.wasmTxt.textContent = '加载中…';
  }

  if (!apiChecked) {
    statusEls.apiDot.className = 'dot dot-off';
    statusEls.apiTxt.textContent = '检测中…';
  } else if (apiAvailable) {
    statusEls.apiDot.className = 'dot dot-on';
    statusEls.apiTxt.textContent = '已连接';
  } else {
    statusEls.apiDot.className = 'dot dot-off';
    statusEls.apiTxt.textContent = '不可用 (静态站)';
  }
}

// ---------- Experiment registry ----------
// Each experiment declares its inputs, how to call WASM, how to call the API
// (optional), and how to render the result.
const experiments = {
  // ----- 文本与编码 -----
  textAnalyze: {
    group: '文本与编码',
    title: '文本分析',
    emoji: '📝',
    help: '统计字符 / 单词 / 行数，并计算 SHA-256 摘要。同一份输入分别走 Rust API 与浏览器 WASM。',
    fields: [
      { name: 'text', kind: 'textarea', label: '输入文本', rows: 5, default: 'Rust + WASM makes browsers surprisingly capable.\n\nTry me.' },
    ],
    quickExamples: [
      { label: '中英混排', text: '你好，Rust 🦀 与 WebAssembly 让浏览器变得很能打。' },
      { label: '多行', text: 'line one\nline two\nline three' },
      { label: 'Markdown', text: '# Title\n\n- item 1\n- item 2\n- item 3' },
    ],
    wasm: (v) => ({ type: 'textAnalyze', text: v.text }),
    api: { endpoint: './api/text/analyze', body: (v) => ({ text: v.text }) },
    parseResult: (data) => ({ chars: data.chars, words: data.words, lines: data.lines, sha256: data.sha256 }),
    render: renderJson,
  },

  baseConvert: {
    group: '文本与编码',
    title: '进制转换',
    emoji: '🔢',
    help: '在 2–36 进制之间互转，支持负数。WASM 与 API 各跑 N 次取平均值。',
    fields: [
      { name: 'value', kind: 'text', label: '数值', default: 'ff' },
      { name: 'from', kind: 'number', label: 'from 进制', default: 16, min: 2, max: 36 },
      { name: 'to', kind: 'number', label: 'to 进制', default: 10, min: 2, max: 36 },
    ],
    quickExamples: [
      { label: 'Hex → Dec', value: 'ff', from: 16, to: 10 },
      { label: 'Bin → Hex', value: '101101', from: 2, to: 16 },
      { label: 'Negative', value: '-42', from: 10, to: 2 },
      { label: 'Base36', value: 'zz', from: 36, to: 10 },
    ],
    wasm: (v) => ({ type: 'convert', value: v.value, from: Number(v.from), to: Number(v.to) }),
    api: { endpoint: './api/base/convert', body: (v) => ({ value: v.value, from: Number(v.from), to: Number(v.to) }) },
    parseResult: (data) => ({ result: data.result }),
    render: renderJson,
    defaultRuns: 5,
  },

  base64Tool: {
    group: '文本与编码',
    title: 'Base64 编解码',
    emoji: '🔐',
    help: '在标准 / URL-safe 两种 Base64 变体之间编解码，纯 Rust 实现，浏览器内完成。',
    fields: [
      { name: 'mode', kind: 'select', label: '操作', options: [{ value: 'encode', label: '编码 Encode' }, { value: 'decode', label: '解码 Decode' }], default: 'encode' },
      { name: 'urlSafe', kind: 'select', label: '变体', options: [{ value: 'false', label: 'standard' }, { value: 'true', label: 'url_safe' }], default: 'false' },
      { name: 'input', kind: 'textarea', label: '输入', rows: 4, default: 'Hello, Rust + WASM!' },
    ],
    quickExamples: [
      { label: 'ASCII', input: 'Hello, Rust + WASM!', mode: 'encode', urlSafe: 'false' },
      { label: '中文', input: '你好，世界！', mode: 'encode', urlSafe: 'false' },
      { label: '解码示例', input: 'SGVsbG8sIFJ1c3QgKyBXQVNNIQ==', mode: 'decode', urlSafe: 'false' },
    ],
    wasm: (v) => v.mode === 'encode'
      ? { type: 'base64Encode', input: v.input, url_safe: v.urlSafe === 'true' }
      : { type: 'base64Decode', encoded: v.input, url_safe: v.urlSafe === 'true' },
    api: {
      endpoint: (v) => v.mode === 'encode' ? './api/base64/encode' : './api/base64/decode',
      body: (v) => v.mode === 'encode'
        ? { input: v.input, url_safe: v.urlSafe === 'true' }
        : { encoded: v.input, url_safe: v.urlSafe === 'true' },
    },
    parseResult: (data) => ({ value: data.value, operation: data.operation, variant: data.variant }),
    render: renderBase64,
  },

  hashTool: {
    group: '文本与编码',
    title: '哈希计算',
    emoji: '#️⃣',
    help: '计算输入字符串的 SHA-256 / SHA-512 摘要，输出 hex 文案。',
    fields: [
      { name: 'algorithm', kind: 'select', label: '算法', options: [{ value: 'SHA256', label: 'SHA-256' }, { value: 'SHA512', label: 'SHA-512' }], default: 'SHA256' },
      { name: 'input', kind: 'textarea', label: '输入', rows: 4, default: 'hello' },
    ],
    quickExamples: [
      { label: 'hello', input: 'hello', algorithm: 'SHA256' },
      { label: 'empty', input: '', algorithm: 'SHA256' },
      { label: 'long → 512', input: 'The quick brown fox jumps over the lazy dog', algorithm: 'SHA512' },
    ],
    wasm: (v) => ({ type: 'hashCompute', input: v.input, algorithm: v.algorithm }),
    api: { endpoint: './api/hash/compute', body: (v) => ({ input: v.input, algorithm: v.algorithm }) },
    parseResult: (data) => ({ algorithm: data.algorithm, hash: data.hash }),
    render: renderHash,
  },

  uuidTool: {
    group: '文本与编码',
    title: 'UUID 生成',
    emoji: '🪪',
    help: '生成 RFC4122 v4 UUID，或一个更短的 URL-safe ID。每次点击重新摇号。',
    fields: [
      { name: 'kind', kind: 'select', label: '类型', options: [{ value: 'uuid', label: 'UUID v4' }, { value: 'short', label: '短 ID (12 chars)' }], default: 'uuid' },
    ],
    runLabel: '🎲 生成',
    wasm: (v) => v.kind === 'uuid' ? { type: 'uuidGenerate' } : { type: 'shortIdGenerate' },
    api: {
      endpoint: (v) => v.kind === 'uuid' ? './api/uuid/generate' : './api/uuid/short',
      body: () => null,
      method: 'GET',
    },
    parseResult: (data) => data,
    render: renderUuid,
    defaultRuns: 1,
  },

  // ----- 数字格式化 -----
  bytesFormat: {
    group: '数字格式化',
    title: '字节大小格式化',
    emoji: '💾',
    help: '把字节数同时输出成 binary（KiB/MiB）/ decimal（KB/MB）以及对应的速率文案，适合文件大小、下载面板。',
    fields: [
      { name: 'bytes', kind: 'number', label: '字节数', default: 5368709120, min: 0, step: 1 },
    ],
    quickExamples: [
      { label: '1.5 KiB', bytes: 1536 },
      { label: '1 MiB', bytes: 1048576 },
      { label: '5 GiB', bytes: 5368709120 },
      { label: 'Tiny', bytes: 999 },
    ],
    wasm: (v) => ({ type: 'formatBytes', bytes: Number(v.bytes) }),
    api: { endpoint: './api/bytes/format', body: (v) => ({ bytes: Number(v.bytes) }) },
    parseResult: (data) => ({
      bytes: data.bytes,
      binary: data.binary,
      decimal: data.decimal,
      binary_per_second: data.binary_per_second,
      decimal_per_second: data.decimal_per_second,
    }),
    render: renderJson,
    defaultRuns: 20,
  },

  durationFormat: {
    group: '数字格式化',
    title: '时长格式化',
    emoji: '⏱️',
    help: '把毫秒数同时格式化为紧凑写法、时钟写法与中文可读文案。',
    fields: [
      { name: 'milliseconds', kind: 'number', label: '毫秒数', default: 3726045, min: 0 },
    ],
    quickExamples: [
      { label: '250 ms', milliseconds: 250 },
      { label: '1h 2m 6s', milliseconds: 3726045 },
      { label: '2d 3h 4m 5s', milliseconds: 183845000 },
      { label: 'Zero', milliseconds: 0 },
    ],
    wasm: (v) => ({ type: 'formatDuration', milliseconds: Number(v.milliseconds) }),
    api: { endpoint: './api/duration/format', body: (v) => ({ milliseconds: Number(v.milliseconds) }) },
    parseResult: (data) => ({ milliseconds: data.milliseconds, compact: data.compact, clock: data.clock, verbose_zh: data.verbose_zh }),
    render: renderJson,
    defaultRuns: 20,
  },

  frameTimeFormat: {
    group: '数字格式化',
    title: '帧时间换算',
    emoji: '🎞️',
    help: '把 FPS 换算成每帧耗时，并附带可读标签与流畅度提示。',
    fields: [
      { name: 'fps', kind: 'number', label: 'FPS', default: 60, step: 0.001, min: 0 },
    ],
    quickExamples: [
      { label: 'Cinema 24', fps: 24 },
      { label: 'Smooth 60', fps: 60 },
      { label: 'High 144', fps: 144 },
      { label: 'NTSC 23.976', fps: 23.976 },
    ],
    wasm: (v) => ({ type: 'formatFrameTime', fps: Number(v.fps) }),
    api: { endpoint: './api/frame-time/format', body: (v) => ({ fps: Number(v.fps) }) },
    parseResult: (data) => ({ fps: data.fps, frame_ms: data.frame_ms, label: data.label, quality_hint: data.quality_hint }),
    render: renderJson,
    defaultRuns: 20,
  },

  countdownFormat: {
    group: '数字格式化',
    title: '倒计时拆解',
    emoji: '⏳',
    help: '把剩余秒数拆成 d / h / m / s、时钟文案与状态提示，适合活动倒计时、任务到期。',
    fields: [
      { name: 'total_seconds', kind: 'number', label: '剩余秒数', default: 3726, step: 1 },
    ],
    quickExamples: [
      { label: 'Due now', total_seconds: 0 },
      { label: 'In 45s', total_seconds: 45 },
      { label: '1h 2m 6s', total_seconds: 3726 },
      { label: 'Expired', total_seconds: -30 },
      { label: '10d 10:10:10', total_seconds: 900610 },
    ],
    wasm: (v) => ({ type: 'formatCountdown', total_seconds: Number(v.total_seconds) }),
    api: { endpoint: './api/countdown/format', body: (v) => ({ total_seconds: Number(v.total_seconds) }) },
    parseResult: (data) => ({
      total_seconds: data.total_seconds, sign: data.sign,
      days: data.days, hours: data.hours, minutes: data.minutes, seconds: data.seconds,
      clock: data.clock, compact: data.compact, status: data.status,
    }),
    render: renderJson,
    defaultRuns: 20,
  },

  percentFormat: {
    group: '数字格式化',
    title: '百分比格式化',
    emoji: '📊',
    help: '把 0–1 的小数同时格式化为百分比、ratio 文案与状态提示。',
    fields: [
      { name: 'value', kind: 'number', label: '比例 (0–1)', default: 0.875, step: 0.01 },
    ],
    quickExamples: [
      { label: 'Tiny', value: 0.0325 },
      { label: 'Healthy', value: 0.875 },
      { label: 'Full', value: 1 },
      { label: 'Overflow clamp', value: 1.42 },
    ],
    wasm: (v) => ({ type: 'formatPercent', value: Number(v.value) }),
    api: { endpoint: './api/percent/format', body: (v) => ({ value: Number(v.value) }) },
    parseResult: (data) => ({ value: data.value, percent: data.percent, ratio: data.ratio, hint: data.hint }),
    render: renderJson,
    defaultRuns: 20,
  },

  // ----- 颜色工具 -----
  colorConvert: {
    group: '颜色工具',
    title: 'HEX ↔ RGB / HSL',
    emoji: '🎨',
    help: '把 HEX 颜色解析为 RGB / HSL，并可视化预览。WASM 与 API 输出对照。',
    fields: [
      { name: 'hex', kind: 'text', label: 'HEX', default: '#38bdf8' },
    ],
    quickExamples: [
      { label: 'Sky', hex: '#38bdf8' },
      { label: 'Rose', hex: '#f43f5e' },
      { label: 'Emerald', hex: '#10b981' },
      { label: 'Short', hex: '#fa0' },
      { label: 'With alpha', hex: '#38bdf880' },
    ],
    wasm: (v) => ({ type: 'hexToRgba', hex: v.hex }),
    api: { endpoint: './api/color/hex-to-rgba', body: (v) => ({ hex: v.hex }) },
    parseResult: (data) => ({ r: data.r, g: data.g, b: data.b, a: data.a, css: data.css }),
    render: renderColor,
  },

  colorBlend: {
    group: '颜色工具',
    title: '颜色混合',
    emoji: '🌈',
    help: '在两个颜色之间按指定比例线性插值，返回结果颜色。WASM-only。',
    apiOnlyHidden: true,
    fields: [
      { name: 'a', kind: 'color', label: '颜色 A', default: '#38bdf8' },
      { name: 'b', kind: 'color', label: '颜色 B', default: '#f43f5e' },
      { name: 'ratio', kind: 'number', label: '比例 (0–1)', default: 0.5, step: 0.05, min: 0, max: 1 },
    ],
    quickExamples: [
      { label: '50/50', a: '#38bdf8', b: '#f43f5e', ratio: 0.5 },
      { label: '偏 A', a: '#10b981', b: '#a855f7', ratio: 0.2 },
      { label: '偏 B', a: '#f59e0b', b: '#06b6d4', ratio: 0.8 },
    ],
    wasm: (v) => {
      const a = parseHex(v.a);
      const b = parseHex(v.b);
      return { type: 'blendColors', r1: a.r, g1: a.g, b1: a.b, r2: b.r, g2: b.g, b2: b.b, ratio: Number(v.ratio) };
    },
    parseResult: (data) => data,
    render: (root, results, vals) => renderBlend(root, results, vals),
  },

  colorPalette: {
    group: '颜色工具',
    title: '调色板生成',
    emoji: '🖌️',
    help: '在两个颜色之间均匀采样，生成一组渐变色。适合做主题色、图表配色起点。WASM-only。',
    apiOnlyHidden: true,
    fields: [
      { name: 'a', kind: 'color', label: '起始色', default: '#0ea5e9' },
      { name: 'b', kind: 'color', label: '结束色', default: '#a855f7' },
      { name: 'steps', kind: 'number', label: '步数', default: 8, min: 2, max: 32, step: 1 },
    ],
    quickExamples: [
      { label: 'Sky → Violet', a: '#0ea5e9', b: '#a855f7', steps: 8 },
      { label: 'Sunset', a: '#f97316', b: '#ec4899', steps: 8 },
      { label: 'Forest', a: '#10b981', b: '#1e3a8a', steps: 10 },
    ],
    wasm: (v) => {
      const a = parseHex(v.a);
      const b = parseHex(v.b);
      return { type: 'generatePalette', r1: a.r, g1: a.g, b1: a.b, r2: b.r, g2: b.g, b2: b.b, steps: Number(v.steps) };
    },
    parseResult: (data) => data,
    render: (root, results) => renderPalette(root, results),
  },
};

// ---------- DOM ----------
const sidebarList = document.getElementById('experiments');
const titleEl = document.getElementById('title');
const helpEl = document.getElementById('help');
const emojiEl = document.getElementById('exp-emoji');
const modePillEl = document.getElementById('exp-mode-pill');
const fieldsEl = document.getElementById('dynamic-fields');
const quickEl = document.getElementById('quick-examples');
const runBtn = document.getElementById('run');
const runLabelEl = document.getElementById('run-label');
const runsInput = document.getElementById('runs');
const resultEl = document.getElementById('result');
const sidebar = document.querySelector('aside.app-sidebar');
const mobileToggle = document.getElementById('mobile-toggle');

let current = null;

// ---------- Sidebar render ----------
function renderSidebar() {
  const groups = new Map();
  for (const [key, exp] of Object.entries(experiments)) {
    if (!groups.has(exp.group)) groups.set(exp.group, []);
    groups.get(exp.group).push({ key, exp });
  }

  for (const [group, items] of groups) {
    const heading = document.createElement('div');
    heading.className = 'group-title';
    heading.textContent = group;
    sidebarList.appendChild(heading);

    for (const { key, exp } of items) {
      const item = document.createElement('div');
      item.className = 'nav-item';
      item.dataset.key = key;
      item.innerHTML = `<span class="nav-emoji">${exp.emoji}</span><span>${exp.title}</span>`;
      item.onclick = () => {
        selectExperiment(key);
        sidebar?.classList.remove('open');
      };
      sidebarList.appendChild(item);
    }
  }
}

function setActiveNav(key) {
  for (const el of document.querySelectorAll('.nav-item')) {
    el.classList.toggle('active', el.dataset.key === key);
  }
}

// ---------- Experiment select ----------
function selectExperiment(key) {
  const exp = experiments[key];
  if (!exp) return;
  current = { key, exp };
  setActiveNav(key);

  emojiEl.textContent = exp.emoji;
  titleEl.textContent = exp.title;
  helpEl.textContent = exp.help ?? '';
  runLabelEl.textContent = exp.runLabel ?? '▶ 运行实验';
  runsInput.value = String(exp.defaultRuns ?? 1);
  runsInput.parentElement.style.display = exp.runLabel === '🎲 生成' ? 'none' : '';

  renderModePill();
  renderFields(exp);
  renderQuickExamples(exp);
  resultEl.innerHTML = '<div class="text-sm text-slate-500 italic">点击 “运行实验” 查看结果。</div>';
}

function renderModePill() {
  if (!current) return;
  const { exp } = current;
  const hasApi = !!exp.api && apiAvailable;
  if (exp.apiOnlyHidden || !exp.api) {
    modePillEl.textContent = 'WASM only';
    modePillEl.className = 'pill pill-violet';
    modePillEl.classList.remove('hidden');
  } else if (hasApi) {
    modePillEl.textContent = 'API + WASM';
    modePillEl.className = 'pill pill-emerald';
    modePillEl.classList.remove('hidden');
  } else {
    modePillEl.textContent = apiChecked ? 'WASM only · API 不可用' : '检测中…';
    modePillEl.className = 'pill pill-amber';
    modePillEl.classList.remove('hidden');
  }
}

function renderFields(exp) {
  fieldsEl.innerHTML = '';
  for (const f of exp.fields ?? []) {
    const wrap = document.createElement('div');
    if (f.kind === 'textarea') {
      wrap.innerHTML = `<label class="label">${f.label}</label>
        <textarea class="field font-mono" rows="${f.rows ?? 4}" data-name="${f.name}"></textarea>`;
      wrap.querySelector('textarea').value = f.default ?? '';
    } else if (f.kind === 'select') {
      const opts = (f.options ?? []).map(o => `<option value="${o.value}">${o.label}</option>`).join('');
      wrap.innerHTML = `<label class="label">${f.label}</label>
        <select class="field" data-name="${f.name}">${opts}</select>`;
      wrap.querySelector('select').value = String(f.default ?? '');
    } else if (f.kind === 'color') {
      wrap.innerHTML = `<label class="label">${f.label}</label>
        <div class="flex items-center gap-2">
          <input type="color" data-name="${f.name}" class="h-10 w-12 rounded-md cursor-pointer bg-transparent border border-white/10" />
          <input type="text" data-name-mirror="${f.name}" class="field font-mono" />
        </div>`;
      const colorInput = wrap.querySelector(`[data-name="${f.name}"]`);
      const textInput = wrap.querySelector(`[data-name-mirror="${f.name}"]`);
      colorInput.value = f.default;
      textInput.value = f.default;
      colorInput.addEventListener('input', () => textInput.value = colorInput.value);
      textInput.addEventListener('input', () => {
        if (/^#([0-9a-fA-F]{6})$/.test(textInput.value)) colorInput.value = textInput.value;
      });
    } else {
      const type = f.kind === 'number' ? 'number' : 'text';
      const extra = f.kind === 'number'
        ? `${f.min != null ? `min="${f.min}"` : ''} ${f.max != null ? `max="${f.max}"` : ''} ${f.step != null ? `step="${f.step}"` : ''}`
        : '';
      wrap.innerHTML = `<label class="label">${f.label}</label>
        <input type="${type}" class="field font-mono" data-name="${f.name}" ${extra} />`;
      wrap.querySelector('input').value = f.default ?? '';
    }
    fieldsEl.appendChild(wrap);
  }
}

function renderQuickExamples(exp) {
  quickEl.innerHTML = '';
  if (!exp.quickExamples?.length) {
    quickEl.classList.add('hidden');
    return;
  }
  quickEl.classList.remove('hidden');
  for (const ex of exp.quickExamples) {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'quick-chip';
    btn.textContent = ex.label;
    btn.onclick = () => applyQuickExample(ex);
    quickEl.appendChild(btn);
  }
}

function applyQuickExample(ex) {
  for (const [key, val] of Object.entries(ex)) {
    if (key === 'label') continue;
    const el = fieldsEl.querySelector(`[data-name="${key}"]`);
    if (el) {
      el.value = String(val);
      // sync mirror for color inputs
      const mirror = fieldsEl.querySelector(`[data-name-mirror="${key}"]`);
      if (mirror) mirror.value = String(val);
      el.dispatchEvent(new Event('input', { bubbles: true }));
    }
  }
}

function readValues() {
  const result = {};
  for (const el of fieldsEl.querySelectorAll('[data-name]')) {
    result[el.dataset.name] = el.value;
  }
  return result;
}

// ---------- Run experiment ----------
async function runExperiment() {
  if (!current) return;
  const { exp } = current;

  if (!workerReady) {
    showError('WASM 还未就绪，请稍候重试' + (workerError ? `（${workerError}）` : ''));
    return;
  }

  const values = readValues();
  const runs = Math.max(1, Math.min(1000, Number(runsInput.value) || 1));

  setRunning(true);
  resultEl.innerHTML = `<div class="text-sm text-slate-400 flex items-center gap-2"><span class="spinner"></span> 运行中…</div>`;

  try {
    const wasmRes = await runWasmBenchmark(runs, () => exp.wasm(values));
    if (wasmRes.error) {
      showError('WASM 错误：' + wasmRes.error);
      return;
    }

    let apiRes = null;
    if (exp.api && apiAvailable && !exp.apiOnlyHidden) {
      apiRes = await runApiBenchmark(runs, exp.api, values);
    }

    renderResults(exp, wasmRes, apiRes, values);
  } catch (err) {
    console.error(err);
    showError('运行失败：' + (err?.message ?? err));
  } finally {
    setRunning(false);
  }
}

function setRunning(running) {
  runBtn.disabled = running;
  runLabelEl.innerHTML = running
    ? '<span class="spinner"></span> 运行中…'
    : (current?.exp.runLabel ?? '▶ 运行实验');
}

function showError(msg) {
  resultEl.innerHTML = `<div class="compare-card border-rose-500/40 bg-rose-500/10 text-rose-200 text-sm">❌ ${escapeHtml(msg)}</div>`;
}

async function runWasmBenchmark(runs, build) {
  let total = 0, last = null;
  for (let i = 0; i < runs; i++) {
    const m = await requestWorker(build());
    if (m.error) return { error: m.error };
    last = m.result;
    total += Number(m.elapsed_us || 0);
  }
  return { result: last, runs, avg_elapsed_us: Math.round(total / runs) };
}

async function runApiBenchmark(runs, apiCfg, values) {
  const endpoint = typeof apiCfg.endpoint === 'function' ? apiCfg.endpoint(values) : apiCfg.endpoint;
  const method = apiCfg.method ?? 'POST';
  const buildBody = apiCfg.body ?? (() => ({}));
  let totalRoundtrip = 0;
  let totalCompute = 0;
  let lastData = null;
  let computeAvailable = false;

  for (let i = 0; i < runs; i++) {
    const start = performance.now();
    let res;
    if (method === 'GET') {
      res = await fetch(endpoint);
    } else {
      const body = buildBody(values);
      res = await fetch(endpoint, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body ?? {}),
      });
    }
    const roundtrip_us = Math.round((performance.now() - start) * 1000);
    let data;
    try { data = await res.json(); } catch { data = { error: `HTTP ${res.status}` }; }
    if (!res.ok) return { error: data?.error ?? `HTTP ${res.status}` };
    lastData = data;
    if (data.elapsed_us != null) {
      computeAvailable = true;
      totalCompute += Number(data.elapsed_us);
    }
    totalRoundtrip += roundtrip_us;
  }

  return {
    result: lastData,
    runs,
    avg_roundtrip_us: Math.round(totalRoundtrip / runs),
    avg_compute_us: computeAvailable ? Math.round(totalCompute / runs) : null,
  };
}

// ---------- Renderers ----------
function fmtUs(us) {
  if (us == null) return '—';
  if (us < 1000) return `${us} µs`;
  if (us < 1_000_000) return `${(us / 1000).toFixed(2)} ms`;
  return `${(us / 1_000_000).toFixed(3)} s`;
}

function escapeHtml(s) {
  return String(s).replace(/[&<>"']/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);
}

function renderResults(exp, wasmRes, apiRes, values) {
  resultEl.innerHTML = '';

  const grid = document.createElement('div');
  grid.className = apiRes ? 'grid grid-cols-1 lg:grid-cols-2 gap-4' : 'grid grid-cols-1 gap-4';

  // WASM card
  const wasmCard = document.createElement('div');
  wasmCard.className = 'compare-card';
  wasmCard.innerHTML = `<h3>
      <span class="flex items-center gap-2">🦀 WASM <span class="pill pill-sky">浏览器内</span></span>
      <span class="text-xs text-slate-400">avg ${fmtUs(wasmRes.avg_elapsed_us)} · ${wasmRes.runs} runs</span>
    </h3>`;
  const wasmBody = document.createElement('div');
  exp.render(wasmBody, parseWasmResult(wasmRes.result, exp), values);
  wasmCard.appendChild(wasmBody);
  grid.appendChild(wasmCard);

  // API card (if applicable)
  if (apiRes) {
    const apiCard = document.createElement('div');
    apiCard.className = 'compare-card';
    if (apiRes.error) {
      apiCard.innerHTML = `<h3><span class="flex items-center gap-2">⚙️ API <span class="pill pill-rose">错误</span></span></h3>
        <div class="text-sm text-rose-200">${escapeHtml(apiRes.error)}</div>`;
    } else {
      apiCard.innerHTML = `<h3>
          <span class="flex items-center gap-2">⚙️ API <span class="pill pill-emerald">服务端</span></span>
          <span class="text-xs text-slate-400">RTT ${fmtUs(apiRes.avg_roundtrip_us)}${apiRes.avg_compute_us != null ? ` · 计算 ${fmtUs(apiRes.avg_compute_us)}` : ''} · ${apiRes.runs} runs</span>
        </h3>`;
      const apiBody = document.createElement('div');
      const parsed = exp.parseResult ? exp.parseResult(apiRes.result) : apiRes.result;
      exp.render(apiBody, parsed, values);
      apiCard.appendChild(apiBody);
    }
    grid.appendChild(apiCard);
  }

  resultEl.appendChild(grid);
}

function parseWasmResult(raw, exp) {
  // Most WASM exports return JSON strings; some (palette) return arrays of strings.
  if (Array.isArray(raw)) {
    return raw.map((s) => typeof s === 'string' ? safeJson(s) : s);
  }
  if (typeof raw === 'string') {
    const parsed = safeJson(raw);
    return exp.parseResult ? exp.parseResult(parsed) : parsed;
  }
  return raw;
}

function safeJson(s) {
  try { return JSON.parse(s); } catch { return s; }
}

function renderJson(root, data) {
  const pre = document.createElement('pre');
  pre.className = 'code-block font-mono';
  pre.textContent = JSON.stringify(data, null, 2);
  root.appendChild(pre);
}

function renderBase64(root, data) {
  root.innerHTML = `
    <div class="space-y-2">
      <div class="text-xs text-slate-400">${escapeHtml(data.operation || '')} · ${escapeHtml(data.variant || '')}</div>
      <pre class="code-block font-mono">${escapeHtml(data.value ?? '')}</pre>
    </div>`;
}

function renderHash(root, data) {
  root.innerHTML = `
    <div class="space-y-2">
      <div class="text-xs text-slate-400">${escapeHtml(data.algorithm || '')}</div>
      <pre class="code-block font-mono break-all">${escapeHtml(data.hash ?? '')}</pre>
    </div>`;
}

function renderUuid(root, data) {
  const value = data?.uuid ?? data?.id ?? '';
  root.innerHTML = `
    <div class="space-y-2">
      <pre class="code-block font-mono text-emerald-300 text-base">${escapeHtml(value)}</pre>
      <div class="text-xs text-slate-400">${data?.uuid ? `version v${data.version} · ${data.variant}` : `length ${data.length} · url-safe ${data.url_safe}`}</div>
    </div>`;
}

function renderColor(root, data, values) {
  const css = data.css ?? `rgba(${data.r},${data.g},${data.b},${(data.a ?? 255) / 255})`;
  const swatch = document.createElement('div');
  swatch.className = 'swatch';
  swatch.style.background = css;
  root.appendChild(swatch);

  const hsl = rgbToHsl(data.r, data.g, data.b);
  const meta = document.createElement('div');
  meta.className = 'stat-grid';
  meta.innerHTML = `
    <div><span>HEX</span><br/><b>${escapeHtml(values.hex)}</b></div>
    <div><span>RGBA</span><br/><b>${data.r}, ${data.g}, ${data.b}, ${data.a}</b></div>
    <div><span>CSS</span><br/><b>${escapeHtml(css)}</b></div>
    <div><span>HSL</span><br/><b>${hsl.h}°, ${hsl.s}%, ${hsl.l}%</b></div>
  `;
  root.appendChild(meta);
}

function renderBlend(root, data, values) {
  const swatch = document.createElement('div');
  swatch.className = 'swatch';
  const css = data.css ?? `rgb(${data.r}, ${data.g}, ${data.b})`;
  swatch.style.background = css;
  root.appendChild(swatch);

  const meta = document.createElement('div');
  meta.className = 'stat-grid';
  meta.innerHTML = `
    <div><span>A</span><br/><b>${escapeHtml(values.a)}</b></div>
    <div><span>B</span><br/><b>${escapeHtml(values.b)}</b></div>
    <div><span>比例</span><br/><b>${escapeHtml(values.ratio)}</b></div>
    <div><span>结果</span><br/><b>${escapeHtml(data.hex ?? css)}</b></div>
  `;
  root.appendChild(meta);
}

function renderPalette(root, data) {
  // data is array of parsed objects { r, g, b, hex }
  const wrap = document.createElement('div');
  wrap.className = 'palette-strip';
  wrap.style.gridTemplateColumns = `repeat(${Math.min(data.length, 8)}, minmax(0, 1fr))`;
  for (const c of data) {
    const div = document.createElement('div');
    div.style.background = `rgb(${c.r}, ${c.g}, ${c.b})`;
    div.textContent = c.hex;
    wrap.appendChild(div);
  }
  root.appendChild(wrap);
}

// ---------- Helpers ----------
function parseHex(hex) {
  const h = hex.replace('#', '');
  if (h.length === 3) {
    return { r: parseInt(h[0] + h[0], 16), g: parseInt(h[1] + h[1], 16), b: parseInt(h[2] + h[2], 16) };
  }
  return { r: parseInt(h.slice(0, 2), 16), g: parseInt(h.slice(2, 4), 16), b: parseInt(h.slice(4, 6), 16) };
}

function rgbToHsl(r, g, b) {
  r /= 255; g /= 255; b /= 255;
  const max = Math.max(r, g, b), min = Math.min(r, g, b);
  let h = 0, s, l = (max + min) / 2;
  if (max === min) {
    h = s = 0;
  } else {
    const d = max - min;
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    switch (max) {
      case r: h = (g - b) / d + (g < b ? 6 : 0); break;
      case g: h = (b - r) / d + 2; break;
      case b: h = (r - g) / d + 4; break;
    }
    h *= 60;
  }
  return { h: Math.round(h), s: Math.round(s * 100), l: Math.round(l * 100) };
}

// ---------- Init ----------
runBtn.onclick = runExperiment;
mobileToggle?.addEventListener('click', () => sidebar?.classList.toggle('open'));

initWorker();
probeApi();
renderSidebar();
selectExperiment('textAnalyze');
updateStatus();
