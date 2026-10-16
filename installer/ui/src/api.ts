import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";

// 后端类型（Rust 侧 derive(Serialize) 的结构，字段名原样对应）。
export type Harness = {
  id: string;
  name: string;
  skills_dir: string;
  installed: boolean;
  existing_skills: number;
};

export type Detect = {
  version: string;
  root: string;
  proxy: string | null;
  proxy_source: string | null;
  harnesses: Harness[];
};

export type Diff = { same: number; stale: number; missing: number; total: number };

export type PlanInfo = {
  version: string;
  layout_revision: number;
  skills: number;
  core: Diff;
  data: Diff;
  data_mb: number;
  skip_data: boolean;
  kit_path: string;
};

export type Summary = {
  version: string;
  kit: string;
  linked: [string, string, number][];
  data_skipped: boolean;
};

export type ReleaseInfo = {
  tag: string;
  version: string;
  asset_name: string;
  asset_url: string;
};

// Msg 是 Rust 的外部标记枚举，序列化成 { Phase: "…" } 这种形状。
export type Msg =
  | { Phase: string }
  | { Total: number }
  | { Step: { done: number; total: number; label: string } }
  | { Log: string }
  | { PlanReady: PlanInfo }
  | { Done: { Ok: Summary } | { Err: string } };

export type NetOpts = { proxy: string; useMirrors: boolean };

const net = ({ proxy, useMirrors }: NetOpts) => ({
  proxy: proxy.trim() || null,
  useMirrors,
});

export const detect = () => invoke<Detect>("detect");

export const plan = (from: string, root: string, o: NetOpts) =>
  invoke<PlanInfo>("plan", { from, root, ...net(o) });

export const install = (from: string, root: string, o: NetOpts, harnesses: string[]) =>
  invoke<Summary>("install", { from, root, ...net(o), harnesses });

export const selfCheck = (o: NetOpts) =>
  invoke<ReleaseInfo | null>("self_check", net(o));

export const selfUpdate = (o: NetOpts) => invoke<string>("self_update", net(o));

// 文件选择走官方 dialog 插件，不自己在 Rust 里阻塞主线程。
export const pickFolder = () => open({ directory: true }) as Promise<string | null>;
export const pickZip = () =>
  open({ filters: [{ name: "发行包", extensions: ["zip"] }] }) as Promise<string | null>;

export const onProgress = (cb: (m: Msg) => void) =>
  listen<Msg>("install://progress", (e) => cb(e.payload));
