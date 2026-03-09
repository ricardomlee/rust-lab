# rust-lab

一个用于探索 Rust + WASM 能力边界的小型实验场。

## 当前实验

- 文本分析：同一份输入分别走服务端 Rust API 与浏览器内 WASM worker，比较结果与耗时。
- 进制转换：支持 2–36 进制与负数，同样支持 API / WASM 对照。
- 字节大小格式化：同一份 Rust 逻辑同时暴露给服务端 API 与浏览器内 WASM worker，可直接用于文件大小、下载速度等前端工具实验。
- 字节速率展示：在字节格式化结果上额外输出 `B/s`、`KiB/s`、`MB/s` 等速率文案，方便做下载面板与监控类小工具。
- 时长格式化：输入毫秒数，同时得到紧凑写法、时钟写法与中文可读文案，适合任务耗时、倒计时、日志与作业面板。

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

最近几轮迭代逐步把文本分析、字节格式化等能力下沉到了共享 `core` crate，并同步暴露给服务端 API 与浏览器内 WASM。本轮继续加一块更偏前端实用的小工具：时长格式化。

- 新增 `core::duration::format_duration`，统一输出 `compact` / `clock` / `verbose_zh` 三种格式；
- 新增服务端接口 `/api/duration/format`；
- WASM 导出 `format_duration`，worker 与前端对照基准已接好；
- Demo 面板新增“时长格式化”实验，可直接比较 API 与 WASM 结果与耗时；
- 补了覆盖 0ms、子秒、混合时长、多天时长的单元测试。
