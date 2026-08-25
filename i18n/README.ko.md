<div align="center">

<img src="b4n1web_presentation.png" alt="B4n1Web" width="100%">

# 🌐 B4n1Web — 에이전트형 브라우저 엔진

**AI 에이전트를 위한 초경량 헤드리스 브라우저.**

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

단일 Rust 바이너리 · 4개 언어 SDK · 33개 MCP 도구.
URL 탐색, 구조화된 콘텐츠 추출(Markdown, 링크, 스크린샷), 에이전트용 자율 워크플로 구축.

</div>

---

## 🌍 Languages / Idiomas / 语言

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](../README.md) | 🇪🇸 [Español](i18n/README.es.md) | 🇫🇷 [Français](i18n/README.fr.md) | 🇩🇪 [Deutsch](i18n/README.de.md) | 🇵🇹 [Português](i18n/README.pt-BR.md) | 🇮🇹 [Italiano](i18n/README.it.md) |
| 🇨🇳 [简体中文](i18n/README.zh-CN.md) | 🇯🇵 [日本語](i18n/README.ja.md) | 🇰🇷 [한국어](i18n/README.ko.md) | 🇷🇺 [Русский](i18n/README.ru.md) | 🇸🇦 [العربية](i18n/README.ar.md) | 🇮🇳 [हिन्दी](i18n/README.hi.md) |

---

## 🖥 🖥 플랫폼 지원

8개 사전 컴파일된 바이너리——모든 곳에서 작동:

| Platform | Architectures | Binary |
|----------|---------------|--------|
| **Linux** | x86_64, aarch64, i686 | `musl` (static, no glibc) |
| **macOS** | x86_64, arm64 | `universal` |
| **Windows** | x86_64, arm64, i686 | `MSVC` |

## 기능

- **33 MCP tools** — AI 에이전트 통합
- **Single self-contained Rust binary** ~11MB, no runtime dependencies
- **4 language SDKs** (Python, JS, Java, C#) with bundled binary
- **Static linking (musl)** — works on any Linux, no glibc required
- **Three modes**: Light (instant), JS (scripts), Render (Chromium)
- **Security shield**: domain filtering, safe browsing
- **Network interception**: block resources, mock responses
- **MCP Server**: stdio transport, no port needed

## 브라우저 모드

| 모드 | 설명 | RAM | 시작 |
|------|-------------|-----|---------|
| Light | HTTP 가져오기 + HTML 파싱 | ~15MB | Instant |
| JS | Light + JavaScript 추출 | ~15MB | Instant |
| Render | 전체 Chromium + 스크린샷 | ~100MB | ~2s |

## 빠른 시작

바이너리를 설치하거나 선호하는 패키지 관리자를 사용하세요:

```bash
# 바이너리 (Linux, macOS, Windows — 의존성 없음)
curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash

# 또는 패키지 관리자 사용
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: Maven Central에서 의존성 추가
```

기본 사용법:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### MCP 서버

```bash
# stdio 모드 (기본값)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## SDK 매트릭스

| 언어 | Package | Version | 바이너리 |
|----------|---------|---------|--------|
| Python | `b4n1-web` | 0.12.3 | 번들 (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | 번들 (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | 번들 (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | 번들 (musl) |

## 문서

- [📖 전체 문서](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCP 도구](https://mcp.so/server/b4n1web/B4N1-com) — MCP registry

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## License

Apache License 2.0
