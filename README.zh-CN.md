<div align="center">

<img src="b4n1web_presentation.png" alt="B4n1Web" width="100%">

# 🌐 B4n1Web — 智能体浏览器引擎

**面向 AI 智能体的超轻量无头浏览器。**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

[![PyPI Downloads/month](https://img.shields.io/pypi/dm/b4n1-web)](https://pypi.org/project/b4n1-web/)
[![npm Downloads/month](https://img.shields.io/npm/dm/b4n1-web)](https://www.npmjs.com/package/b4n1-web)
[![NuGet Downloads](https://img.shields.io/nuget/dt/B4n1Web)](https://www.nuget.org/packages/B4n1Web)
[![GitHub Release](https://img.shields.io/github/v/release/B4N1-com/b4n1-web)](https://github.com/B4N1-com/b4n1-web/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/B4N1-com/b4n1-web/total)](https://github.com/B4N1-com/b4n1-web/releases)

单个 Rust 二进制文件 · 4 种语言 SDK · 33 个 MCP 工具。
导航 URL、提取结构化内容（Markdown、链接、截图），并为智能体构建自主工作流。

---

## 🌍 Languages / Idiomas / 语言

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## 🖥 🖥 平台支持

8 个预编译二进制文件——适用于所有平台：

| Platform | Architectures | Binary |
|----------|---------------|--------|
| **Linux** | x86_64, aarch64, i686 | `musl` (static, no glibc) |
| **macOS** | x86_64, arm64 | `universal` |
| **Windows** | x86_64, arm64, i686 | `MSVC` |

## 特性

- **33 MCP tools** — AI 智能体集成
- **Single self-contained Rust binary** ~11MB, no runtime dependencies
- **4 language SDKs** (Python, JS, Java, C#) with bundled binary
- **Static linking (musl)** — works on any Linux, no glibc required
- **Three modes**: Light (instant), JS (scripts), Render (Chromium)
- **Security shield**: domain filtering, safe browsing
- **Network interception**: block resources, mock responses
- **MCP Server**: stdio transport, no port needed

## 浏览器模式

| 模式 | 描述 | 内存 | 启动 |
|------|-------------|-----|---------|
| Light | HTTP 抓取 + HTML 解析 | ~15MB | Instant |
| JS | Light + JavaScript 提取 | ~15MB | Instant |
| Render | 完整 Chromium + 截图 | ~100MB | ~2s |

## 快速开始

安装二进制文件或使用您偏好的包管理器：

```bash
# 二进制文件（Linux、macOS、Windows——无依赖）
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

| 语言 | Package | Version | 二进制 |
|----------|---------|---------|--------|
| Python | `b4n1-web` | 0.12.3 | 内置（musl） |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | 内置（musl） |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | 内置（musl） |
| C# (.NET) | `B4n1Web` | 0.12.3 | 内置（musl） |

## 文档

- [📖 完整文档](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCP 工具](https://mcp.so/server/b4n1web/B4N1-com) — MCP registry

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## License

Apache License 2.0
