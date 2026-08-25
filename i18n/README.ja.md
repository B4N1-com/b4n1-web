<div align="center">

<img src="b4n1web_presentation.png" alt="B4n1Web" width="100%">

# 🌐 B4n1Web — エージェント型ブラウザエンジン

**AIエージェント向けの超軽量ヘッドレスブラウザ。**

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

単一のRustバイナリ · 4言語SDK · 33のMCPツール。
URLへの移動、構造化コンテンツの抽出（Markdown、リンク、スクリーンショット）、エージェント向けの自律型ワークフロー構築。

</div>

---

## 🌍 Languages / Idiomas / 语言

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](../README.md) | 🇪🇸 [Español](i18n/README.es.md) | 🇫🇷 [Français](i18n/README.fr.md) | 🇩🇪 [Deutsch](i18n/README.de.md) | 🇵🇹 [Português](i18n/README.pt-BR.md) | 🇮🇹 [Italiano](i18n/README.it.md) |
| 🇨🇳 [简体中文](i18n/README.zh-CN.md) | 🇯🇵 [日本語](i18n/README.ja.md) | 🇰🇷 [한국어](i18n/README.ko.md) | 🇷🇺 [Русский](i18n/README.ru.md) | 🇸🇦 [العربية](i18n/README.ar.md) | 🇮🇳 [हिन्दी](i18n/README.hi.md) |

---

## 🖥 🖥 プラットフォーム対応

8つのプリコンパイル済みバイナリ——どこでも動作：

| Platform | Architectures | Binary |
|----------|---------------|--------|
| **Linux** | x86_64, aarch64, i686 | `musl` (static, no glibc) |
| **macOS** | x86_64, arm64 | `universal` |
| **Windows** | x86_64, arm64, i686 | `MSVC` |

## 特徴

- **33 MCP tools** — AIエージェント統合
- **Single self-contained Rust binary** ~11MB, no runtime dependencies
- **4 language SDKs** (Python, JS, Java, C#) with bundled binary
- **Static linking (musl)** — works on any Linux, no glibc required
- **Three modes**: Light (instant), JS (scripts), Render (Chromium)
- **Security shield**: domain filtering, safe browsing
- **Network interception**: block resources, mock responses
- **MCP Server**: stdio transport, no port needed

## ブラウザモード

| モード | 説明 | メモリ | 起動 |
|------|-------------|-----|---------|
| Light | HTTP取得 + HTML解析 | ~15MB | Instant |
| JS | Light + JavaScript抽出 | ~15MB | Instant |
| Render | 完全なChromium + スクリーンショット | ~100MB | ~2s |

## クイックスタート

バイナリをインストールするか、お好みのパッケージマネージャーを使用してください：

```bash
# バイナリ（Linux、macOS、Windows——依存なし）
curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash

# またはパッケージマネージャー経由
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: Maven Centralから依存関係を追加
```

基本的な使用方法：

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### MCPサーバー

```bash
# stdioモード（デフォルト）
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## SDKマトリクス

| 言語 | Package | Version | バイナリ |
|----------|---------|---------|--------|
| Python | `b4n1-web` | 0.12.3 | 同梱（musl） |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | 同梱（musl） |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | 同梱（musl） |
| C# (.NET) | `B4n1Web` | 0.12.3 | 同梱（musl） |

## ドキュメント

- [📖 完全なドキュメント](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCPツール](https://mcp.so/server/b4n1web/B4N1-com) — MCP registry

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## License

Apache License 2.0
