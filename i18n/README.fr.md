<div align="center">

<img src="b4n1web_presentation.png" alt="B4n1Web" width="100%">

# 🌐 B4n1Web — Moteur de Navigateur Agentique

**Navigateur headless ultra-léger pour agents IA.**

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

Un seul binaire Rust · 4 SDK de langage · 33 outils MCP.
Naviguez vers des URLs, extrayez du contenu structuré (markdown, liens, captures d'écran) et créez des workflows autonomes pour agents.

</div>

---

## 🌍 Languages / Idiomas / 语言

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](../README.md) | 🇪🇸 [Español](i18n/README.es.md) | 🇫🇷 [Français](i18n/README.fr.md) | 🇩🇪 [Deutsch](i18n/README.de.md) | 🇵🇹 [Português](i18n/README.pt-BR.md) | 🇮🇹 [Italiano](i18n/README.it.md) |
| 🇨🇳 [简体中文](i18n/README.zh-CN.md) | 🇯🇵 [日本語](i18n/README.ja.md) | 🇰🇷 [한국어](i18n/README.ko.md) | 🇷🇺 [Русский](i18n/README.ru.md) | 🇸🇦 [العربية](i18n/README.ar.md) | 🇮🇳 [हिन्दी](i18n/README.hi.md) |

---

## 🖥 🖥 Prise en charge des plateformes

8 binaires pré-compilés — fonctionne partout :

| Platform | Architectures | Binary |
|----------|---------------|--------|
| **Linux** | x86_64, aarch64, i686 | `musl` (static, no glibc) |
| **macOS** | x86_64, arm64 | `universal` |
| **Windows** | x86_64, arm64, i686 | `MSVC` |

## Fonctionnalités

- **33 MCP tools** — Intégration IA
- **Single self-contained Rust binary** ~11MB, no runtime dependencies
- **4 language SDKs** (Python, JS, Java, C#) with bundled binary
- **Static linking (musl)** — works on any Linux, no glibc required
- **Three modes**: Light (instant), JS (scripts), Render (Chromium)
- **Security shield**: domain filtering, safe browsing
- **Network interception**: block resources, mock responses
- **MCP Server**: stdio transport, no port needed

## Modes du Navigateur

| Mode | Description | RAM | Démarrage |
|------|-------------|-----|---------|
| Light | Récupération HTTP + parsing HTML | ~15MB | Instant |
| JS | Light + extraction JavaScript | ~15MB | Instant |
| Render | Chromium complet + captures d'écran | ~100MB | ~2s |

## Démarrage Rapide

Installez le binaire ou utilisez votre gestionnaire de paquets préféré :

```bash
# Binaire (Linux, macOS, Windows — sans dépendances)
curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash

# Ou via les gestionnaires de paquets
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java : ajoutez la dépendance depuis Maven Central
```

Utilisation de base :

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### Serveur MCP

```bash
# mode stdio (par défaut)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## Matrice des SDK

| Langage | Package | Version | Binaire |
|----------|---------|---------|--------|
| Python | `b4n1-web` | 0.12.3 | Intégré (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Intégré (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Intégré (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Intégré (musl) |

## Documentation

- [📖 Documentation complète](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [Outils MCP](https://mcp.so/server/b4n1web/B4N1-com) — MCP registry

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## License

Apache License 2.0
