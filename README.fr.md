<div align="center">

# 🌐 B4n1Web — Moteur de Navigateur Agentique

**Navigateur headless ultra-léger pour agents IA.**

[![GitHub](https://img.shields.io/github/license/B4N1-com/b4n1-web)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

Un seul binaire Rust · 4 SDK de langage · 33 outils MCP.
Naviguez vers des URLs, extrayez du contenu structuré (markdown, liens, captures d'écran) et créez des workflows autonomes pour agents.

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 Langues

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## Fonctionnalités

- **33 outils MCP** pour l'intégration d'agents IA
- **Un seul binaire Rust autonome** ~11 Mo, aucune dépendance d'exécution
- **4 SDK de langage** (Python, JS, Java, C#) avec binaire intégré
- **Liaison statique (musl)** — fonctionne sur tout Linux, sans glibc requis
- **Trois modes** : Light (instantané), JS (scripts), Render (Chromium)
- **Bouclier de sécurité** : filtrage de domaines, navigation sûre
- **Interception réseau** : bloquer des ressources, simuler des réponses
- **Serveur MCP** : transport stdio, aucun port requis

## Modes du Navigateur

| Mode | Description | RAM | Démarrage |
|------|-------------|-----|-----------|
| Light | Récupération HTTP + parsing HTML | ~15 Mo | Instantané |
| JS | Light + extraction JavaScript | ~15 Mo | Instantané |
| Render | Chromium complet + captures | ~100 Mo | ~2 s |

## Démarrage Rapide

Installez le binaire ou utilisez votre gestionnaire de paquets préféré :

```bash
# Binaire (tout Linux, sans dépendances)
curl -sL https://b4n1.com/install | bash

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

| Langage | Paquet | Version | Binaire |
|---------|--------|---------|---------|
| Python | `b4n1-web` | 0.12.3 | Intégré (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Intégré (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Intégré (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Intégré (musl) |

## Documentation

- [📖 Documentation complète](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [Outils MCP](https://mcp.so/server/b4n1web/B4N1-com) — registre MCP
- [📊 Statistiques du projet](STATS.md) — téléchargements, versions, releases

## Liens

- Site web : https://b4n1.com
- GitHub : https://github.com/B4N1-com/b4n1-web
- PyPI : https://pypi.org/project/b4n1-web
- npm : https://www.npmjs.com/package/b4n1-web
- NuGet : https://www.nuget.org/packages/B4n1Web
- Maven Central : https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## Licence

MIT