# B4n1Web Java SDK

<p align="center">
  <a href="https://mavencentral.com/artifact/com.b4n1/b4n1-web"><img src="https://img.shields.io/maven/v/b4n1-web.svg" alt="Maven version"></a>
  <a href="https://github.com/B4N1-com/b4n1-web"><img src="https://img.shields.io/github/stars/B4N1-com/b4n1web?style=flat" alt="GitHub stars"></a>
</p>

Java bindings for B4n1Web: The Agentic Browser Engine.

## Installation

### 1. Install the B4n1Web Binary

```bash
curl -sL https://web.b4n1.com/install | bash
```

### 2. Add to Maven

```xml
<dependency>
    <groupId>com.b4n1</groupId>
    <artifactId>b4n1-web</artifactId>
    <version>0.2.3</version>
</dependency>
```

### Or Gradle

```groovy
implementation 'com.b4n1:b4n1-web:0.2.3'
```

## Quick Start

```java
import com.b4n1.web.*;

public class Main {
    public static void main(String[] args) {
        try (var browser = new AgentBrowser()) {
            var page = browser.goto("https://example.com");
            
            System.out.println("Page content: " + page.getMarkdown());
            System.out.println("Found " + page.getLinks().size() + " links");
            
            var mainContent = page.getMainContent();
            System.out.println("Main content: " + mainContent.substring(0, Math.min(200, mainContent.length())) + "...");
            
            var githubLinks = page.findLinksByText("github");
            System.out.println("GitHub links: " + githubLinks);
        }
    }
}
```

## Browser Modes

### Light Mode (Default)

```java
var options = new BrowserOptions();
options.setMode(BrowserMode.LIGHT);
var browser = new AgentBrowser(options);
```

### JS Mode

```java
var options = new BrowserOptions();
options.setMode(BrowserMode.JS);
var browser = new AgentBrowser(options);
```

### Render Mode (Coming Soon)

```java
var options = new BrowserOptions();
options.setMode(BrowserMode.RENDER);
var browser = new AgentBrowser(options);
```

## Options

```java
var options = new BrowserOptions();
options.setTimeout(60);
options.setUserAgent("MyAgent/1.0");
var browser = new AgentBrowser(options);
```

## SecurityShield

Optional security validation:

```java
var shield = new SecurityShield();
var result = shield.isUrlSafe("https://example.com");

if (!result.isSafe) {
    System.out.println("URL is unsafe!");
}
```

## License

MIT
