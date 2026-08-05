<div align="center">

# 🌐 B4n1Web — Агентный браузерный движок

**Сверхлёгкий headless-браузер для ИИ-агентов.**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

Один Rust-бинарник · 4 SDK · 33 MCP-инструмента.
Переходите по URL, извлекайте структурированный контент (markdown, ссылки, скриншоты) и создавайте автономные рабочие процессы для агентов.

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 Языки

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## Возможности

- **33 MCP-инструмента** для интеграции с ИИ-агентами
- **Один автономный Rust-бинарник** ~11 МБ, без зависимостей времени выполнения
- **4 SDK** (Python, JS, Java, C#) со встроенным бинарником
- **Статическая линковка (musl)** — работает на любом Linux, без glibc
- **Три режима**: Light (мгновенный), JS (скрипты), Render (Chromium)
- **Защитный экран**: фильтрация доменов, безопасный просмотр
- **Перехват сети**: блокировка ресурсов, имитация ответов
- **MCP-сервер**: stdio-транспорт, порт не нужен

## Режимы браузера

| Режим | Описание | RAM | Запуск |
|-------|----------|-----|--------|
| Light | HTTP-загрузка + парсинг HTML | ~15 МБ | Мгновенно |
| JS | Light + извлечение JavaScript | ~15 МБ | Мгновенно |
| Render | Полный Chromium + скриншоты | ~100 МБ | ~2 с |

## Быстрый старт

Установите бинарник или используйте ваш любимый менеджер пакетов:

```bash
# Бинарник (любой Linux, без зависимостей)
curl -sL https://b4n1.com/install | bash

# Или через менеджеры пакетов
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: добавьте зависимость из Maven Central
```

Базовое использование:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### MCP-сервер

```bash
# режим stdio (по умолчанию)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## Матрица SDK

| Язык | Пакет | Версия | Бинарник |
|------|-------|--------|----------|
| Python | `b4n1-web` | 0.12.3 | Встроенный (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Встроенный (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Встроенный (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Встроенный (musl) |

## Документация

- [📖 Полная документация](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCP-инструменты](https://mcp.so/server/b4n1web/B4N1-com) — реестр MCP
- [📊 Статистика проекта](STATS.md) — загрузки, версии, релизы

## Ссылки

- Сайт: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## Лицензия

Apache License 2.0