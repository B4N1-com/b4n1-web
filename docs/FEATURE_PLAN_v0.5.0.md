# Feature Plan: v0.5.0

## Features to Implement

### 1. `--wait-for <selector>` Flag

#### Problem
When using **RENDER mode** on SPAs (Single Page Applications) like HackerOne, GitHub, etc., the binary returns markdown **before** dynamic content finishes loading. This results in incomplete data (e.g., "JavaScript disabled" messages instead of actual content).

#### Solution
Add `--wait-for <css-selector>` flag to the binary CLI. The browser waits until the specified CSS selector appears in the DOM before extracting content.

#### Binary Changes (Rust - `engine/cli-core/`)

**CLI argument:**
```rust
// In cli-core/src/main.rs or equivalent
#[arg(long)]
wait_for: Option<String>,
```

**Browser execution:**
```rust
// In the render mode browser loop
if let Some(selector) = &args.wait_for {
    // Wait up to timeout for selector to appear
    let deadline = Instant::now() + Duration::from_secs(timeout);
    loop {
        if page.query_selector(selector).is_some() {
            break; // Element found, proceed
        }
        if Instant::now() > deadline {
            eprintln!("⚠️  Timeout waiting for selector: {}", selector);
            break; // Timeout, return what we have
        }
        sleep(Duration::from_millis(100)).await;
    }
}
```

**SDK Changes (Public Repo)**

| SDK | Change |
|-----|--------|
| **Python** | `browser.goto(url, wait_for="#app")` → adds `--wait-for #app` to subprocess args |
| **JavaScript** | `browser.goto(url, { waitFor: '#app' })` → same |
| **Go** | `WithWaitFor(selector string) BrowserOption` → same |
| **Java** | `browserOptions.setWaitFor("#app")` → same |
| **C#** | `new BrowserOptions { WaitFor = "#app" }` → same |

**Example Usage:**
```python
# Python
browser = AgentBrowser(mode=BrowserMode.RENDER, wait_for="#content-loaded")
page = browser.goto("https://hackerone.com")
# Now page.markdown has the dynamically loaded content
```

```typescript
// JavaScript
const browser = new AgentBrowser({ mode: BrowserMode.RENDER, waitFor: '#app-root' });
const page = await browser.goto('https://spa.example.com');
```

**Test Plan (Private Repo):**
1. Create a test HTML page with delayed content injection (setTimeout 2s)
2. Test without `--wait-for`: should return incomplete content
3. Test with `--wait-for="#dynamic-content"`: should return complete content
4. Test timeout scenario: selector never appears → should warn and return what's available

---

### 2. `--only-repos` Flag (Smart Extraction)

#### Problem
When scraping GitHub/GitLab pages, the `links` array includes many URLs that aren't useful for agents wanting to clone repositories:
- `/blob/master/file.py` → should become repository root
- `/tree/main/src/` → should become repository root
- `#readme` → fragment, useless for cloning
- `/issues`, `/pulls`, `/actions` → not repo URLs

Agents currently have to clean these URLs manually before `git clone`.

#### Solution
Add `--only-repos` flag that filters and normalizes links to only return **base repository URLs**.

#### Binary Changes (Rust - `engine/cli-core/`)

**CLI argument:**
```rust
#[arg(long)]
only_repos: bool,
```

**Link filtering logic:**
```rust
fn extract_repo_url(url: &str) -> Option<String> {
    // Only process GitHub/GitLab URLs
    if !url.contains("github.com") && !url.contains("gitlab.com") {
        return None;
    }

    // Remove /blob/, /tree/, /raw/ paths
    let path = url
        .replace("/blob/", "/")
        .replace("/tree/", "/")
        .replace("/raw/", "/");

    // Remove fragments (#readme, #contents, etc.)
    let path = path.split('#').next().unwrap_or(&path);

    // Normalize: keep only /owner/repo
    let parts: Vec<&str> = path.trim_end_matches('/').split('/').collect();
    if parts.len() >= 3 {
        // Return just https://github.com/owner/repo
        Some(format!("{}/{}/{}", parts[0], parts[1], parts[2]))
    } else {
        Some(path.to_string())
    }
}

fn filter_links(links: &[String], only_repos: bool) -> Vec<String> {
    if !only_repos {
        return links.to_vec();
    }

    let mut seen = HashSet::new();
    links.iter()
        .filter_map(|url| extract_repo_url(url))
        .filter(|url| seen.insert(url.clone())) // deduplicate
        .collect()
}
```

**SDK Changes (Public Repo)**

| SDK | Change |
|-----|--------|
| **Python** | `page.links_only_repos()` method or `browser.goto(url, only_repos=True)` |
| **JavaScript** | `page.getRepoLinks()` method or `browser.goto(url, { onlyRepos: true })` |
| **Go** | `page.GetRepoLinks() []string` or `WithOnlyRepos(true)` option |
| **Java** | `page.getRepoLinks()` method or `browserOptions.setOnlyRepos(true)` |
| **C#** | `page.GetRepoLinks()` method or `new BrowserOptions { OnlyRepos = true }` |

**Example Usage:**
```python
# Python
page = browser.goto("https://github.com/trending")
repo_links = page.links_only_repos()
# Returns: ["https://github.com/user/repo1", "https://github.com/user/repo2", ...]
# NOT: ["https://github.com/user/repo1/blob/main/file.py", ...]
```

**Test Plan (Private Repo):**
1. Input: list of mixed URLs (blob, tree, raw, fragments, non-repo)
2. Output: only base repo URLs, deduplicated
3. Edge cases: non-GitHub/GitLab URLs (return as-is or filter out?)
4. Test with real GitHub trending page

---

## Implementation Order

### Phase 1: Binary (Private Repo)
1. `--wait-for` flag in Rust binary
2. `--only-repos` flag + link filtering in Rust binary
3. Binary version bump to `0.5.0`

### Phase 2: SDKs (Public Repo)
1. Python: add `wait_for`, `only_repos` params
2. JavaScript: add `waitFor`, `onlyRepos` options
3. Go: add `WithWaitFor`, `WithOnlyRepos` options
4. Java: add `setWaitFor`, `setOnlyRepos` methods
5. C#: add `WaitFor`, `OnlyRepos` properties
6. Sync all SDK versions to `0.5.0`

### Phase 3: Tests (Private Repo)
1. Add tests to `tests/python/` for new features
2. Add tests to `internal/tests/javascript/`
3. Add tests to `internal/tests/go/`
4. Add tests to `internal/tests/java/`
5. Add tests to `internal/tests/csharp/`
6. Add Podman E2E tests per `PODMAN_E2E_STRATEGY.md`

### Phase 4: Documentation
1. Update README.md in public repo
2. Update RELEASE_WORKFLOW.md
3. Update this document with implementation status

---

## Notes

- **`--wait-for` only works in RENDER mode**: Light and JS modes don't execute JavaScript, so waiting for a selector is meaningless. Should return error if used with wrong mode.
- **`--only-repos` works with any mode**: It's a post-processing step on the links array.
- **Timeout behavior**: `--wait-for` should NOT fail if selector doesn't appear — just warn and return what's available. This is important for agent workflows.
- **Performance**: `--wait-for` adds polling overhead (100ms intervals). Should document this.
