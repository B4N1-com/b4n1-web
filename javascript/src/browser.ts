/**
 * B4n1Web Browser - JavaScript/TypeScript Implementation
 */

import { execSync } from 'child_process';
import { BrowserMode, type BrowserOptions, type PageData, BinaryNotFoundError } from './types';

/**
 * Find the b4n1web binary in common locations
 */
function getB4n1webBinary(): string | null {
  const home = process.env.HOME || process.env.USERPROFILE || '/home/' + process.env.USER;
  const paths = [
    '/usr/local/bin/b4n1web',
    '/usr/bin/b4n1web',
    home + '/.local/bin/b4n1web',
    home + '/.b4n1web/bin/b4n1web',
  ];
  
  // Also check PATH
  const pathEnv = process.env.PATH || '';
  const pathDirs = pathEnv.split(':').filter(p => p);
  for (const dir of pathDirs) {
    paths.push(dir + '/b4n1web');
  }

  for (const path of paths) {
    try {
      const fs = require('fs');
      if (fs.existsSync(path)) {
        const stats = fs.statSync(path);
        if (stats.isFile() && (stats.mode & 0o111) !== 0) {
          return path;
        }
      }
    } catch {
      // Path doesn't exist or not executable
    }
  }
  return null;
}

/**
 * Get B4n1Web binary version
 */
export function getB4n1webVersion(): string | null {
  const binaryPath = getB4n1webBinary();
  if (!binaryPath) {
    return null;
  }
  try {
    const version = execSync(`${binaryPath} --version`, { timeout: 5000 }).toString().trim();
    const parts = version.split(' ');
    if (parts.length >= 2 && parts[0] === 'b4n1web') {
      return parts[1];
    }
    return null;
  } catch {
    return null;
  }
}

const SDK_VERSION = '0.4.0';

/**
 * Check version compatibility and warn if mismatch
 */
export function checkVersionCompatibility(): string | null {
  const binaryVersion = getB4n1webVersion();
  if (!binaryVersion) {
    return null;
  }
  
  if (binaryVersion !== SDK_VERSION) {
    console.warn(
      `⚠️  Version mismatch: SDK v${SDK_VERSION} requires binary v${SDK_VERSION}, ` +
      `but found v${binaryVersion}. Some features may not work correctly. ` +
      `To update: curl -sL https://web.b4n1.com/install | bash`
    );
  }
  return binaryVersion;
}

/**
 * Page data returned by B4n1Web
 */
export class Page implements PageData {
  url: string;
  markdown: string;
  links: string[];
  screenshot?: string;

  constructor(data: PageData) {
    this.url = data.url;
    this.markdown = data.markdown;
    this.links = data.links;
    this.screenshot = data.screenshot;
  }

  /**
   * Extract main content from markdown, skipping headers
   */
  getMainContent(): string {
    const lines = this.markdown.split('\n');
    const contentLines = lines.length > 2 ? lines.slice(2) : lines;
    return contentLines.join('\n').trim();
  }

  /**
   * Find links containing specific text
   */
  findLinksByText(text: string): string[] {
    const lowerText = text.toLowerCase();
    return this.links.filter(link => link.toLowerCase().includes(lowerText));
  }
}

/**
 * B4n1Web Agent Browser
 * 
 * A browser instance optimized for AI agent workflows.
 * Requires B4n1Web binary to be installed.
 * 
 * @example
 * ```typescript
 * import { AgentBrowser, BrowserMode } from 'b4n1-web';
 * 
 * const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
 * const page = await browser.goto('https://example.com');
 * console.log(page.markdown);
 * browser.close();
 * ```
 */
export class AgentBrowser {
  private mode: BrowserMode;
  private timeout: number;
  private userAgent: string;
  private binaryPath!: string;

  constructor(options: BrowserOptions = {}) {
    this.mode = options.mode ?? BrowserMode.LIGHT;
    this.timeout = options.timeout ?? 30;
    this.userAgent = options.userAgent ?? 'B4n1Web-Agent/1.0';
    
    const binary = getB4n1webBinary();
    if (!binary) {
      const home = process.env.HOME || '';
      const pathDirs = (process.env.PATH || '').split(':').filter(p => p);
      throw new BinaryNotFoundError(
        `b4n1web binary not found.\nChecked paths:\n` +
        `- /usr/local/bin/b4n1web\n` +
        `- /usr/bin/b4n1web\n` +
        `- ${home}/.local/bin/b4n1web\n` +
        `- ${home}/.b4n1web/bin/b4n1web\n` +
        `- PATH: ${pathDirs.join(', ')}\n\n` +
        `Run: curl -sL https://web.b4n1.com/install | bash`
      );
    }
    this.binaryPath = binary;
  }

  /**
   * Navigate to a URL and extract structured content
   */
  async goto(url: string): Promise<Page> {
    return new Promise((resolve, reject) => {
      try {
        const output = execSync(
          `${this.binaryPath} goto ${url} --mode ${this.mode}`,
          { timeout: this.timeout * 1000 }
        ).toString();

        const page = this.parseOutput(url, output);
        resolve(page);
      } catch (error: any) {
        if (error.message?.includes('timed out')) {
          reject(new Error(`Binary timed out after ${this.timeout}s`));
        } else {
          reject(new Error(`Binary error: ${error.message}`));
        }
      }
    });
  }

  /**
   * Parse text output from the binary
   */
  private parseOutput(url: string, output: string): Page {
    let markdown = '';
    let links: string[] = [];
    let screenshot: string | undefined;

    for (const line of output.split('\n')) {
      if (line.startsWith('URL:')) {
        continue;
      } else if (line.startsWith('Markdown:')) {
        continue;
      } else if (line.startsWith('Links:')) {
        try {
          links = eval(line.slice(6).trim()); // Safe: we control the output format
        } catch {
          links = [];
        }
      } else if (line.startsWith('Screenshot:')) {
        screenshot = line.slice(10).trim() || undefined;
      } else {
        markdown += line + '\n';
      }
    }

    return new Page({
      url,
      markdown: markdown.trim(),
      links,
      screenshot,
    });
  }

  /**
   * Close the browser session
   */
  close(): void {
    // No persistent session to close in current implementation
  }

  /**
   * Use as async context manager
   */
  async [Symbol.asyncDispose]() {
    this.close();
  }
}

/**
 * Create a browser and navigate in one go
 */
export async function createBrowserAndGoto(
  url: string,
  options: BrowserOptions = {}
): Promise<Page> {
  const browser = new AgentBrowser(options);
  try {
    return await browser.goto(url);
  } finally {
    browser.close();
  }
}
