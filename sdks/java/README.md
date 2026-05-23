# B4n1Web Java SDK

[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> Ultra-lightweight agentic browser engine with bundled binary.

## Install

```xml
<dependency>
    <groupId>com.b4n1</groupId>
    <artifactId>b4n1-web</artifactId>
    <version>0.9.0</version>
</dependency>
```

Binary is bundled — no separate installation needed.

## Quick Start

```java
import com.b4n1.web.*;

AgentBrowser browser = new AgentBrowser(new BrowserOptions());
Page page = browser.goto_("https://example.com");
System.out.println(page.getMarkdown());
System.out.println(page.getLinks().size() + " links found");
System.out.println(page.getMainContent());
System.out.println(page.findLinksByText("github"));
browser.close();
```

## Browser Modes

```java
// Light mode (default)
new AgentBrowser(new BrowserOptions());

// JS mode
new AgentBrowser(new BrowserOptions(BrowserMode.JS));

// Render mode
new AgentBrowser(new BrowserOptions(BrowserMode.RENDER));
```

## Security

```java
SecurityShield shield = new SecurityShield();
shield.markDomain("evil.com", false);
SecurityCheckResult result = shield.isUrlSafe("https://evil.com");
// result.isSafe(), result.needsApiCheck()
```

## Version

SDK: **0.9.0** | Binary: **0.9.0** (bundled)

## Links

- [Maven Central](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
- [GitHub](https://github.com/B4N1-com/b4n1-web)
- [Website](https://web.b4n1.com)

---
*Built by B4N1 with ❤️ · All rights reserved.*
