# rust-lab

一个用于探索 Rust + WASM 能力边界的小型实验场。

## 当前实验

- 文本分析：同一份输入分别走服务端 Rust API 与浏览器内 WASM worker，比较结果与耗时。
- 进制转换：支持 2–36 进制与负数，同样支持 API / WASM 对照。
- 字节大小格式化：同一份 Rust 逻辑同时暴露给服务端 API 与浏览器内 WASM worker，可直接用于文件大小、下载速度等前端工具实验。
- 字节速率展示：在字节格式化结果上额外输出 `B/s`、`KiB/s`、`MB/s` 等速率文案，方便做下载面板与监控类小工具。

## 本地运行

### 1. 构建 WASM

```bash
cargo install wasm-pack
wasm-pack build wasm --target web --out-dir ../static/wasm
```

说明：`wasm` 子 crate 的输出目录需要放到仓库根目录下的 `static/wasm`，这样前端 worker 才能通过 `/wasm/wasm_lab.js` 加载。

### 2. 启动服务

```bash
cargo run
```

然后访问：<http://localhost:3000>

## 测试

```bash
cargo test
```

## 迭代说明

最近一轮迭代把“文本分析”下沉到了共享 `core` crate，并暴露给 WASM；随后接入了“字节大小格式化”完整 demo；本轮则继续往实用方向推进一步：补齐了字节速率展示。

- 服务端 `/api/bytes/format` 现在同时返回大小文案与速率文案；
- WASM worker 的 `format_bytes` 已同步输出 `binary_per_second` / `decimal_per_second`；
- `core::bytes::format_byte_size` 新增速率格式化能力，并补了对应测试；
- 这样同一份 Rust 逻辑就能直接复用于文件工具、下载面板、吞吐监控等小型 demo。
