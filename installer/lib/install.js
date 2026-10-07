'use strict';
const BT = String.fromCharCode(96);   // backtick, kept out of source literals on purpose
/**
 * Install / update the kit and expose its skills to the supported harnesses.
 *
 * Three things this does that a plain "unzip everything" does not:
 *
 *   1. Splits the download. DataEditorXML is ~166 MB of exported game data against
 *      ~10 MB for everything else. It ships as its own artifact and is skipped outright
 *      when the installed copy already hashes correctly.
 *   2. Keeps one kit on disk. Harnesses receive only skills, flattened into
 *      <skills-dir>/<name>/, with the kit path written into each SKILL.md so the
 *      documented "python tools/sc2.py" commands resolve.
 *   3. Records what it did, so a later run can answer "is anything stale".
 */
const crypto = require('node:crypto');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { execFileSync } = require('node:child_process');

const STATE_FILE = 'install.json';
const KIT_DIR = 'kit';

function sha256File(file) {
  const h = crypto.createHash('sha256');
  const fd = fs.openSync(file, 'r');
  const buf = Buffer.allocUnsafe(1 << 20);
  try {
    for (;;) {
      const n = fs.readSync(fd, buf, 0, buf.length, null);
      if (n <= 0) break;
      h.update(buf.subarray(0, n));
    }
  } finally {
    fs.closeSync(fd);
  }
  return h.digest("hex");
}

function defaultInstallRoot() {
  // Beside the user's other per-user data, not in a repo: the kit is a tool,
  // the repo is the project being worked on.
  const base = process.env.LOCALAPPDATA || path.join(os.homedir(), 'AppData', 'Local');
  return path.join(base, 'sc2agent');
}

function readState(root) {
  const file = path.join(root, STATE_FILE);
  if (!fs.existsSync(file)) return null;
  try { return JSON.parse(fs.readFileSync(file, "utf8")); } catch { return null; }
}

function writeState(root, state) {
  fs.mkdirSync(root, { recursive: true });
  fs.writeFileSync(path.join(root, STATE_FILE), JSON.stringify(state, null, 1) + "\n", "utf8");
}

/** Expand a zip with the platform tooling, so the installer needs no zip dependency. */
function extractZip(zipPath, dest) {
  fs.mkdirSync(dest, { recursive: true });
  if (process.platform === 'win32') {
    const q = (s) => "'" + String(s).replace(/'/g, "''") + "'";
    execFileSync("powershell.exe", ["-NoProfile", "-NonInteractive", "-Command",
      "Expand-Archive -LiteralPath " + q(zipPath) + " -DestinationPath " + q(dest) + " -Force"],
      { stdio: "pipe" });
    return;
  }
  execFileSync("unzip", ["-oq", zipPath, "-d", dest], { stdio: "pipe" });
}

function copyDir(src, dest) {
  fs.mkdirSync(dest, { recursive: true });
  for (const entry of fs.readdirSync(src, { withFileTypes: true })) {
    const s = path.join(src, entry.name);
    const d = path.join(dest, entry.name);
    if (entry.isDirectory()) copyDir(s, d); else fs.copyFileSync(s, d);
  }
}

/** Which of these files are missing or hash-different under root. */
function diffFiles(root, fileHashes) {
  const missing = [];
  const stale = [];
  let same = 0;
  const entries = Object.entries(fileHashes || {});
  for (const [rel, want] of entries) {
    const target = path.join(root, rel);
    if (!fs.existsSync(target)) { missing.push(rel); continue; }
    if (sha256File(target) === want) same += 1; else stale.push(rel);
  }
  return { missing, stale, same, total: entries.length };
}

function kitNote(kitPath) {
  return "> 本技能由 StarCraftIIAgent 安装器部署。套件根目录：" + BT + kitPath + BT + "\n"
       + "> 文中 " + BT + "python tools/sc2.py ..." + BT + " 命令请在该目录下执行。";
}

/**
 * Copy one skill into a harness, flattened.
 *
 * The note is injected AFTER the frontmatter: a harness parses the leading --- block
 * as YAML, so prepending anything would silently break the skill.
 */
function installSkill(skillSource, skillsDir, skillName, kitPath, banner) {
  const dest = path.join(skillsDir, skillName);
  fs.rmSync(dest, { recursive: true, force: true });
  copyDir(skillSource, dest);
  if (!banner || !kitPath) return dest;
  const skillFile = path.join(dest, "SKILL.md");
  if (!fs.existsSync(skillFile)) return dest;
  const text = fs.readFileSync(skillFile, "utf8");
  if (text.includes(kitPath)) return dest;
  const lines = text.split(/\r?\n/);
  if (lines[0] !== "---") return dest;
  const end = lines.indexOf("---", 1);
  if (end < 0) return dest;
  const out = lines.slice(0, end + 1)
    .concat(["", kitNote(kitPath), ""])
    .concat(lines.slice(end + 1));
  fs.writeFileSync(skillFile, out.join("\n"), "utf8");
  return dest;
}

module.exports = {
  KIT_DIR, STATE_FILE, defaultInstallRoot, readState, writeState, extractZip,
  copyDir, sha256File, diffFiles, installSkill, kitNote,
};
