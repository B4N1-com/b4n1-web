# Installation

## Binary Install

### One-liner (Linux/macOS/Windows via WSL)

```bash
curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash
```

This downloads the correct binary for your platform and installs it to `~/.local/bin/b4n1web`.

### Manual Download

Download from [GitHub Releases](https://github.com/B4N1-com/b4n1-web/releases):

| Platform | File |
|----------|------|
| Linux x86_64 | `b4n1web-linux-amd64` |
| Linux ARM64 | `b4n1web-linux-arm64` |
| macOS Intel | `b4n1web-macos-x64` |
| macOS Apple Silicon | `b4n1web-macos-arm64` |
| Windows x86_64 | `b4n1web-windows-amd64.exe` |
| Windows ARM64 | `b4n1web-windows-arm64.exe` |

Make executable (Linux/macOS):
```bash
chmod +x b4n1web-*
sudo mv b4n1web-* /usr/local/bin/b4n1web
```

### Verify Installation

```bash
b4n1web --version
# b4n1web 0.13.0
```

---

## Python SDK

```bash
# From PyPI
pip install b4n1-web

# Or with UV
uv pip install b4n1-web
```

Requires: Python 3.8+

The wheel includes binaries for all 6 platforms.

---

## JavaScript/TypeScript SDK

```bash
# npm
npm install b4n1-web

# pnpm
pnpm add b4n1-web

# yarn
yarn add b4n1-web
```

Requires: Node.js 18+

The package includes binaries for all 6 platforms.

---

## Java SDK

### Maven

```xml
<dependency>
    <groupId>com.b4n1</groupId>
    <artifactId>b4n1-web</artifactId>
    <version>0.13.0</version>
</dependency>
```

### Gradle

```groovy
implementation 'com.b4n1:b4n1-web:0.13.0'
```

Requires: Java 11+

The JAR includes native binaries for all 6 platforms.

---

## C# .NET SDK

```bash
dotnet add package B4n1Web
```

Requires: .NET 6.0+

The NuGet package includes native binaries for all 6 platforms.

---

## Platform Compatibility

All SDKs bundle binaries for these 6 platforms:

| Platform | Binary | Python | JS | Java | C# |
|----------|--------|--------|-----|------|-----|
| Linux x86_64 | `b4n1web-linux-amd64` | ✅ | ✅ | ✅ | ✅ |
| Linux ARM64 | `b4n1web-linux-arm64` | ✅ | ✅ | ✅ | ✅ |
| macOS Intel | `b4n1web-macos-x64` | ✅ | ✅ | ✅ | ✅ |
| macOS Apple Silicon | `b4n1web-macos-arm64` | ✅ | ✅ | ✅ | ✅ |
| Windows x86_64 | `b4n1web-windows-amd64.exe` | ✅ | ✅ | ✅ | ✅ |
| Windows ARM64 | `b4n1web-windows-arm64.exe` | ✅ | ✅ | ✅ | ✅ |

---

## Chromium Dependency

**Chromium is downloaded automatically** when you use Render mode. No separate installation needed.

Supported platforms for Chromium:
- Linux x86_64 ✅
- Linux ARM64 ✅
- macOS Intel ✅
- macOS Apple Silicon ✅
- Windows x86_64 ✅
- Windows ARM64 ✅

The first Render mode call downloads Chromium (~160MB) to `~/.b4n1web/chromium/`. Subsequent calls reuse it.

---

## Troubleshooting Install

### Binary not found in PATH

```bash
# Add to PATH
export PATH="$HOME/.local/bin:$PATH"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### Permission denied

```bash
chmod +x ~/.local/bin/b4n1web
```

### Chromium download fails

Check internet connection. Chromium is downloaded from Google's storage. For offline environments, pre-download and place in `~/.b4n1web/chromium/`.

### Architecture mismatch

Ensure you're using the correct binary for your platform:
```bash
uname -m  # x86_64 or aarch64
```