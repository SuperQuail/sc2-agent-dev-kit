import { theme, type ThemeConfig } from "antd";

// Koishi 的暗色语言：深蓝紫底、圆角卡片、紫色强调。
// 组件外观全部交给 antd，这里只覆盖颜色与圆角，不自己写组件样式。
export const appTheme: ThemeConfig = {
  algorithm: theme.darkAlgorithm,
  token: {
    colorPrimary: "#8b7cf0",
    colorInfo: "#8b7cf0",
    colorBgBase: "#11111e",
    colorBgContainer: "#222237",
    colorBgElevated: "#2a2a43",
    colorBorder: "#2e2e49",
    colorBorderSecondary: "#2e2e49",
    colorText: "#eaeaf8",
    colorTextSecondary: "#9b9bba",
    colorTextTertiary: "#6a6a8b",
    colorSuccess: "#4ade80",
    colorWarning: "#fbbf24",
    colorError: "#f87171",
    borderRadius: 10,
    fontFamily:
      '"Microsoft YaHei", "PingFang SC", "Noto Sans SC", system-ui, sans-serif',
  },
  components: {
    Layout: { bodyBg: "#11111e", headerBg: "#18182b", footerBg: "#18182b" },
    Card: { paddingLG: 18 },
  },
};
