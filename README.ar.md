<div align="center">

# 🌐 B4n1Web — محرك متصفح وكيل

**متصفح خفيف للغاية بدون واجهة لوكيلات الذكاء الاصطناعي.**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

ملف Rust واحد · 4 حزم SDK للغات · 33 أداة MCP.
تصفح الروابط، واستخرج المحتوى المنظم (ماركداون، روابط، لقطات شاشة)، وابنِ سير عمل مستقلاً للوكلاء.

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 اللغات

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## المميزات

- **33 أداة MCP** لدمج وكلاء الذكاء الاصطناعي
- **ملف Rust واحد مكتفٍ ذاتياً** بحجم ~11 ميجابايت، بدون تبعيات تشغيل
- **4 حزم SDK للغات** (Python، JS، Java، C#) مع الملف المدمج
- **ربط ثابت (musl)** — يعمل على أي لينكس، بدون الحاجة إلى glibc
- **ثلاثة أوضاع**: Light (فوري)، JS (سكربتات)، Render (Chromium)
- **درع أمان**: تصفية النطاقات، تصفح آمن
- **اعتراض الشبكة**: حظر الموارد، محاكاة الاستجابات
- **خادم MCP**: نقل stdio، لا يحتاج منفذاً

## أوضاع المتصفح

| الوضع | الوصف | الذاكرة | الإقلاع |
|-------|--------|---------|---------|
| Light | جلب HTTP + تحليل HTML | ~15MB | فوري |
| JS | Light + استخراج JavaScript | ~15MB | فوري |
| Render | Chromium كامل + لقطات شاشة | ~100MB | ~2 ثانية |

## البداية السريعة

ثبّت الملف أو استخدم مدير الحزم المفضل لديك:

```bash
# الملف الثنائي (أي لينكس، بدون تبعيات)
curl -sL https://b4n1.com/install | bash

# أو عبر مديري الحزم
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: أضف التبعية من Maven Central
```

الاستخدام الأساسي:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### خادم MCP

```bash
# وضع stdio (افتراضي)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## مصفوفة حزم SDK

| اللغة | الحزمة | الإصدار | الملف الثنائي |
|-------|--------|---------|---------------|
| Python | `b4n1-web` | 0.12.3 | مدمج (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | مدمج (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | مدمج (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | مدمج (musl) |

## التوثيق

- [📖 التوثيق الكامل](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [أدوات MCP](https://mcp.so/server/b4n1web/B4N1-com) — سجل MCP
- [📊 إحصائيات المشروع](STATS.md) — التنزيلات، الإصدارات، النشرات

## الروابط

- الموقع: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## الترخيص

Apache License 2.0