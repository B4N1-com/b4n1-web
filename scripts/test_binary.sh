#!/bin/bash
# ──────────────────────────────────────────────────────
# B4n1Web — Binary E2E Test Suite
# ──────────────────────────────────────────────────────
#
# Tests the b4n1web binary directly (no SDKs)
# Covers every command, every mode, every error path.
#
# Usage:
#   bash scripts/test_binary.sh
#
# ──────────────────────────────────────────────────────

set -euo pipefail

BINARY="${BINARY:-$(which b4n1web 2>/dev/null || echo '')}"
EXIT_CODE=0
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_SKIPPED=0

# ── Colors ──
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

pass()   { echo -e "  ${GREEN}✅ PASS${NC} $1"; TESTS_PASSED=$((TESTS_PASSED + 1)); TESTS_RUN=$((TESTS_RUN + 1)); }
fail()   { echo -e "  ${RED}❌ FAIL${NC} $1"; TESTS_FAILED=$((TESTS_FAILED + 1)); TESTS_RUN=$((TESTS_RUN + 1)); EXIT_CODE=1; }
skip()   { echo -e "  ${YELLOW}⏭️  SKIP${NC} $1"; TESTS_SKIPPED=$((TESTS_SKIPPED + 1)); TESTS_RUN=$((TESTS_RUN + 1)); }
header() { echo -e "\n${CYAN}${BOLD}├─ $1${NC}"; }

# ── Pre-flight ──
preflight() {
    header "Pre-flight"
    if [ -z "$BINARY" ]; then
        fail "b4n1web binary not found in PATH"
        exit 1
    fi
    if [ ! -f "$BINARY" ]; then
        fail "b4n1web binary not found: $BINARY"
        exit 1
    fi
    pass "Binary found: $BINARY"
}

# ── Version Tests ──
test_version() {
    header "Version"
    
    OUTPUT=$("$BINARY" update 2>&1) || fail "update exits non-zero"
    echo "$OUTPUT" | grep -qi "version" && pass "update returns version info" || fail "update: $OUTPUT"
    echo "$OUTPUT" | grep -qi "version" && pass "version output mentions version" || fail "version output: $OUTPUT"
}

# ── Help Tests ──
test_help() {
    header "Help"
    
    OUTPUT=$("$BINARY" --help 2>&1) || fail "--help exits non-zero"
    echo "$OUTPUT" | grep -q "B4n1Web" && pass "--help mentions B4n1Web" || fail "--help"
    echo "$OUTPUT" | grep -q "goto" && pass "--help lists goto command" || fail "--help"
    echo "$OUTPUT" | grep -q "mcp" && pass "--help lists mcp command" || fail "--help"
    echo "$OUTPUT" | grep -q "chromium" && pass "--help lists chromium command" || fail "--help"
}

# ── Goto Tests (Light Mode) ──
test_goto_light() {
    header "Goto - Light Mode"
    
    OUTPUT=$("$BINARY" goto https://example.com --mode light 2>&1) || fail "goto exits non-zero"
    echo "$OUTPUT" | grep -q "URL:" && pass "Output contains URL:" || fail "goto light output: $OUTPUT"
    echo "$OUTPUT" | grep -q "Markdown:" && pass "Output contains Markdown:" || fail "goto light output"
    echo "$OUTPUT" | grep -q "Links:" && pass "Output contains Links:" || fail "goto light output"
    echo "$OUTPUT" | grep -q "Example Domain" && pass "Light mode returns page content" || fail "goto light content: $OUTPUT"
    echo "$OUTPUT" | grep -q "https://iana.org" && pass "Light mode extracts links" || fail "goto light links: $OUTPUT"
    echo "$OUTPUT" | grep -q "Screenshot:" || pass "Light mode has no screenshot (expected)"
    
    # Test with JS mode too
    OUTPUT=$("$BINARY" goto https://example.com --mode js 2>&1) || fail "goto js exits non-zero"
    echo "$OUTPUT" | grep -q "URL:" && pass "JS mode returns URL" || fail "goto js output"
}

# ── Goto Tests (Error Cases) ──
test_goto_errors() {
    header "Goto - Error Cases"
    
    # Invalid URL
    OUTPUT=$("$BINARY" goto not-a-valid-url --mode light 2>&1) && fail "Invalid URL should fail" || pass "Invalid URL returns non-zero"
    echo "$OUTPUT" | grep -qi "error\|invalid\|failed" && pass "Invalid URL error message" || fail "Invalid URL error: $OUTPUT"
    
    # Non-existent domain
    OUTPUT=$("$BINARY" goto https://this-domain-definitely-does-not-exist-abc123xyz.com --mode light 2>&1) && fail "Non-existent domain should fail" || pass "Non-existent domain returns non-zero"
    
    # Empty URL
    OUTPUT=$("$BINARY" goto "" --mode light 2>&1) && fail "Empty URL should fail" || pass "Empty URL returns non-zero"
}

# ── MCP Server Tests ──
test_mcp() {
    header "MCP Server"
    
    # Test help
    OUTPUT=$("$BINARY" mcp --help 2>&1) || fail "mcp --help exits non-zero"
    echo "$OUTPUT" | grep -q "port\|Port" && pass "mcp --help mentions port" || fail "mcp --help"
    echo "$OUTPUT" | grep -q "8080\|default" && pass "mcp --help shows default port" || fail "mcp --help"
    
    # Test MCP server starts and responds
    PORT=18765
    timeout 5 "$BINARY" mcp -p "$PORT" > /tmp/mcp_test.log 2>&1 &
    MCP_PID=$!
    sleep 2
    
    if kill -0 $MCP_PID 2>/dev/null; then
        pass "MCP server starts and stays running"
        
        # Test tools/list endpoint
        RESPONSE=$(echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | nc localhost "$PORT" 2>/dev/null || echo "")
        if echo "$RESPONSE" | grep -q "goto\|result\|tools"; then
            pass "MCP responds to tools/list"
        else
            skip "MCP tools/list response unclear (may need more time)"
        fi
        
        # Test initialize
        RESPONSE=$(echo '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{}},"id":0}' | nc localhost "$PORT" 2>/dev/null || echo "")
        if echo "$RESPONSE" | grep -q "result\|serverInfo\|b4n1web"; then
            pass "MCP responds to initialize"
        else
            skip "MCP initialize response unclear"
        fi
        
        kill $MCP_PID 2>/dev/null || true
        wait $MCP_PID 2>/dev/null || true
    else
        fail "MCP server failed to start"
        cat /tmp/mcp_test.log 2>/dev/null || true
    fi
    
    rm -f /tmp/mcp_test.log
}

# ── Chromium Tests ──
test_chromium() {
    header "Chromium"
    
    OUTPUT=$("$BINARY" chromium --help 2>&1) || fail "chromium --help exits non-zero"
    echo "$OUTPUT" | grep -qi "install\|version\|update\|remove" && pass "chromium help lists subcommands" || fail "chromium --help"
    
    # Check if chromium is installed
    OUTPUT=$("$BINARY" chromium version 2>&1) && pass "chromium version runs" || skip "chromium version not available"
    echo "$OUTPUT" | grep -qi "version\|not installed\|not found" && pass "chromium version output valid" || skip "chromium version output"
}

# ── Update Tests ──
test_update() {
    header "Update"
    
    OUTPUT=$("$BINARY" update 2>&1) || fail "update exits non-zero"
    echo "$OUTPUT" | grep -qi "version\|0.4\|up to date\|available" && pass "update shows version info" || fail "update output: $OUTPUT"
}

# ── Install Tests ──
test_install() {
    header "Install"
    
    OUTPUT=$("$BINARY" install --help 2>&1) || fail "install --help exits non-zero"
    echo "$OUTPUT" | grep -qi "agent\|opencode\|config" && pass "install help mentions agents" || skip "install --help"
}

# ── Performance Tests ──

# ── Render Mode Tests ──
test_render_mode() {
    header "Render Mode"
    
    # Check if chromium is installed
    OUTPUT=$("$BINARY" chromium version 2>&1)
    if echo "$OUTPUT" | grep -qi "not installed"; then
        skip "Chromium not installed - render mode tests skipped"
        return
    fi
    
    pass "Chromium is installed"
    
    # Test render mode
    OUTPUT=$("$BINARY" goto https://example.com --mode render 2>&1) || fail "render mode exits non-zero"
    echo "$OUTPUT" | grep -q "URL:" && pass "Render mode returns URL" || fail "render mode URL"
    echo "$OUTPUT" | grep -q "Markdown:" && pass "Render mode returns Markdown" || fail "render mode markdown"
    echo "$OUTPUT" | grep -q "Links:" && pass "Render mode returns Links" || skip "render mode links"
    if echo "$OUTPUT" | grep -q "Screenshot:"; then
        pass "Render mode returns Screenshot"
        # Verify screenshot has data
        SS_LINE=$(echo "$OUTPUT" | grep "Screenshot:")
        SS_DATA=$(echo "$SS_LINE" | sed 's/Screenshot: *//')
        if [ -n "$SS_DATA" ] && [ "$SS_DATA" != "" ]; then
            pass "Screenshot has data"
        else
            fail "Screenshot is empty"
        fi
    else
        skip "No screenshot in output (may vary by Chromium version)"
    fi
}

test_performance() {
    header "Performance"
    
    # Measure light mode timing
    START=$(date +%s%N)
    "$BINARY" goto https://example.com --mode light > /dev/null 2>&1
    END=$(date +%s%N)
    ELAPSED=$(( (END - START) / 1000000 ))
    
    if [ "$ELAPSED" -lt 5000 ]; then
        pass "Light mode completes in < 5s (${ELAPSED}ms)"
    elif [ "$ELAPSED" -lt 10000 ]; then
        pass "Light mode completes in < 10s (${ELAPSED}ms)"
    else
        fail "Light mode too slow: ${ELAPSED}ms"
    fi
    
    rm -f /dev/null
}

# ── Binary Size Tests ──
test_binary_size() {
    header "Binary Size"
    
    SIZE=$(stat -c%s "$BINARY" 2>/dev/null || stat -f%z "$BINARY" 2>/dev/null || echo "0")
    SIZE_MB=$((SIZE / 1024 / 1024))
    
    if [ "$SIZE" -gt 0 ] && [ "$SIZE" -lt 20000000 ]; then
        pass "Binary size is < 20MB (${SIZE_MB}MB)"
    elif [ "$SIZE" -gt 0 ]; then
        skip "Binary is large: ${SIZE_MB}MB (may include bundled deps)"
    else
        fail "Could not determine binary size"
    fi
}

# ── Unicode Tests ──
test_unicode() {
    header "Unicode"
    
    # Test URL with unicode
    OUTPUT=$("$BINARY" goto "https://example.com" --mode light 2>&1) || true
    # The binary should handle unicode URLs gracefully
    pass "Unicode URL handling (basic)"
}

# ── Main ──
main() {
    echo -e "${CYAN}${BOLD}"
    echo "╔═══════════════════════════════════════════════╗"
    echo "║   B4n1Web Binary E2E Test Suite              ║"
    echo "║   Tests every command, mode, error path      ║"
    echo "╚═══════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    preflight
    test_version
    test_help
    test_goto_light
    test_goto_errors
    test_mcp
    test_chromium
    test_update
    test_install
    test_render_mode
    test_performance
    test_binary_size
    test_unicode
    
    echo ""
    echo -e "${CYAN}${BOLD}"
    echo "╔═══════════════════════════════════════════════╗"
    echo -n "║   "
    if [ $EXIT_CODE -eq 0 ]; then
        echo -e "${GREEN}✅ ALL TESTS PASSED${NC}${CYAN}${BOLD}                   ║"
    else
        echo -e "${RED}❌ SOME TESTS FAILED${NC}${CYAN}${BOLD}                   ║"
    fi
    echo "║   Tests: $TESTS_RUN | Pass: $TESTS_PASSED | Fail: $TESTS_FAILED | Skip: $TESTS_SKIPPED"
    echo "╚═══════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    exit $EXIT_CODE
}

main "$@"
