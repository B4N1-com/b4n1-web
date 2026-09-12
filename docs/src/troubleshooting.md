# Troubleshooting

Common issues and solutions.

## Chromium Not Found

**Error:** `Chromium not found` or `Failed to download Chromium`

**Solutions:**

1. **Auto-download failed** - Check internet connection. Chromium (~160MB) downloads from Google's storage.

2. **Use system Chromium:**
   ```bash
   export B4N1WEB_CHROMIUM=/usr/bin/chromium
   # or
   export B4N1WEB_CHROMIUM=/Applications/Chromium.app/Contents/MacOS/Chromium
   ```

3. **Manual download:**
   - Download from [Chromium Snapshots](https://chromium.woolyss.com/)
   - Place in `~/.b4n1web/chromium/`

4. **Linux ARM64 (Raspberry Pi):**
   ```bash
   sudo apt install chromium-browser
   export B4N1WEB_CHROMIUM=/usr/bin/chromium
   ```

## Permission Errors

**Error:** `Permission denied` when running binary

```bash
chmod +x b4n1web
```

**Error:** `Permission denied` for Chromium

```bash
chmod +x ~/.b4n1web/chromium/chrome
```

## Binary Not Found

**Error:** `Binary not found` or `b4n1web: command not found`

1. **Check PATH:**
   ```bash
   echo $PATH
   ```

2. **Add to PATH:**
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   ```

3. **Use full path:**
   ```bash
   /full/path/to/b4n1web goto https://example.com
   ```

## Timeout Issues

**Error:** `Binary timed out after 30s`

Increase timeout:

```python
browser = AgentBrowser(options={"timeout": 60})
```

```javascript
const browser = new AgentBrowser({ timeout: 60 });
```

```csharp
var browser = new AgentBrowser(new BrowserOptions { Timeout = 60 });
```

```java
AgentBrowser browser = new AgentBrowser(new BrowserOptions().setTimeout(60));
```

Or CLI:
```bash
b4n1web goto https://slow-site.com --timeout 60
```

## Connection Errors

**Error:** `Connection refused` or `Network error`

1. Check internet connection
2. Try different mode (Light mode doesn't need Chromium)
3. Check firewall/proxy settings
4. For corporate networks, configure proxy:

```bash
export HTTP_PROXY=http://proxy:8080
export HTTPS_PROXY=http://proxy:8080
```

## Version Mismatch

**Warning:** `Version mismatch: SDK v0.13.0 requires binary v0.13.0, but found v0.9.4`

Update both SDK and binary to same version:

```bash
# Reinstall SDK (gets latest binary)
pip install --upgrade b4n1-web
npm install b4n1-web@latest
dotnet add package B4n1Web --version 0.13.0
```

Or update binary manually:
```bash
curl -sL https://b4n1.com/install | bash
```

## Architecture Mismatch

**Error:** Binary runs but crashes immediately

Check architecture:
```bash
uname -m        # Linux/macOS
# x86_64 = amd64
# aarch64 = arm64

# Windows
echo %PROCESSOR_ARCHITECTURE%
```

Download correct binary from [Releases](https://github.com/B4N1-com/b4n1-web/releases).

## Mode-Specific Issues

### Light/JS Mode
- No Chromium needed
- Fastest
- Limited to static content

### Render Mode
- Needs Chromium
- Slower startup (~2s first run)
- Full browser features

**If Render mode fails, try JS mode first:**
```python
browser = AgentBrowser(options={"mode": "js"})
```

## Debug Mode

Enable verbose logging:

```bash
RUST_LOG=debug b4n1web goto https://example.com
```

```python
import logging
logging.basicConfig(level=logging.DEBUG)
```

## Getting Help

1. Check [GitHub Issues](https://github.com/B4N1-com/b4n1-web/issues)
2. Search existing issues
3. Create new issue with:
   - OS and version
   - b4n1web version (`b4n1web --version`)
   - Full error message
   - Minimal reproduction code