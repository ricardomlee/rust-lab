let wasm = null;
let ready = false;

async function initWasm() {
  try {
    // sanity-check the wasm JS module before dynamic import to surface clear errors
    try {
      const head = await fetch('/wasm/wasm_lab.js', { method: 'HEAD' });
      if (!head.ok) {
        throw new Error(`HEAD /wasm/wasm_lab.js returned ${head.status}`);
      }
    } catch (err) {
      postMessage({ type: 'error', error: `wasm JS not reachable: ${err}` });
      return;
    }

    const mod = await import('/wasm/wasm_lab.js');
    if (typeof mod.default === 'function') await mod.default();
    wasm = mod;
    ready = true;
    postMessage({ type: 'ready' });
  } catch (e) {
    postMessage({ type: 'error', error: String(e) });
  }
}

initWasm();

onmessage = function (ev) {
  const m = ev.data;
  if (m && m.type === 'convert') {
    if (!ready || !wasm) {
      postMessage({ id: m.id, error: 'wasm not ready' });
      return;
    }

    try {
      const s = performance.now();
      const result = wasm.base_convert(m.value, m.from, m.to);
      const elapsed_us = Math.round((performance.now() - s) * 1000);
      postMessage({ id: m.id, result, elapsed_us });
    } catch (e) {
      postMessage({ id: m.id, error: String(e) });
    }
  }
};
