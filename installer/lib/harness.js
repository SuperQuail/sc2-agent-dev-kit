'use strict';
/**
 * Where each supported harness keeps its skills.
 *
 * Verified on a real machine rather than assumed: every row below was confirmed by
 * listing the directory.  The convention is flat — <skills-dir>/<name>/SKILL.md —
 * so nested kit skills are flattened on install.
 */
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');

const HARNESSES = [
  { id: 'dsh', name: 'DeepSeek Harness',
    detect: () => path.join(os.homedir(), '.dsh'),
    skills: () => path.join(os.homedir(), '.dsh', 'skills') },
  { id: 'claude', name: 'Claude Code',
    detect: () => path.join(os.homedir(), '.claude'),
    skills: () => path.join(os.homedir(), '.claude', 'skills') },
  { id: 'codex', name: 'Codex',
    detect: () => path.join(os.homedir(), '.codex'),
    skills: () => path.join(os.homedir(), '.codex', 'skills') },
  { id: 'opencode', name: 'opencode',
    detect: () => path.join(os.homedir(), '.config', 'opencode'),
    skills: () => path.join(os.homedir(), '.config', 'opencode', 'skills') },
];

function detectAll() {
  return HARNESSES.map((h) => {
    const root = h.detect();
    const skills = h.skills();
    const hasSkillsDir = fs.existsSync(skills);
    return {
      id: h.id,
      name: h.name,
      configRoot: root,
      skillsDir: skills,
      // Present when the config root exists; the skills subdirectory is made on demand.
      installed: fs.existsSync(root),
      skillsDirExists: hasSkillsDir,
      installedSkillCount: hasSkillsDir
        ? fs.readdirSync(skills, { withFileTypes: true }).filter((e) => e.isDirectory()).length
        : 0,
    };
  });
}

module.exports = { HARNESSES, detectAll };
