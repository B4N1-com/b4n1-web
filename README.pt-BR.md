<div align="center">

# 🌐 B4n1Web — Mecanismo de Navegação Agêntico

**Navegador headless ultraleve para agentes de IA.**

[![GitHub](https://img.shields.io/github/license/B4N1-com/b4n1-web)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

Um único binário Rust · 4 SDKs de linguagem · 33 ferramentas MCP.
Navegue por URLs, extraia conteúdo estruturado (markdown, links, screenshots) e construa fluxos de trabalho autônomos para agentes.

**[📊 Estatísticas do projeto → STATS.md](STATS.md)**

---

## 🌍 Idiomas

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## Recursos

- **33 ferramentas MCP** para integração com agentes de IA
- **Um único binário Rust autônomo** ~11 MB, sem dependências de runtime
- **4 SDKs de linguagem** (Python, JS, Java, C#) com binário integrado
- **Ligação estática (musl)** — funciona em qualquer Linux, sem glibc
- **Três modos**: Light (instantâneo), JS (scripts), Render (Chromium)
- **Escudo de segurança**: filtragem de domínios, navegação segura
- **Interceptação de rede**: bloquear recursos, simular respostas
- **Servidor MCP**: transporte stdio, sem necessidade de porta

## Modos do Navegador

| Modo | Descrição | RAM | Inicialização |
|------|-----------|-----|---------------|
| Light | Busca HTTP + parsing de HTML | ~15 MB | Instantânea |
| JS | Light + extração de JavaScript | ~15 MB | Instantânea |
| Render | Chromium completo + screenshots | ~100 MB | ~2 s |

## Início Rápido

Instale o binário ou use seu gerenciador de pacotes preferido:

```bash
# Binário (qualquer Linux, sem dependências)
curl -sL https://b4n1.com/install | bash

# Ou via gerenciadores de pacotes
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: adicione a dependência do Maven Central
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
# modo stdio (padrão)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## Matriz de SDKs

| Linguagem | Pacote | Versão | Binário |
|-----------|--------|--------|---------|
| Python | `b4n1-web` | 0.12.3 | Integrado (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Integrado (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Integrado (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Integrado (musl) |

## Documentação

- [📖 Documentação completa](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [Ferramentas MCP](https://mcp.so/server/b4n1web/B4N1-com) — registro MCP
- [📊 Estatísticas do projeto](STATS.md) — downloads, versões, releases

## Links

- Site: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## Licença

MIT