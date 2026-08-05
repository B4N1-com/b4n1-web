<div align="center">

# 🌐 B4n1Web — 智能体浏览器引擎

**面向 AI 智能体的超轻量无头浏览器。**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

单个 Rust 二进制文件 · 4 种语言 SDK · 33 个 MCP 工具。
导航 URL、提取结构化内容（Markdown、链接、截图），并为智能体构建自主工作流。

**[📊 项目统计 → STATS.md](STATS.md)**

---

## 🌍 语言

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## 特性

- **33 个 MCP 工具**，用于 AI 智能体集成
- **单个自包含的 Rust 二进制文件**（约 11MB），无运行时依赖
- **4 种语言 SDK**（Python、JS、Java、C#），内置二进制文件
- **静态链接（musl）**——可在任何 Linux 上运行，无需 glibc
- **三种模式**：Light（即时）、JS（脚本）、Render（Chromium）
- **安全护盾**：域名过滤、安全浏览
- **网络拦截**：阻止资源、模拟响应
- **MCP 服务器**：stdio 传输，无需端口

## 浏览器模式

| 模式 | 描述 | 内存 | 启动 |
|------|------|------|------|
| Light | HTTP 抓取 + HTML 解析 | ~15MB | 即时 |
| JS | Light + JavaScript 提取 | ~15MB | 即时 |
| Render | 完整 Chromium + 截图 | ~100MB | ~2 秒 |

## 快速开始

安装二进制文件或使用您偏好的包管理器：

```bash
# 二进制文件（任何 Linux，无依赖）
curl -sL https://b4n1.com/install | bash

# 或通过包管理器
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java：从 Maven Central 添加依赖
```

基本用法：

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### MCP 服务器

```bash
# stdio 模式（默认）
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## SDK 矩阵

| 语言 | 包 | 版本 | 二进制 |
|------|-----|------|--------|
| Python | `b4n1-web` | 0.12.3 | 内置（musl） |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | 内置（musl） |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | 内置（musl） |
| C# (.NET) | `B4n1Web` | 0.12.3 | 内置（musl） |

## 文档

- [📖 完整文档](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCP 工具](https://mcp.so/server/b4n1web/B4N1-com) — MCP 注册表
- [📊 项目统计](STATS.md) — 下载量、版本、发布

## 链接

- 网站：https://b4n1.com
- GitHub：https://github.com/B4N1-com/b4n1-web
- PyPI：https://pypi.org/project/b4n1-web
- npm：https://www.npmjs.com/package/b4n1-web
- NuGet：https://www.nuget.org/packages/B4n1Web
- Maven Central：https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## 许可证

MIT