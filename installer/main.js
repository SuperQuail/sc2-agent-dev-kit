'use strict';
/** Electron main process: the UI drives the same logic the CLI exercises. */
const { app, BrowserWindow, ipcMain, dialog, shell } = require('electron');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { execFileSync, spawn } = require('node:child_process');
const { detectAll } = require('./lib/harness');
const I = require('./lib/install');

let win = null;

function createWindow() {
  win = new BrowserWindow({
    width: 980, height: 720, minWidth: 820, minHeight: 600,
    title: "StarCraftIIAgent 安装器",
    backgroundColor: "#12161d",
    autoHideMenuBar: true,
    webPreferences: { preload: path.join(__dirname, "preload.js"), contextIsolation: true, nodeIntegration: false },
  });
  win.loadFile(path.join(__dirname, "renderer", "index.html"));
}

/** Resolve a source the same way the CLI does, but report progress to the renderer. */
function resolveSource(from, send) {
  if (/^https?:\/\/|^[\w.-]+\/[\w.-]+(@|$)/.test(from.trim()) && !from.endsWith(".zip")) {
    const gh = /^(?:https:\/\/github\.com\/)?([\w.-]+\/[\w.-]+?)(?:@([\w.\/-]+)|\/releases\/tag\/([\w.\/-]+))?$/.exec(from.trim());
    const repo = gh[1];
    const tag = gh[2] || gh[3];
    const api = "https://api.github.com/repos/" + repo + "/releases/" + (tag ? "tags/" + tag : "latest");
    send("解析 GitHub release: " + api);
    const json = execFileSync("curl.exe", ["-sL", "--fail", "-H", "Accept: application/vnd.github+json", api],
      { encoding: "utf8", maxBuffer: 64 << 20 });
    const release = JSON.parse(json);
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), "sc2agent-gh-"));
    for (const asset of release.assets || []) {
      if (!/-(core|data)\.zip$|-manifest\.json$/.test(asset.name)) continue;
      send("下载 " + asset.name + " (" + (asset.size / 1048576).toFixed(1) + " MB)");
      execFileSync("curl.exe", ["-sL", "--fail", "-o", path.join(dir, asset.name), asset.browser_download_url]);
    }
    return resolveSource(dir, send);
  }
  if (/^https?:\/\//.test(from)) {
    const dest = fs.mkdtempSync(path.join(os.tmpdir(), "sc2agent-dl-"));
    const target = path.join(dest, path.basename(new URL(from).pathname) || "release.zip");
    send("下载 " + from);
    execFileSync("curl.exe", ["-sL", "--fail", "-o", target, from]);
    return resolveSource(target, send);
  }
  const stat = fs.statSync(from);
  if (stat.isDirectory()) {
    const m = fs.readdirSync(from).find((f) => f.endsWith("-manifest.json"));
    if (!m) throw new Error("目录里没有 *-manifest.json: " + from);
    return { dir: from, manifest: JSON.parse(fs.readFileSync(path.join(from, m), "utf8")) };
  }
  if (from.endsWith(".zip")) {
    const dest = fs.mkdtempSync(path.join(os.tmpdir(), "sc2agent-src-"));
    I.extractZip(from, dest);
    const embedded = path.join(dest, "sc2agent-release.json");
    if (!fs.existsSync(embedded)) throw new Error("压缩包内没有 sc2agent-release.json，请改选发行目录");
    const outer = JSON.parse(fs.readFileSync(embedded, "utf8"));
    outer.artifacts = outer.artifacts || {};
    outer.artifacts.core = { file: path.basename(from), sha256: I.sha256File(from), required: true };
    outer.files.core = {};
    return { dir: path.dirname(from), manifest: outer, extracted: dest };
  }
  throw new Error("不支持的来源: " + from);
}

ipcMain.handle("detect", () => ({ harnesses: detectAll(), root: I.defaultInstallRoot(), state: I.readState(I.defaultInstallRoot()) }));

ipcMain.handle("pick-directory", async () => {
  const r = await dialog.showOpenDialog(win, { properties: ["openDirectory"] });
  return r.canceled ? null : r.filePaths[0];
});
ipcMain.handle("pick-archive", async () => {
  const r = await dialog.showOpenDialog(win, { properties: ["openFile"], filters: [{ name: "发行包", extensions: ["zip"] }] });
  return r.canceled ? null : r.filePaths[0];
});

ipcMain.handle("plan", (_e, { from, root }) => {
  const send = () => {};
  const source = resolveSource(from, send);
  const kitPath = path.join(root, I.KIT_DIR);
  const core = I.diffFiles(kitPath, source.manifest.files.core);
  const data = I.diffFiles(kitPath, source.manifest.files.data);
  const dataPack = source.manifest.artifacts && source.manifest.artifacts.data;
  const dataIdentical = data.total > 0 && data.missing.length === 0 && data.stale.length === 0;
  const state = I.readState(root);
  return {
    version: source.manifest.version,
    layoutRevision: source.manifest.layout_revision,
    skills: (source.manifest.skills || []).length,
    core: { total: core.total, same: core.same, stale: core.stale.length, missing: core.missing.length },
    data: { total: data.total, same: data.same, stale: data.stale.length, missing: data.missing.length,
            mb: dataPack ? +(dataPack.uncompressed_bytes / 1048576).toFixed(1) : 0 },
    skipData: dataIdentical && dataPack && state && state.artifacts && state.artifacts.data === dataPack.sha256,
    dataIdentical,
    kitPath,
  };
});

ipcMain.handle("summary", () => {
  const root = I.defaultInstallRoot();
  const state = I.readState(root);
  if (!state) return null;
  const kitPath = path.join(root, I.KIT_DIR);
  const exists = fs.existsSync(kitPath);
  return { ...state, kitExists: exists, harnesses: state.harnesses || [] };
});

ipcMain.handle("open-path", (_e, p) => shell.openPath(p));

ipcMain.handle("run-install", async (e, { from, root, harnessIds, skipBanner }) => {
  const send = (line) => e.sender.send("log", line);
  try {
    const source = resolveSource(from, send);
    const plan = { manifest: source.manifest };
    const targets = detectAll().filter((h) => h.installed && (!harnessIds || harnessIds.includes(h.id)));
    send("目标 harness：" + (targets.map((t) => t.id).join(", ") || "（无）"));

    const kitPath = path.join(root, I.KIT_DIR);
    const dataBefore = I.diffFiles(kitPath, source.manifest.files.data);
    const dataPack = source.manifest.artifacts && source.manifest.artifacts.data;
    const state = I.readState(root);
    const skipData = dataBefore.total > 0 && dataBefore.missing.length === 0 && dataBefore.stale.length === 0
      && dataPack && state && state.artifacts && state.artifacts.data === dataPack.sha256;

    send("解压核心包 …");
    const stage = path.join(root, ".stage-" + Date.now());
    fs.rmSync(stage, { recursive: true, force: true });
    fs.mkdirSync(stage, { recursive: true });
    I.extractZip(path.join(source.dir, source.manifest.artifacts.core.file), stage);
    fs.mkdirSync(kitPath, { recursive: true });
    for (const rel of fs.readdirSync(stage)) fs.rmSync(path.join(kitPath, rel), { recursive: true, force: true });
    I.copyDir(stage, kitPath);
    fs.rmSync(stage, { recursive: true, force: true });

    if (dataPack && dataPack.file && !skipData) {
      send("解压数据包（" + (dataPack.uncompressed_bytes / 1048576).toFixed(1) + " MB）…");
      const ds = path.join(root, ".stage-data-" + Date.now());
      I.extractZip(path.join(source.dir, dataPack.file), ds);
      I.copyDir(ds, kitPath);
      fs.rmSync(ds, { recursive: true, force: true });
    } else if (dataPack) {
      send("数据包未变化，跳过（" + (dataPack.uncompressed_bytes / 1048576).toFixed(1) + " MB 未重写）");
    }

    const linked = [];
    for (const t of targets) {
      fs.mkdirSync(t.skillsDir, { recursive: true });
      let n = 0;
      for (const s of source.manifest.skills || []) {
        const src = path.join(kitPath, s.source);
        if (!fs.existsSync(src)) continue;
        I.installSkill(src, t.skillsDir, s.name, kitPath, !skipBanner);
        n += 1;
      }
      linked.push({ id: t.id, skillsDir: t.skillsDir, skills: n });
      send("已链接 " + n + " 个技能 -> " + t.skillsDir);
    }

    I.writeState(root, {
      version: source.manifest.version,
      layoutRevision: source.manifest.layout_revision,
      kit: kitPath,
      installedUtc: new Date().toISOString(),
      artifacts: { core: source.manifest.artifacts.core.sha256, data: dataPack ? dataPack.sha256 : null },
      harnesses: linked,
    });
    send("完成。安装清单：" + path.join(root, I.STATE_FILE));
    return { ok: true, linked };
  } catch (err) {
    send("错误：" + err.message);
    return { ok: false, error: err.message };
  }
});

const SMOKE = process.argv.includes('--smoke');

app.whenReady().then(async () => {
  createWindow();
  if (!SMOKE) return;
  // Headless verification: prove main, preload, IPC and the renderer all load,
  // then leave no window behind.  Never launches a harness.
  const fail = (why) => { console.error('SMOKE FAIL: ' + why); app.exit(1); };
  win.webContents.on('did-fail-load', (_e, code, desc) => fail('renderer did-fail-load ' + code + ' ' + desc));
  win.webContents.on('preload-error', (_e, p, err) => fail('preload ' + p + ': ' + err.message));
  win.webContents.on('did-finish-load', async () => {
    try {
      const out = await win.webContents.executeJavaScript("(" + (async () => {
        const harnesses = await window.sc2.detect();
        const state = await window.sc2.summary();
        return {
          bridged: typeof window.sc2.runInstall === 'function',
          harnesses: harnesses.harnesses.map((h) => h.id + (h.installed ? '+' : '-')).join(','),
          root: harnesses.root,
          installedVersion: state ? state.version : null,
          logLines: document.getElementById('log').textContent.split('\n').filter(Boolean).length,
        };
      }).toString() + ")()");
      console.log('SMOKE OK ' + JSON.stringify(out));
      app.exit(0);
    } catch (err) { fail(err.message); }
  });
  setTimeout(() => fail('timed out'), 25000);
});
app.on('window-all-closed', () => { if (process.platform !== 'darwin') app.quit(); });
app.on('activate', () => { if (BrowserWindow.getAllWindows().length === 0) createWindow(); });
