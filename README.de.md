<div align="center">

# 🌐 B4n1Web — Agentischer Browser-Engine

**Ultra-leichter headless Browser für KI-Agenten.**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

Ein einzelnes Rust-Binary · 4 Sprach-SDKs · 33 MCP-Tools.
URLs aufrufen, strukturierte Inhalte extrahieren (Markdown, Links, Screenshots) und autonome Agenten-Workflows aufbauen.

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 Sprachen

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## Funktionen

- **33 MCP-Tools** für die KI-Agenten-Integration
- **Ein einzelnes autarkes Rust-Binary** ~11 MB, ohne Laufzeit-Abhängigkeiten
- **4 Sprach-SDKs** (Python, JS, Java, C#) mit integriertem Binary
- **Statisches Linking (musl)** — funktioniert auf jedem Linux, ohne glibc
- **Drei Modi**: Light (sofort), JS (Skripte), Render (Chromium)
- **Sicherheitsschild**: Domain-Filterung, sicheres Surfen
- **Netzwerk-Interception**: Ressourcen blockieren, Antworten simulieren
- **MCP-Server**: stdio-Transport, kein Port nötig

## Browser-Modi

| Modus | Beschreibung | RAM | Start |
|-------|--------------|-----|-------|
| Light | HTTP-Abruf + HTML-Parsing | ~15 MB | Sofort |
| JS | Light + JavaScript-Extraktion | ~15 MB | Sofort |
| Render | Volles Chromium + Screenshots | ~100 MB | ~2 s |

## Schnellstart

Installieren Sie das Binary oder verwenden Sie Ihren bevorzugten Paketmanager:

```bash
# Binary (jedes Linux, ohne Abhängigkeiten)
curl -sL https://b4n1.com/install | bash

# Oder über Paketmanager
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: Abhängigkeit von Maven Central hinzufügen
```

Grundlegende Verwendung:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### MCP-Server

```bash
# stdio-Modus (Standard)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## SDK-Matrix

| Sprache | Paket | Version | Binary |
|---------|-------|---------|--------|
| Python | `b4n1-web` | 0.12.3 | Integriert (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Integriert (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Integriert (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Integriert (musl) |

## Dokumentation

- [📖 Vollständige Dokumentation](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCP-Tools](https://mcp.so/server/b4n1web/B4N1-com) — MCP-Registry
- [📊 Projektstatistiken](STATS.md) — Downloads, Versionen, Releases

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## Lizenz

MIT