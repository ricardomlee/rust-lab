// WASM worker: dynamically imports the wasm-pack `web` build and dispatches messages.
// Uses paths relative to the worker file so the site works under any base
// (e.g. `https://user.github.io/rust-lab/`).

let wasm = null;
let ready = false;

const wasmJsUrl = new URL('./wasm/wasm_lab.js', self.location.href).toString();

async function initWasm() {
  try {
    try {
      const head = await fetch(wasmJsUrl, { method: 'HEAD' });
      if (!head.ok) {
        throw new Error(`HEAD ${wasmJsUrl} returned ${head.status}`);
      }
    } catch (err) {
      postMessage({ type: 'error', error: `wasm JS not reachable: ${err}` });
      return;
    }

    const mod = await import(wasmJsUrl);
    if (typeof mod.default === 'function') await mod.default();
    wasm = mod;
    ready = true;
    postMessage({ type: 'ready' });
  } catch (e) {
    postMessage({ type: 'error', error: String(e) });
  }
}

initWasm();

function timed(fn) {
  const s = performance.now();
  const result = fn();
  const elapsed_us = Math.round((performance.now() - s) * 1000);
  return { result, elapsed_us };
}

const handlers = {
  textAnalyze: (m) => timed(() => wasm.analyze_text(m.text)),
  convert: (m) => timed(() => wasm.base_convert(m.value, m.from, m.to)),
  formatBytes: (m) => timed(() => wasm.format_bytes(m.bytes)),
  formatDuration: (m) => timed(() => wasm.format_duration(m.milliseconds)),
  formatFrameTime: (m) => timed(() => wasm.format_frame_time(m.fps)),
  formatPercent: (m) => timed(() => wasm.format_percent(m.value)),
  formatCountdown: (m) => timed(() => wasm.format_countdown(m.total_seconds)),
  hexToRgba: (m) => timed(() => wasm.hex_to_rgba(m.hex)),
  rgbToHsl: (m) => timed(() => wasm.rgb_to_hsl(m.r, m.g, m.b)),
  hslToRgb: (m) => timed(() => wasm.hsl_to_rgb(m.h, m.s, m.l)),
  blendColors: (m) => timed(() => wasm.blend_colors(m.r1, m.g1, m.b1, m.r2, m.g2, m.b2, m.ratio)),
  generatePalette: (m) => timed(() => wasm.generate_palette(m.r1, m.g1, m.b1, m.r2, m.g2, m.b2, m.steps)),
  base64Encode: (m) => timed(() => wasm.base64_encode(m.input, !!m.url_safe)),
  base64Decode: (m) => timed(() => wasm.base64_decode(m.encoded, !!m.url_safe)),
  hashCompute: (m) => timed(() => wasm.hash_compute(m.input, m.algorithm)),
  uuidGenerate: () => timed(() => wasm.generate_uuid()),
  shortIdGenerate: () => timed(() => wasm.generate_short_id()),
};

onmessage = function (ev) {
  const m = ev.data;
  if (!m) return;

  if (!ready || !wasm) {
    postMessage({ id: m.id, error: 'wasm not ready' });
    return;
  }

  const handler = handlers[m.type];
  if (!handler) {
    postMessage({ id: m.id, error: `unknown wasm message type: ${m.type}` });
    return;
  }

  try {
    const { result, elapsed_us } = handler(m);
    postMessage({ id: m.id, result, elapsed_us });
  } catch (e) {
    postMessage({ id: m.id, error: String(e) });
  }
};
