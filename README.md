# rust-lab

一个用于探索 **Rust + WebAssembly** 能力边界的小型实验场。所有实验都在浏览器内通过 WASM 运行，
本地开发时还可以同时和服务端 Rust API 做对照基准。

> 在线体验：通过 GitHub Pages 部署的静态站，即使没有服务端 API 也能完整运行所有实验（自动切换到「WASM only」模式）。

## 当前实验

**文本与编码**
- 文本分析：字符 / 单词 / 行数 + SHA-256
- 进制转换：2–36 进制互转，支持负数
- Base64 编解码：标准 / URL-safe 两种变体
- 哈希计算：SHA-256 / SHA-512
- UUID 生成：RFC4122 v4 与 12 字符短 ID

**数字格式化**
- 字节大小（KiB/MiB/KB/MB + B/s）
- 时长（紧凑写法 / 时钟 / 中文）
- 帧时间（FPS ↔ ms）
- 倒计时拆解（d/h/m/s）
- 百分比格式化

**颜色工具**
- HEX ↔ RGB / HSL，带可视化 swatch
- 颜色混合（线性插值）
- 调色板生成（在两色间均匀采样）

## 本地运行

### 1. 构建 WASM

```bash
cargo install wasm-pack
wasm-pack build wasm --target web --out-dir ../static/wasm
```

`wasm` 子 crate 的输出会落到仓库根目录下的 `static/wasm`，前端 worker 通过相对路径 `./wasm/wasm_lab.js` 加载，
因此同样适用于 GitHub Pages 子路径部署。

### 2. 启动服务

```bash
cargo run
```

然后访问 <http://localhost:3000>。前端会自动探测 `/api/health` —— API 可用时显示「API + WASM」对照，
否则自动切换到「WASM only」模式。

## 测试

```bash
cargo test --manifest-path core/Cargo.toml
```

## 部署到 GitHub Pages

仓库已包含 `.github/workflows/pages.yml` 工作流：

1. 安装 wasm-pack 并构建 wasm（`--target web`），输出到 `static/wasm`
2. 跑一遍 `core` 单测
3. 把整个 `static/` 目录作为 Pages artifact 上传并部署

首次启用：在仓库 **Settings → Pages → Build and deployment** 中把 Source 切到 "GitHub Actions"。

## 架构

```
core/      纯 Rust 共享逻辑（每个工具一个模块，附单元测试）
src/       Axum 服务端，把 core 暴露为 JSON API
wasm/      wasm-bindgen 子 crate，把 core 暴露给浏览器
static/    前端：Tailwind + 原生 JS，所有实验通过 WebWorker 调用 WASM
```

每加一个实验只需：在 `core` 写一个纯 Rust 函数 → 在 `wasm/src/lib.rs` 暴露 `#[wasm_bindgen]` ↔
（可选）在 `src/api/` 加一个 axum handler ↔ 在 `static/app.js` 的 `experiments` 注册表里加一项。
