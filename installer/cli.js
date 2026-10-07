#!/usr/bin/env node
'use strict';
/** Headless driver: what the Electron UI calls, and what the tests exercise. */
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const { execFileSync } = require('node:child_process');
const { detectAll } = require('./lib/harness');
const I = require('./lib/install');

function arg(name, def) {
  const i = process.argv.indexOf("--" + name);
  return i >= 0 && process.argv[i + 1] && !process.argv[i + 1].startsWith("--") ? process.argv[i + 1] : def;
}
const flag = (name) => process.argv.includes("--" + name);

/** Resolve --from into { dir, manifest } where dir holds the artifacts. */
function resolveSource(from, manifestHint) {
  if (!from) throw new Error("--from is required (a release dir, a zip, or a github release)");

  // GitHub: "owner/repo", "owner/repo@tag", or a release URL.
  const gh = /^(?:https:\/\/github\.com\/)?([\w.-]+\/[\w.-]+?)(?:@([\w.\/-]+)|\/releases\/tag\/([\w.\/-]+))?$/.exec(from.trim());
  if (gh && !from.endsWith(".zip")) {
    const repo = gh[1];
    const tag = gh[2] || gh[3];
    const api = "https://api.github.com/repos/" + repo + "/releases/" + (tag ? "tags/" + tag : "latest");
    console.log("resolving github release:", api);
    const json = execFileSync("curl.exe", ["-sL", "--fail",
      "-H", "Accept: application/vnd.github+json", api], { encoding: "utf8", maxBuffer: 64 << 20 });
    const release = JSON.parse(json);
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), "sc2agent-gh-"));
    for (const asset of release.assets || []) {
      if (!/-(core|data)\.zip$|-manifest\.json$/.test(asset.name)) continue;
      console.log("  downloading", asset.name);
      execFileSync("curl.exe", ["-sL", "--fail", "-o", path.join(dir, asset.name), asset.browser_download_url],
        { stdio: "inherit" });
    }
    return resolveSource(dir);
  }

  if (/^https?:\/\//.test(from)) {
    const dest = fs.mkdtempSync(path.join(os.tmpdir(), "sc2agent-dl-"));
    const target = path.join(dest, path.basename(new URL(from).pathname) || "release.zip");
    console.log("downloading", from);
    execFileSync("curl.exe", ["-sL", "--fail", "-o", target, from], { stdio: "inherit" });
    return resolveSource(target, manifestHint);
  }

  const stat = fs.statSync(from);
  if (stat.isDirectory()) {
    const manifest = fs.readdirSync(from).find((f) => f.endsWith("-manifest.json"));
    if (!manifest) throw new Error("no *-manifest.json in " + from);
    return { dir: from, manifest: JSON.parse(fs.readFileSync(path.join(from, manifest), "utf8")) };
  }

  if (from.endsWith(".zip")) {
    // A lone core.zip from a release carries its own descriptor; a sibling
    // manifest next to it wins because it also knows the data pack hash.
    const sibling = from.replace(/-core\.zip$/, "-manifest.json");
    if (manifestHint && fs.existsSync(manifestHint)) {
      const dest0 = fs.mkdtempSync(path.join(os.tmpdir(), "sc2agent-src-"));
      I.extractZip(from, dest0);
      return { dir: path.dirname(from), manifest: JSON.parse(fs.readFileSync(manifestHint, "utf8")), extracted: dest0 };
    }
    const dest = fs.mkdtempSync(path.join(os.tmpdir(), "sc2agent-src-"));
    I.extractZip(from, dest);
    const embedded = path.join(dest, "sc2agent-release.json");
    const outer = fs.existsSync(sibling) ? JSON.parse(fs.readFileSync(sibling, "utf8"))
      : (fs.existsSync(embedded) ? JSON.parse(fs.readFileSync(embedded, "utf8"))
         : (manifestHint && fs.existsSync(manifestHint) ? JSON.parse(fs.readFileSync(manifestHint, "utf8")) : null));
    if (!outer) throw new Error("no manifest: put *-manifest.json next to the zip, pass --manifest, or use a release dir");
    // The embedded descriptor has no core hash (it cannot contain its own); fill it in.
    if (!outer.artifacts) outer.artifacts = {};
    if (!outer.artifacts.core) {
      outer.artifacts.core = { file: path.basename(from), sha256: I.sha256File(from), required: true };
    }
    // core.zip has already been expanded; point the planner at the extracted tree.
    outer.files.core = {};
    return { dir: path.dirname(from), manifest: outer, extracted: dest };
  }
  throw new Error("unsupported --from: " + from);
}

function plan(source, root, state) {
  const { manifest } = source;
  const kitPath = path.join(root, I.KIT_DIR);
  const core = I.diffFiles(kitPath, manifest.files.core);
  const data = I.diffFiles(kitPath, manifest.files.data);
  const dataPack = manifest.artifacts.data;
  const dataIdentical = data.total > 0 && data.missing.length === 0 && data.stale.length === 0;
  return {
    version: manifest.version,
    layoutRevision: manifest.layout_revision,
    kitPath,
    core,
    data,
    dataIdentical,
    // The whole point of the split: an unchanged data pack is never re-extracted.
    skipData: dataIdentical && dataPack && dataPack.sha256 && state
      && state.artifacts && state.artifacts.data === dataPack.sha256,
    skills: manifest.skills || [],
  };
}

function run() {
  const cmd = process.argv[2] || "detect";
  const root = arg("root", I.defaultInstallRoot());
  const harnessArg = arg("harness", null);
  const from = arg("from", null);
  const dryRun = flag("dry-run");
  const banner = !flag("no-banner");

  if (cmd === "detect") {
    for (const h of detectAll()) {
      console.log((h.installed ? "[x]" : "[ ]"), h.id.padEnd(9), h.skillsDir, "existing skills:", h.installedSkillCount);
    }
    console.log("install root:", root);
    return 0;
  }

  const source = resolveSource(from);
  const state = I.readState(root);
  const p = plan(source, root, state);

  if (cmd === "plan") {
    console.log("version          ", p.version, "(layout", p.layoutRevision + ")");
    console.log("install root     ", root);
    console.log("core             ", p.core.total, "files:", p.core.same, "ok,", p.core.stale.length, "stale,", p.core.missing.length, "missing");
    console.log("data             ", p.data.total, "files:", p.data.same, "ok,", p.data.stale.length, "stale,", p.data.missing.length, "missing");
    console.log("data pack        ", p.dataIdentical ? "already identical — will be SKIPPED" : "needs applying");
    console.log("skills           ", p.skills.length);
    return 0;
  }

  if (cmd !== "install") throw new Error("unknown command: " + cmd);

  const wanted = harnessArg ? harnessArg.split(",").map((s) => s.trim()) : null;
  const targets = detectAll().filter((h) => h.installed && (!wanted || wanted.includes(h.id)));
  console.log("harnesses       ", targets.map((t) => t.id).join(", ") || "(none detected)");
  if (dryRun) { console.log("dry run — nothing written"); return 0; }

  const kits = path.join(root, I.KIT_DIR);
  const stage = path.join(root, ".stage-" + Date.now());
  fs.rmSync(stage, { recursive: true, force: true });
  fs.mkdirSync(stage, { recursive: true });

  console.log("extracting core …");
  I.extractZip(path.join(source.dir, source.manifest.artifacts.core.file), stage);
  for (const rel of fs.readdirSync(stage)) {
    fs.rmSync(path.join(kits, rel), { recursive: true, force: true });
  }
  I.copyDir(stage, kits);
  fs.rmSync(stage, { recursive: true, force: true });

  const dataPack = source.manifest.artifacts.data;
  if (dataPack && dataPack.file && !p.skipData) {
    const dataStage = path.join(root, ".stage-data-" + Date.now());
    console.log("extracting data (" + (dataPack.uncompressed_bytes / 1048576).toFixed(1) + " MB) …");
    I.extractZip(path.join(source.dir, dataPack.file), dataStage);
    I.copyDir(dataStage, kits);
    fs.rmSync(dataStage, { recursive: true, force: true });
  } else if (dataPack && p.dataIdentical) {
    console.log("data pack unchanged — skipped (" + (dataPack.uncompressed_bytes / 1048576).toFixed(1) + " MB not rewritten)");
  }

  const linked = [];
  for (const t of targets) {
    fs.mkdirSync(t.skillsDir, { recursive: true });
    for (const s of p.skills) {
      const src = path.join(kits, s.source);
      if (!fs.existsSync(src)) { console.log("  missing skill source:", s.source); continue; }
      I.installSkill(src, t.skillsDir, s.name, kits, banner);
    }
    linked.push({ id: t.id, skillsDir: t.skillsDir, skills: p.skills.length });
    console.log("linked " + p.skills.length + " skills -> " + t.skillsDir);
  }

  I.writeState(root, {
    version: p.version,
    layoutRevision: p.layoutRevision,
    kit: kits,
    installedUtc: new Date().toISOString(),
    artifacts: {
      core: source.manifest.artifacts.core.sha256,
      data: dataPack ? dataPack.sha256 : null,
    },
    harnesses: linked,
  });
  console.log("wrote", path.join(root, I.STATE_FILE));
  return 0;
}

try { process.exit(run()); } catch (e) { console.error("ERROR:", e.message); process.exit(1); }