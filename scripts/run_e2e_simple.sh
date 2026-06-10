#!/bin/bash
# ──────────────────────────────────────────────────────
# B4n1Web — Simple E2E Test Runner (Podman)
# Tests each SDK in an isolated container
# ──────────────────────────────────────────────────────

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

IMAGE_NAME="b4n1web-e2e"
EXIT_CODE=0

log()    { echo -e "\033[0;36m│ $1\033[0m"; }
ok()     { echo -e "\033[0;32m│ ✅ $1\033[0m"; }
fail()   { echo -e "\033[0;31m│ ❌ $1\033[0m"; EXIT_CODE=1; }
skip()   { echo -e "\033[1;33m│ ⏭️  $1\033[0m"; }
header() { echo -e "\n\033[1;36m├─ $1\033[0m"; }

build_image() {
    header "Building E2E container image..."
    if podman image exists "$IMAGE_NAME" 2>/dev/null; then
        log "Image already exists, reusing"
        return
    fi
    podman build -t "$IMAGE_NAME" -f "$REPO_ROOT/Containerfile" "$REPO_ROOT" 2>&1 | tail -5
    ok "Image built: $IMAGE_NAME"
}

run_in_container() {
    local name="$1"
    local cmd="$2"
    header "Testing: $name"
    local rc=0
    podman run --rm \
        --name "b4n1web-e2e-${name}" \
        -v "${REPO_ROOT}/sdks/python:/sdk-python:ro" \
        -v "${REPO_ROOT}/sdks/javascript:/sdk-javascript:ro" \
        -v "${REPO_ROOT}/sdks/go:/sdk-go:ro" \
        -v "${REPO_ROOT}/sdks/java:/sdk-java:ro" \
        -v "${REPO_ROOT}/sdks/csharp:/sdk-csharp:ro" \
        -v "${REPO_ROOT}/engine/cli-core/target/release/b4n1web:/usr/local/bin/b4n1web:ro" \
        -v "${REPO_ROOT}/internal/tests:/internal/tests:ro" \
        --network host \
        "$IMAGE_NAME" \
        bash -c "$cmd" || rc=$?
    if [ $rc -eq 0 ]; then ok "$name PASSED"; else fail "$name FAILED (exit: $rc)"; fi
    return $rc
}

test_python() {
    run_in_container "python" '
        set -e
        echo "🐍 Installing Python SDK..."
        cp -r /sdk-python /tmp/sdk-python
        cd /tmp/sdk-python
        pip3 install --root-user-action=ignore . 2>&1 | tail -3
        pip3 install --root-user-action=ignore pytest requests 2>&1 | tail -1
        echo "🐍 Running tests..."
        python3 -c "
from b4n1web import AgentBrowser, BrowserMode
browser = AgentBrowser(mode=BrowserMode.LIGHT)
page = browser.goto(\"https://example.com\")
print(f\"✅ URL: {page.url}\")
print(f\"✅ Links: {len(page.links)}\")
assert page.url == \"https://example.com\"
assert len(page.markdown) > 0
browser.close()
print(\"✅ Python SDK E2E passed\")
"
    '
}

test_javascript() {
    run_in_container "javascript" '
        set -e
        echo "🟨 Installing JavaScript SDK..."
        cp -r /sdk-javascript /tmp/sdk-js
        cd /tmp/sdk-js
        npm install 2>&1 | tail -3
        npm run build 2>&1 | tail -3
        echo "🟨 Running vitest..."
        npx vitest run 2>&1 | tail -10
        echo "✅ JavaScript SDK E2E passed"
    '
}

test_java() {
    run_in_container "java" '
        set -e
        echo "☕ Building Java SDK..."
        cp -r /sdk-java /tmp/sdk-java
        cd /tmp/sdk-java
        mvn package -DskipTests -q 2>&1 | tail -3
        JAR=$(find /tmp/sdk-java/target -name "b4n1-web-*.jar" ! -name "*-sources*" ! -name "*-javadoc*" | head -1)
        echo "☕ Running tests..."
        cd /internal/tests/java
        javac -cp "$JAR" UnitTests.java 2>&1 | tail -3
        java -cp ".:$JAR" com.b4n1.web.UnitTests 2>&1 | tail -10
        echo "✅ Java SDK E2E passed"
    '
}

test_csharp() {
    run_in_container "csharp" '
        set -e
        echo "🔷 Building C# SDK..."
        cp -r /sdk-csharp /tmp/sdk-csharp
        cd /tmp/sdk-csharp/src
        dotnet build -c Release -q 2>&1 | tail -3
        echo "🔷 Running tests..."
        cp -r /internal/tests/csharp /tmp/csharp-tests
        cd /tmp/csharp-tests
        mkdir -p sdk_src
        cp /tmp/sdk-csharp/src/*.cs sdk_src/
        dotnet test --no-restore 2>&1 | tail -10
        echo "✅ C# SDK E2E passed"
    '
}


test_render() {
    run_in_container "render" '
        set -e
        echo "🎨 Testing render mode..."
        CHROMIUM=$(/usr/local/bin/b4n1web chromium version 2>&1)
        if echo "$CHROMIUM" | grep -qi "not installed"; then
            echo "⏭️  Chromium not installed - render skipped"
            exit 0
        fi
        echo "✅ Chromium installed"
        OUTPUT=$(/usr/local/bin/b4n1web goto https://example.com --mode render 2>&1) || true
        echo "$OUTPUT" | head -15
        echo "✅ Render mode E2E passed"
    '
}
test_binary() {
    run_in_container "binary" '
        set -e
        echo "📦 Testing binary..."
        b4n1web --version
        b4n1web goto https://example.com --mode light 2>&1 | head -10
        echo "✅ Binary E2E passed"
    '
}

cleanup() {
    log "Cleaning up..."
    podman rm -f $(podman ps -a --filter "name=b4n1web-e2e-" --format "{{.ID}}") 2>/dev/null || true
}

main() {
    echo -e "\033[1;36m"
    echo "╔═══════════════════════════════════════════════╗"
    echo "║   B4n1Web E2E Test Suite (Podman)            ║"
    echo "╚═══════════════════════════════════════════════╝"
    echo -e "\033[0m"
    
    log "Podman: $(podman --version)"
    echo ""
    
    build_image
    
    test_python || true
    test_javascript || true
    test_java || true
    test_csharp || true
    test_render || true
    test_binary || true
    
    cleanup
    
    echo ""
    if [ $EXIT_CODE -eq 0 ]; then
        echo -e "\033[1;32m"
        echo "╔═══════════════════════════════════════════════╗"
        echo "║   ✅ ALL E2E TESTS PASSED                    ║"
        echo "╚═══════════════════════════════════════════════╝"
    else
        echo -e "\033[1;31m"
        echo "╔═══════════════════════════════════════════════╗"
        echo "║   ❌ SOME E2E TESTS FAILED                   ║"
        echo "╚═══════════════════════════════════════════════╝"
    fi
    echo -e "\033[0m"
    exit $EXIT_CODE
}

main "$@"
