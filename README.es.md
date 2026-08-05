<div align="center">

# 🌐 B4n1Web — Motor de Navegación Agéntico

**Navegador headless ultraligero para agentes de IA.**

[![GitHub](https://img.shields.io/github/license/B4N1-com/b4n1-web)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

Un solo binario Rust · 4 SDKs de lenguaje · 33 herramientas MCP.
Navega URLs, extrae contenido estructurado (markdown, enlaces, capturas) y construye flujos de trabajo autónomos para agentes.

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 Idiomas

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## Características

- **33 herramientas MCP** para integración con agentes de IA
- **Un único binario Rust autocontenido** de ~11MB, sin dependencias de runtime
- **4 SDKs de lenguaje** (Python, JS, Java, C#) con binario integrado
- **Enlazado estático (musl)** — funciona en cualquier Linux, no requiere glibc
- **Tres modos**: Light (instantáneo), JS (scripts), Render (Chromium)
- **Escudo de seguridad**: filtrado de dominios, navegación segura
- **Interceptación de red**: bloquear recursos, simular respuestas
- **Servidor MCP**: transporte stdio, no necesita puerto

## Modos del Navegador

| Modo | Descripción | RAM | Arranque |
|------|-------------|-----|----------|
| Light | Descarga HTTP + parseo de HTML | ~15MB | Instantáneo |
| JS | Light + extracción de JavaScript | ~15MB | Instantáneo |
| Render | Chromium completo + capturas | ~100MB | ~2s |

## Inicio Rápido

Instala el binario o usa tu gestor de paquetes preferido:

```bash
# Binario (cualquier Linux, sin dependencias)
curl -sL https://b4n1.com/install | bash

# O mediante gestores de paquetes
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: añade la dependencia desde Maven Central
```

Uso básico:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### Servidor MCP

```bash
# modo stdio (por defecto)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## Matriz de SDKs

| Lenguaje | Paquete | Versión | Binario |
|----------|---------|---------|---------|
| Python | `b4n1-web` | 0.12.3 | Integrado (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Integrado (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Integrado (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Integrado (musl) |

## Documentación

- [📖 Documentación completa](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [Herramientas MCP](https://mcp.so/server/b4n1web/B4N1-com) — registro MCP
- [📊 Estadísticas del proyecto](STATS.md) — descargas, versiones, releases

## Enlaces

- Sitio web: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## Licencia

MIT