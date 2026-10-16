import { ConfigProvider } from "antd";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { appTheme } from "./theme";
import "./styles.css";

const el = document.getElementById("app");
if (!el) throw new Error("找不到挂载点 #app");

createRoot(el).render(
  <ConfigProvider theme={appTheme}>
    <App />
  </ConfigProvider>,
);
