# Release history — b4n1-web

The current release is **0.16.4**. Every language package on every channel
carries the same version and ships the prebuilt engine for its platform, so
nothing is compiled on the client machine.

## 0.16.4 — 2026-11-25

- Version alignment across the seven language packages: one version number
  identifies the whole family.

## 0.16.3 — 2026-11-18

- `b4n1web docs` — the complete manual ships inside the engine, so it reads
  offline on your own machine (`b4n1web docs --section <name>` for one chapter).
- `b4n1web demo` — one command runs a self-contained tour, with no network and
  nothing to configure.

## 0.16.2 — 2026-11-16

- Every language package states the same capabilities: 7 SDKs, 35 MCP tools and
  6 prebuilt platform builds.
- Each package documents installing with its own registry command only.

## 0.16.1 — 2026-11-15

- The registry page of each package shows a README written for that language
  (installation, page object, interactions, modes, security, errors, MCP)
  instead of sharing one across languages.
- Published license text states Business Source License 1.1 (BUSL-1.1) in every
  package.

## 0.16.0 — 2026-10-15

- JavaScript `getLinks()` and `getLinksFromPage(url)` reach parity with the other
  six packages: the links of the last visited page, or of any URL in one call.
- Errors are reported to the caller instead of being folded into an empty list.

## 0.15.0 — 2026-10-13

- New MCP tool `get_links`: the href and text of a page's links, with `mailto:`
  and `tel:` left out.
- Global `--licenses` flag prints the full license and the third-party notices.
  It needs no key and no network.
- `evaluate` gates its mode explicitly: asking for a mode the call cannot honour
  fails with an actionable message instead of returning an empty result.
- `goto(url, selector)` waits for the selector and, if it never appears, fails
  naming it instead of reporting success.
- Rendered text no longer includes script or stylesheet bodies.
- Background browser instances are closed deterministically, so repeated runs do
  not leave processes behind.

## 0.13.0 — 2026-09-02

- Release builds are fully optimised, stripped of internal names and free of
  build-path strings.

## 0.12.3 — 2026-07-08

- Python: screenshots returned by the engine are parsed correctly.

## 0.12.2 — 2026-07-08

- Linux builds are fully self-contained: no system library and no runtime to
  install.

## 0.12.0 — 2026-07-03

- A newer version is announced in the background on startup, without holding up
  the command you ran.

## 0.11.0 — 2026-07-01

- Builds for Linux (x64, arm64, 32-bit), macOS (x64, arm64) and Windows (x64,
  arm64, 32-bit), each shipped inside the packages.

## 0.10.0 — 2026-07-01

- PDFs produced through the MCP server carry the page's real content.
- One version number across the engine, every language package, the MCP server
  and the documentation.

Versions 0.9.x and below are superseded.
