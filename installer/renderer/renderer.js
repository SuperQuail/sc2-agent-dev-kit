'use strict';
const $ = (id) => document.getElementById(id);
let state = { harnesses: [], root: "" };
function log(line) { const el = $("log"); el.textContent += line + "\n"; el.scrollTop = el.scrollHeight; }

function renderHarnesses() {
  const box = $("harnesses"); box.innerHTML = "";
  for (const h of state.harnesses) {
    const div = document.createElement("label");
    div.className = "harness" + (h.installed ? "" : " missing");
    const cb = document.createElement("input");
    cb.type = "checkbox"; cb.value = h.id; cb.checked = h.installed; cb.disabled = !h.installed;
    const text = document.createElement("span");
    const b = document.createElement("b");
    b.textContent = h.name + (h.installed ? "" : "（未安装）");
    const small = document.createElement("small");
    small.textContent = h.skillsDir + (h.installed ? "  ·  现有技能 " + h.installedSkillCount : "");
    text.appendChild(b); text.appendChild(small);
    div.appendChild(cb); div.appendChild(text); box.appendChild(div);
  }
}
const selected = () => [...document.querySelectorAll("#harnesses input:checked")].map((c) => c.value);
const sourceValue = () => $("source").value.trim();

async function preview() {
  const from = sourceValue();
  if (!from) { $("plan").innerHTML = "<span class='warn'>请先填写来源。</span>"; return; }
  $("plan").textContent = "预演中…"; log("预演：" + from);
  try {
    const p = await window.sc2.plan({ from, root: state.root });
    const skip = p.skipData
      ? "<span class='skip'>哈希一致 —— 整体跳过，不重写 " + p.data.mb + " MB</span>"
      : (p.dataIdentical ? "<span class='skip'>已是最新</span>"
         : "<span class='warn'>需要应用（" + p.data.mb + " MB）</span>");
    $("plan").innerHTML = "<table>"
      + "<tr><td>版本</td><td>" + p.version + "（layout " + p.layoutRevision + "）</td></tr>"
      + "<tr><td>技能数</td><td>" + p.skills + "</td></tr>"
      + "<tr><td>核心包</td><td>" + p.core.total + " 文件：" + p.core.same + " 一致，" + p.core.stale + " 待更新，" + p.core.missing + " 缺失</td></tr>"
      + "<tr><td>数据包</td><td>" + p.data.total + " 文件：" + p.data.same + " 一致，" + p.data.stale + " 待更新，" + p.data.missing + " 缺失</td></tr>"
      + "<tr><td>数据包处理</td><td>" + skip + "</td></tr>"
      + "<tr><td>套件路径</td><td><code>" + p.kitPath + "</code></td></tr></table>";
    log("预演完成：核心 " + p.core.total + "，数据 " + p.data.total);
  } catch (e) { $("plan").innerHTML = "<span class='warn'>" + e.message + "</span>"; log("预演失败：" + e.message); }
}

async function install() {
  const from = sourceValue();
  if (!from) { log("请先填写来源。"); return; }
  const harnessIds = selected();
  if (!harnessIds.length) { log("请至少选择一个 harness。"); return; }
  $("install").disabled = true;
  try {
    const r = await window.sc2.runInstall({ from, root: state.root, harnessIds, skipBanner: false });
    if (r.ok) { log("安装完成。"); await refreshSummary(); }
  } finally { $("install").disabled = false; }
}

async function refreshSummary() {
  const s = await window.sc2.summary();
  if (!s) return;
  log("已装版本 " + s.version + "，套件位于 " + s.kit);
  for (const h of s.harnesses || []) log("  " + h.id + " -> " + h.skillsDir + "（" + h.skills + " 个技能）");
}

(async function init() {
  state = await window.sc2.detect();
  $("root").textContent = state.root;
  renderHarnesses();
  $("pick-dir").addEventListener("click", async () => { const p = await window.sc2.pickDirectory(); if (p) $("source").value = p; });
  $("pick-zip").addEventListener("click", async () => { const p = await window.sc2.pickArchive(); if (p) $("source").value = p; });
  $("preview").addEventListener("click", preview);
  $("install").addEventListener("click", install);
  window.sc2.onLog(log);
  await refreshSummary();
  log("检测到 " + state.harnesses.filter((h) => h.installed).length + " 个 harness。");
})();
