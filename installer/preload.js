'use strict';
const { contextBridge, ipcRenderer } = require('electron');
contextBridge.exposeInMainWorld("sc2", {
  detect: () => ipcRenderer.invoke("detect"),
  plan: (a) => ipcRenderer.invoke("plan", a),
  summary: () => ipcRenderer.invoke("summary"),
  runInstall: (a) => ipcRenderer.invoke("run-install", a),
  pickDirectory: () => ipcRenderer.invoke("pick-directory"),
  pickArchive: () => ipcRenderer.invoke("pick-archive"),
  openPath: (p) => ipcRenderer.invoke("open-path", p),
  onLog: (cb) => ipcRenderer.on("log", (_e, line) => cb(line)),
});
