<div align="center">

# 🌐 B4n1Web — 에이전트형 브라우저 엔진

**AI 에이전트를 위한 초경량 헤드리스 브라우저.**

[![GitHub](https://img.shields.io/github/license/B4N1-com/b4n1-web)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

단일 Rust 바이너리 · 4개 언어 SDK · 33개 MCP 도구.
URL 탐색, 구조화된 콘텐츠 추출(Markdown, 링크, 스크린샷), 에이전트용 자율 워크플로 구축.

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 언어

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## 기능

- **33개 MCP 도구** — AI 에이전트 통합용
- **단일 자체 포함 Rust 바이너리** (약 11MB), 런타임 의존성 없음
- **4개 언어 SDK** (Python, JS, Java, C#) — 바이너리 번들 포함
- **정적 링킹 (musl)** — 모든 Linux에서 작동, glibc 불필요
- **세 가지 모드**: Light(즉시), JS(스크립트), Render(Chromium)
- **보안 실드**: 도메인 필터링, 안전한 브라우징
- **네트워크 인터셉트**: 리소스 차단, 응답 모킹
- **MCP 서버**: stdio 전송, 포트 불필요

## 브라우저 모드

| 모드 | 설명 | RAM | 시작 |
|------|------|-----|------|
| Light | HTTP 가져오기 + HTML 파싱 | ~15MB | 즉시 |
| JS | Light + JavaScript 추출 | ~15MB | 즉시 |
| Render | 전체 Chromium + 스크린샷 | ~100MB | ~2초 |

## 빠른 시작

바이너리를 설치하거나 선호하는 패키지 관리자를 사용하세요:

```bash
# 바이너리 (모든 Linux, 의존성 없음)
curl -sL https://b4n1.com/install | bash

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

| 언어 | 패키지 | 버전 | 바이너리 |
|------|--------|------|----------|
| Python | `b4n1-web` | 0.12.3 | 번들 (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | 번들 (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | 번들 (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | 번들 (musl) |

## 문서

- [📖 전체 문서](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCP 도구](https://mcp.so/server/b4n1web/B4N1-com) — MCP 레지스트리
- [📊 프로젝트 통계](STATS.md) — 다운로드, 버전, 릴리스

## 링크

- 웹사이트: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## 라이선스

MIT