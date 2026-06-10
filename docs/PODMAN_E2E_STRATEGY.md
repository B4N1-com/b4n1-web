# Podman E2E Testing Strategy

## Overview

End-to-end testing using **Podman** containers to simulate a **clean user environment**.
Each SDK bundles its own binary, so tests verify the complete bundled package.
Standalone binary and MCP are also tested separately.

### Why Podman?

- **Isolated**: No pollution of the host system
- **Reproducible**: Same environment every time
- **Multi-distro**: Test on Alpine, Ubuntu, Fedora, etc.
- **Agent-safe**: Agents can run tests without side effects

---

## Architecture

```
┌─────────────────────────────────────────────┐
│               Host Machine                   │
│  ┌───────────────────────────────────────┐   │
│  │         Podman Container               │   │
│  │  ┌────────────────────────────────┐  │   │
│  │  │  SDK with Bundled Binary       │  │   │
│  │  │  (Python/JS/Go/Java/C#)        │  │   │
│  │  │                                │  │   │
│  │  │  ┌──────────┐  ┌────────────┐  │  │   │
│  │  │  │ b4n1web  │  │ SDK wrapper│  │  │   │
│  │  │  │ binary   │← │ (subprocess│  │  │   │
│  │  │  │ bundled  │  │  finder)   │  │  │   │
│  │  │  └──────────┘  └────────────┘  │  │   │
│  │  └────────────────────────────────┘  │   │
│  │                                      │   │
│  │  E2E Tests (pytest / vitest / etc)  │   │
│  └──────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

---

## Test Flow (Per SDK)

### Phase 1: Install SDK
1. **Start clean container** (Ubuntu 22.04 base)
2. **Install SDK**: `pip install b4n1-web` / `npm install b4n1-web` / etc.
3. **Binary auto-detected**: SDK finds bundled binary automatically

### Phase 2: Verify Bundled Binary
1. Call `get_b4n1web_binary()` → returns path to bundled binary
2. Assert binary is executable and reports version

### Phase 3: Core Function Tests (Per SDK)

| Test | Description | Expected |
|------|-------------|----------|
| `test_goto_light` | `browser.goto("https://example.com")` | Returns Page with markdown, links |
| `test_goto_js_mode` | `browser.goto(url, mode=JS)` | Same as light + JS tags |
| `test_get_main_content` | `page.getMainContent()` | Returns content without headers |
| `test_find_links_by_text` | `page.findLinksByText("more")` | Returns matching links |
| `test_context_manager` | `with AgentBrowser() as b:` | Clean exit, no leaks |
| `test_timeout` | `browser.goto(url, timeout=1)` | Raises timeout error |
| `test_invalid_url` | `browser.goto("not-a-url")` | Raises navigation error |

### Phase 4: Security Tests (Per SDK)

| Test | Description | Expected |
|------|-------------|----------|
| `test_security_new_domain` | `shield.isUrlSafe("https://new.com")` | `{isSafe: true, needsApiCheck: true}` |
| `test_security_mark_safe` | `shield.markDomain("evil.com", false)` | Domain marked unsafe |
| `test_security_blocked` | `shield.isUrlSafe("https://evil.com")` | `{isSafe: false, needsApiCheck: false}` |
| `test_navigate_blocked` | `navigate("https://evil.com")` | Raises "URL flagged as unsafe" |

### Phase 5: Python-Specific (MCP)

| Test | Description | Expected |
|------|-------------|----------|
| `test_mcp_connect` | `AsyncMcpClient()` connects to `localhost:8765` | Connection successful |
| `test_mcp_goto` | `client.goto("https://example.com")` | Returns Page object |
| `test_mcp_get_links` | `client.getLinks(...)` | Returns list of links |
| `test_mcp_error` | Call with invalid URL | Returns McpError |

### Phase 6: Cleanup
1. **Stop MCP server** (if running)
2. **Remove container**: `podman rm -f b4n1web-e2e`
3. **Remove image**: `podman rmi b4n1web-e2e-base` (optional)

---

## Container Images

### Base Image: `b4n1web-e2e-base`

```dockerfile
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    python3 python3-pip \
    curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install b4n1web binary
RUN curl -sL https://web.b4n1.com/install | bash

# Copy SDK sources for local install
COPY python/ /sdk/python/
COPY javascript/ /sdk/javascript/
COPY go/ /sdk/go/
COPY java/ /sdk/java/
COPY csharp/ /sdk/csharp/

CMD ["bash"]
```

### Per-SDK Test Commands

```bash
# Python
podman run --rm b4n1web-e2e-base \
  bash -c "pip3 install /sdk/python/ && python3 -m pytest /tests/python/ -v"

# JavaScript
podman run --rm b4n1web-e2e-base \
  bash -c "apt-get install -y nodejs npm && cd /sdk/javascript && npm install && npm run build && cd / && node /tests/javascript/run.js"

# Go
podman run --rm b4n1web-e2e-base \
  bash -c "apt-get install -y golang && cd /sdk/go && go test ./internal/ -v"

# Java
podman run --rm b4n1web-e2e-base \
  bash -c "apt-get install -y maven && cd /sdk/java && mvn test"

# C#
podman run --rm b4n1web-e2e-base \
  bash -c "apt-get install -y dotnet-sdk-8.0 && cd /sdk/csharp && dotnet test"
```

---

## Test Runner Script

```bash
#!/bin/bash
# tests/e2e/run_all.sh - Run all E2E tests in Podman

set -euo pipefail

IMAGE="b4n1web-e2e-base"
CONTAINER="b4n1web-e2e-test"

echo "🧪 B4n1Web E2E Tests (Podman)"
echo "=============================="

# Build base image
echo "📦 Building base image..."
podman build -t "$IMAGE" -f tests/e2e/Containerfile .

# Run tests per SDK
echo ""
echo "🐍 Testing Python SDK..."
podman run --rm --name "$CONTAINER-python" "$IMAGE" \
  bash -c "pip3 install /sdk/python/ && python3 -m pytest /tests/python/ -v"

echo ""
echo "🟨 Testing JavaScript SDK..."
podman run --rm --name "$CONTAINER-js" "$IMAGE" \
  bash -c "apt-get install -y nodejs npm && cd /sdk/javascript && npm install && npm run build"

echo ""
echo "🐹 Testing Go SDK..."
podman run --rm --name "$CONTAINER-go" "$IMAGE" \
  bash -c "apt-get install -y golang && cd /sdk/go && go test ./internal/ -v"

echo ""
echo "☕ Testing Java SDK..."
podman run --rm --name "$CONTAINER-java" "$IMAGE" \
  bash -c "apt-get install -y maven && cd /sdk/java && mvn test"

echo ""
echo "🔷 Testing C# SDK..."
podman run --rm --name "$CONTAINER-csharp" "$IMAGE" \
  bash -c "apt-get install -y dotnet-sdk-8.0 && cd /sdk/csharp && dotnet test"

echo ""
echo "🎉 All E2E tests passed!"
```

---

## Version Mismatch Testing

### Scenario: SDK 0.4.0 + Binary 0.3.0

```bash
# Install older binary first
podman run --rm b4n1web-e2e-base \
  bash -c "
    # Remove current binary
    rm -f /usr/local/bin/b4n1web
    # Download older version (simulated)
    echo '#!/bin/bash\necho b4n1web 0.3.0' > /usr/local/bin/b4n1web
    chmod +x /usr/local/bin/b4n1web
    # Now import SDK - should warn
    pip3 install /sdk/python/
    python3 -c 'from b4n1web import AgentBrowser; AgentBrowser()' 2>&1 | grep -q 'Version mismatch'
  "
```

---

## CI/CD Integration

### GitHub Actions (optional, for private repo)

```yaml
name: E2E Tests
on:
  push:
    branches: [main]
  pull_request:

jobs:
  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Podman
        run: sudo apt-get install -y podman
      - name: Run E2E Tests
        run: bash tests/e2e/run_all.sh
```

---

## Future Improvements

### 1. `--wait-for <selector>` Feature
When implemented, add tests:
```python
# Python
page = browser.goto("https://spa.example.com", wait_for="#app-loaded")
assert "dynamic content" in page.markdown

# Verify binary receives flag: b4n1web goto URL --mode render --wait-for #app-loaded
```

### 2. `--only-repos` Feature
When implemented, add tests:
```python
# Python
links = browser.goto("https://github.com/user/repo").links_only_repos()
# Should strip /blob/, /tree/, #readme, etc.
assert all(is_repo_url(link) for link in links)
```

### 3. Multi-Distro Testing
```bash
# Test on different base images
for distro in alpine:3.19 ubuntu:22.04 fedora:39; do
  podman build -t b4n1web-e2e-$distro -f tests/e2e/Containerfile.$distro .
  podman run --rm b4n1web-e2e-$distro bash -c "..."
done
```

---

## Notes

- **Podman vs Docker**: Podman is rootless by default, more secure. Use `podman` CLI (drop-in replacement for `docker`).
- **Binary must be installed separately**: SDKs do NOT auto-download the binary. This is intentional for security.
- **Version warning is non-fatal**: SDK should still work with mismatch, just warn.
- **MCP server needs port**: Container must expose `8765` for MCP tests.
- **Network access**: Container needs internet to test real URLs. Use `--network=host` or Podman's default bridge.
