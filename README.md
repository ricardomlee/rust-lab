# rust-lab

一个用于探索 Rust + WASM 能力边界的小型实验场。

## 当前实验

- 文本分析：同一份输入分别走服务端 Rust API 与浏览器内 WASM worker，比较结果与耗时。
- 进制转换：支持 2–36 进制与负数，同样支持 API / WASM 对照。

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

这一轮把“文本分析”也下沉到了共享 `core` crate，并暴露给 WASM：

- 服务端 API 与 WASM 复用同一套 Rust 逻辑；
- 浏览器端可以直接比较本地 WASM 与服务端 API 的结果/耗时；
- `core` 增加了文本分析单元测试，便于后续继续扩展更多纯计算型实验。
