import {
  Alert, Button, Card, Checkbox, Col, Flex, Input, Layout, Progress, Row, Space, Tag, Typography,
} from "antd";
import { useEffect, useState } from "react";
import * as api from "./api";

const { Text, Title } = Typography;

const cardTitle = (n: number, t: string) => (
  <Space size={8}>
    <Text style={{ color: "#8b7cf0", fontWeight: 700 }}>{n}</Text>
    <Text strong>{t}</Text>
  </Space>
);

function DiffLine({ label, d }: { label: string; d: api.Diff }) {
  return (
    <Text type="secondary" style={{ fontSize: 13 }}>
      {label}：{d.total} 个文件（{d.same} 一致，{d.stale} 待更新，{d.missing} 缺失）
    </Text>
  );
}

export function App() {
  const [det, setDet] = useState<api.Detect | null>(null);
  const [source, setSource] = useState("");
  const [proxy, setProxy] = useState("");
  const [useMirrors, setUseMirrors] = useState(true);
  const [root, setRoot] = useState("");
  const [picked, setPicked] = useState<string[]>([]);
  const [busy, setBusy] = useState(false);
  const [phase, setPhase] = useState("");
  const [done, setDone] = useState(0);
  const [total, setTotal] = useState(0);
  const [current, setCurrent] = useState("");
  const [log, setLog] = useState<string[]>([]);
  const [plan, setPlan] = useState<api.PlanInfo | null>(null);
  const [summary, setSummary] = useState<api.Summary | null>(null);
  const [err, setErr] = useState("");
  const [update, setUpdate] = useState<api.ReleaseInfo | null>(null);
  const [updateMsg, setUpdateMsg] = useState("");

  useEffect(() => {
    api.detect().then((d) => {
      setDet(d);
      setRoot(d.root);
      if (d.proxy) setProxy(d.proxy);
      // 顺带查一次安装器自身有没有新版；失败静默，用户没要求检查。
      api
        .selfCheck({ proxy: d.proxy ?? "", useMirrors: true })
        .then((r) => r && setUpdate(r))
        .catch(() => {});
    });
    const sub = api.onProgress((m) => {
      if ("Phase" in m) {
        setPhase(m.Phase);
        setCurrent("");
      } else if ("Total" in m) setTotal(m.Total);
      else if ("Step" in m) {
        setDone(m.Step.done);
        setTotal(m.Step.total);
        setCurrent(m.Step.label);
      } else if ("Log" in m) setLog((l) => [...l, m.Log]);
    });
    return () => {
      sub.then((f) => f());
    };
  }, []);

  const opts: api.NetOpts = { proxy, useMirrors };

  async function run<T>(f: () => Promise<T>) {
    setBusy(true);
    setErr("");
    setLog([]);
    try {
      return await f();
    } catch (e) {
      setErr(String(e));
      return undefined;
    } finally {
      setBusy(false);
      setPhase("");
    }
  }

  const preview = () =>
    run(async () => {
      setSummary(null);
      const p = await api.plan(source, root, opts);
      setPlan(p);
    });

  const doInstall = () =>
    run(async () => {
      setPlan(null);
      setDone(0);
      setTotal(0);
      const s = await api.install(source, root, opts, picked);
      setSummary(s);
    });

  const doUpdate = () =>
    run(async () => {
      setUpdateMsg("正在下载新版本…");
      try {
        const p = await api.selfUpdate(opts);
        setUpdateMsg("已替换为 " + p + "，重启后生效。");
      } catch (e) {
        setUpdateMsg("更新失败：" + String(e));
      }
    });

  const pct = total > 0 ? Math.floor((done / total) * 100) : 0;

  return (
    <Layout style={{ height: "100vh" }}>
      {/* antd 的 Header 默认固定 64px 且 overflow:hidden，装不下标题+副标题会被裁掉。 */}
      <Layout.Header
        style={{
          height: "auto",
          lineHeight: 1.5,
          padding: "14px 24px",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
        }}
      >
        <div>
          <Title level={4} style={{ margin: 0 }}>开发套件安装器</Title>
          <Text type="secondary" style={{ fontSize: 12 }}>
            安装开发套件，并把技能交付给你选择的 AI 助手
          </Text>
        </div>
        <Tag color="#8b7cf0">v{det?.version ?? "…"}</Tag>
      </Layout.Header>

      <Layout.Content style={{ overflow: "auto", padding: 20 }}>
        <Flex vertical gap={14} style={{ maxWidth: 980, margin: "0 auto" }}>
          {update && (
            <Alert
              type="info"
              showIcon
              message={"安装器有新版本 " + update.version + "（当前 " + det?.version + "）"}
              description={updateMsg || update.asset_name}
              action={<Button type="primary" disabled={busy} onClick={doUpdate}>安装更新</Button>}
            />
          )}

          <Card title={cardTitle(1, "来源")}>
            <Flex vertical gap={10}>
              <Flex gap={8}>
                <Input
                  value={source}
                  onChange={(e) => setSource(e.target.value)}
                  placeholder="owner/repo@tag，或本地发行目录 / zip"
                />
                <Button onClick={async () => { const p = await api.pickFolder(); if (p) setSource(p); }}>选目录</Button>
                <Button onClick={async () => { const p = await api.pickZip(); if (p) setSource(p); }}>选 zip</Button>
              </Flex>
              <Flex gap={8} align="center">
                <Text type="secondary" style={{ whiteSpace: "nowrap" }}>代理</Text>
                <Input
                  value={proxy}
                  onChange={(e) => setProxy(e.target.value)}
                  placeholder="留空则自动探测（环境变量 → Windows 系统代理）"
                  style={{ maxWidth: 320 }}
                />
                <Text type="secondary" style={{ fontSize: 12 }}>
                  {det?.proxy_source ? "已探测到：" + det.proxy_source : ""}
                </Text>
              </Flex>
              <Flex gap={8} align="center">
                <Checkbox checked={useMirrors} onChange={(e) => setUseMirrors(e.target.checked)}>
                  镜像加速
                </Checkbox>
                <Text type="secondary" style={{ fontSize: 12 }}>
                  直连 + 若干 GitHub 镜像同时开跑，谁先完成用谁
                </Text>
              </Flex>
            </Flex>
          </Card>

          <Card title={cardTitle(2, "目标 harness")}>
            <Text type="secondary" style={{ fontSize: 12 }}>
              默认不勾选，请自己决定交付到哪里。只有检测到的才能勾。
            </Text>
            <Row gutter={[10, 10]} style={{ marginTop: 10 }}>
              {det?.harnesses.map((h) => (
                <Col span={12} key={h.id}>
                  <Card size="small" style={{ opacity: h.installed ? 1 : 0.45 }}>
                    <Checkbox
                      disabled={!h.installed}
                      checked={picked.includes(h.id)}
                      onChange={(e) =>
                        setPicked((p) => (e.target.checked ? [...p, h.id] : p.filter((x) => x !== h.id)))
                      }
                    >
                      <Text strong>{h.name}</Text>
                      {!h.installed && <Text type="secondary">（未安装）</Text>}
                    </Checkbox>
                    <div><Text type="secondary" style={{ fontSize: 11 }}>{h.skills_dir}</Text></div>
                    {h.installed && (
                      <div><Text type="secondary" style={{ fontSize: 11 }}>现有 {h.existing_skills} 个技能</Text></div>
                    )}
                  </Card>
                </Col>
              ))}
            </Row>
          </Card>

          <Card title={cardTitle(3, "安装位置")}>
            <Flex gap={8} align="center">
              <Text type="secondary" style={{ fontSize: 12 }}>{root}</Text>
              <Button size="small" onClick={async () => { const p = await api.pickFolder(); if (p) setRoot(p); }}>更改</Button>
            </Flex>
            <div><Text type="secondary" style={{ fontSize: 11 }}>
              套件只装一份；各 harness 只收到技能，已拍平为 &lt;技能目录&gt;/&lt;名称&gt;/
            </Text></div>
          </Card>

          <Card title={cardTitle(4, "预演与进度")}>
            {busy && (
              <>
                <Progress percent={pct} status="active" />
                <Text type="secondary" style={{ fontSize: 12 }}>
                  {phase}　{total > 0 ? done + " / " + total + "　" + current : ""}
                </Text>
              </>
            )}
            {!busy && !plan && !summary && !err && (
              <Text type="secondary">填好来源后点「预演」看会改什么，或直接「开始安装」。</Text>
            )}
            {!busy && plan && (
              <Flex vertical gap={4}>
                <Text>版本 {plan.version}（layout {plan.layout_revision}）　技能 {plan.skills} 个</Text>
                <DiffLine label="套件本体" d={plan.core} />
                <DiffLine label="游戏数据" d={plan.data} />
                <Text type={plan.skip_data ? "success" : "warning"}>
                  {plan.skip_data
                    ? "数据包哈希一致 —— 整体跳过，不重写 " + plan.data_mb.toFixed(1) + " MB"
                    : "数据包需要应用（" + plan.data_mb.toFixed(1) + " MB）"}
                </Text>
              </Flex>
            )}
            {!busy && summary && (
              <Flex vertical gap={4}>
                <Text type="success">安装完成：版本 {summary.version}</Text>
                <Text type="secondary" style={{ fontSize: 12 }}>套件位于 {summary.kit}</Text>
                {summary.linked.map(([id, dir, n]) => (
                  <Text key={id} type="secondary" style={{ fontSize: 12 }}>{id} → {dir}（{n} 个技能）</Text>
                ))}
                {summary.data_skipped && <Text type="success" style={{ fontSize: 12 }}>游戏数据未变化，已跳过</Text>}
              </Flex>
            )}
            {err && <Alert type="error" showIcon message={err} />}
            {log.length > 0 && (
              <pre style={{
                marginTop: 12, maxHeight: 180, overflow: "auto", fontSize: 11,
                color: "#6a6a8b", background: "#11111e", padding: 10, borderRadius: 8,
              }}>
                {log.join("\n")}
              </pre>
            )}
          </Card>
        </Flex>
      </Layout.Content>

      <Layout.Footer style={{ padding: "12px 24px", display: "flex", alignItems: "center", gap: 10 }}>
        <Button disabled={busy || !source} onClick={preview}>预演</Button>
        <Button type="primary" disabled={busy || !source} onClick={doInstall}>开始安装</Button>
        <div style={{ marginLeft: "auto" }}>
          {busy && <Text type="secondary">{phase}</Text>}
          {!busy && summary && <Text type="success">安装完成</Text>}
        </div>
      </Layout.Footer>
    </Layout>
  );
}
