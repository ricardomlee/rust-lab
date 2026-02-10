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

let current = null;

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
  } catch {
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
