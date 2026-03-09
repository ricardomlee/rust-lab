# rust-lab

一个用于探索 Rust + WASM 能力边界的小型实验场。

## 当前实验

- 文本分析：同一份输入分别走服务端 Rust API 与浏览器内 WASM worker，比较结果与耗时。
- 进制转换：支持 2–36 进制与负数，同样支持 API / WASM 对照。
- 字节大小格式化：共享 `core` crate 已提供二进制 / 十进制两套格式化结果，可直接暴露给 WASM 侧做文件大小、下载速度等前端工具实验。

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

最近一轮迭代把“文本分析”下沉到了共享 `core` crate，并暴露给 WASM；本轮则补上了一个更偏实用的小能力：字节大小格式化。

- 服务端 API 与 WASM 继续复用同一套 Rust 逻辑；
- `core::bytes::format_byte_size` 同时输出 binary（KiB/MiB）和 decimal（KB/MB）格式；
- 这类能力很适合继续扩展成文件工具、下载面板、性能监控 demo；
- `core` 为该能力增加了单元测试，便于后续继续叠加更多纯计算型实验。
