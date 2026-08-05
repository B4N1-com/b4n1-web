<div align="center">

# 🌐 B4n1Web — Motore di Navigazione Agéntico

**Browser headless ultraleggero per agenti IA.**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

Un singolo binario Rust · 4 SDK di linguaggio · 33 strumenti MCP.
Naviga URL, estrai contenuti strutturati (markdown, link, screenshot) e costruisci flussi di lavoro autonomi per agenti.

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 Lingue

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## Caratteristiche

- **33 strumenti MCP** per l'integrazione con agenti IA
- **Un singolo binario Rust autonomo** ~11 MB, senza dipendenze di runtime
- **4 SDK di linguaggio** (Python, JS, Java, C#) con binario integrato
- **Linking statico (musl)** — funziona su qualsiasi Linux, senza glibc
- **Tre modalità**: Light (istantanea), JS (script), Render (Chromium)
- **Scudo di sicurezza**: filtraggio domini, navigazione sicura
- **Intercettazione di rete**: blocca risorse, simula risposte
- **Server MCP**: trasporto stdio, nessuna porta richiesta

## Modalità del Browser

| Modalità | Descrizione | RAM | Avvio |
|----------|-------------|-----|-------|
| Light | Fetch HTTP + parsing HTML | ~15 MB | Istantaneo |
| JS | Light + estrazione JavaScript | ~15 MB | Istantaneo |
| Render | Chromium completo + screenshot | ~100 MB | ~2 s |

## Avvio Rapido

Installa il binario o usa il tuo gestore di pacchetti preferito:

```bash
# Binario (qualsiasi Linux, senza dipendenze)
curl -sL https://b4n1.com/install | bash

# Oppure tramite gestori di pacchetti
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: aggiungi la dipendenza da Maven Central
```

Utilizzo di base:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### Server MCP

```bash
# modalità stdio (predefinita)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## Matrice SDK

| Linguaggio | Pacchetto | Versione | Binario |
|------------|-----------|----------|---------|
| Python | `b4n1-web` | 0.12.3 | Integrato (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Integrato (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Integrato (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Integrato (musl) |

## Documentazione

- [📖 Documentazione completa](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [Strumenti MCP](https://mcp.so/server/b4n1web/B4N1-com) — registro MCP
- [📊 Statistiche del progetto](STATS.md) — download, versioni, release

## Link

- Sito web: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## Licenza

MIT