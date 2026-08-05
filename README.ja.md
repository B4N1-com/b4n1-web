<div align="center">

# 🌐 B4n1Web — エージェント型ブラウザエンジン

**AIエージェント向けの超軽量ヘッドレスブラウザ。**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

単一のRustバイナリ · 4言語SDK · 33のMCPツール。
URLへの移動、構造化コンテンツの抽出（Markdown、リンク、スクリーンショット）、エージェント向けの自律型ワークフロー構築が可能です。

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 言語

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## 特徴

- **33のMCPツール** — AIエージェント統合用
- **単一の自己完結型Rustバイナリ**（約11MB）、ランタイム依存なし
- **4言語SDK**（Python、JS、Java、C#）— バイナリ同梱
- **静的リンク（musl）** — あらゆるLinuxで動作、glibc不要
- **3つのモード**：Light（即時）、JS（スクリプト）、Render（Chromium）
- **セキュリティシールド**：ドメインフィルタリング、安全な閲覧
- **ネットワークインターセプト**：リソースのブロック、レスポンスのモック
- **MCPサーバー**：stdioトランスポート、ポート不要

## ブラウザモード

| モード | 説明 | メモリ | 起動 |
|--------|------|--------|------|
| Light | HTTP取得 + HTML解析 | ~15MB | 即時 |
| JS | Light + JavaScript抽出 | ~15MB | 即時 |
| Render | 完全なChromium + スクリーンショット | ~100MB | ~2秒 |

## クイックスタート

バイナリをインストールするか、お好みのパッケージマネージャーを使用してください：

```bash
# バイナリ（任意のLinux、依存なし）
curl -sL https://b4n1.com/install | bash

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

| 言語 | パッケージ | バージョン | バイナリ |
|------|-----------|-----------|----------|
| Python | `b4n1-web` | 0.12.3 | 同梱（musl） |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | 同梱（musl） |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | 同梱（musl） |
| C# (.NET) | `B4n1Web` | 0.12.3 | 同梱（musl） |

## ドキュメント

- [📖 完全なドキュメント](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCPツール](https://mcp.so/server/b4n1web/B4N1-com) — MCPレジストリ
- [📊 プロジェクト統計](STATS.md) — ダウンロード、バージョン、リリース

## リンク

- ウェブサイト：https://b4n1.com
- GitHub：https://github.com/B4N1-com/b4n1-web
- PyPI：https://pypi.org/project/b4n1-web
- npm：https://www.npmjs.com/package/b4n1-web
- NuGet：https://www.nuget.org/packages/B4n1Web
- Maven Central：https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## ライセンス

MIT