<div align="center">

# 🌐 B4n1Web — एजेंटिक ब्राउज़र इंजन

**AI एजेंटों के लिए अल्ट्रा-लाइटवेट हेडलेस ब्राउज़र।**

[![GitHub](https://img.shields.io/github/license/B4N1-com/b4n1-web)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

एकल Rust बाइनरी · 4 भाषा SDK · 33 MCP टूल।
URL नेविगेट करें, संरचित सामग्री (मार्कडाउन, लिंक, स्क्रीनशॉट) निकालें, और एजेंटों के लिए स्वायत्त वर्कफ़्लो बनाएं।

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 भाषाएँ

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## विशेषताएँ

- **33 MCP टूल** — AI एजेंट एकीकरण के लिए
- **एकल स्व-निहित Rust बाइनरी** ~11MB, कोई रनटाइम निर्भरता नहीं
- **4 भाषा SDK** (Python, JS, Java, C#) — बंडल बाइनरी के साथ
- **स्टैटिक लिंकिंग (musl)** — किसी भी Linux पर काम करता है, glibc आवश्यक नहीं
- **तीन मोड**: Light (तत्काल), JS (स्क्रिप्ट), Render (Chromium)
- **सुरक्षा कवच**: डोमेन फ़िल्टरिंग, सुरक्षित ब्राउज़िंग
- **नेटवर्क इंटरसेप्शन**: संसाधन ब्लॉक करें, प्रतिक्रियाएँ मॉक करें
- **MCP सर्वर**: stdio ट्रांसपोर्ट, कोई पोर्ट आवश्यक नहीं

## ब्राउज़र मोड

| मोड | विवरण | RAM | स्टार्टअप |
|------|--------|-----|-----------|
| Light | HTTP फ़ेच + HTML पार्सिंग | ~15MB | तत्काल |
| JS | Light + JavaScript निष्कर्षण | ~15MB | तत्काल |
| Render | पूर्ण Chromium + स्क्रीनशॉट | ~100MB | ~2 सेकंड |

## त्वरित आरंभ

बाइनरी इंस्टॉल करें या अपना पसंदीदा पैकेज मैनेजर उपयोग करें:

```bash
# बाइनरी (कोई भी Linux, बिना निर्भरता)
curl -sL https://b4n1.com/install | bash

# या पैकेज मैनेजर के माध्यम से
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: Maven Central से निर्भरता जोड़ें
```

मूल उपयोग:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### MCP सर्वर

```bash
# stdio मोड (डिफ़ॉल्ट)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## SDK मैट्रिक्स

| भाषा | पैकेज | संस्करण | बाइनरी |
|------|--------|---------|--------|
| Python | `b4n1-web` | 0.12.3 | बंडल (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | बंडल (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | बंडल (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | बंडल (musl) |

## दस्तावेज़ीकरण

- [📖 पूर्ण दस्तावेज़ीकरण](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCP टूल](https://mcp.so/server/b4n1web/B4N1-com) — MCP रजिस्ट्री
- [📊 प्रोजेक्ट आँकड़े](STATS.md) — डाउनलोड, संस्करण, रिलीज़

## लिंक

- वेबसाइट: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## लाइसेंस

MIT