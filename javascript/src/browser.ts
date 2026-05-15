import { execSync } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import { BrowserMode, type BrowserOptions, type PageData, type IframeInfo, BinaryNotFoundError } from './types';

export function getB4n1webBinary(): string | null {
  const home = process.env.HOME || process.env.USERPROFILE || '/home/' + process.env.USER;
  const paths: string[] = [];

  const bundledBinary = path.join(__dirname, '..', 'bin', 'b4n1web-linux');
  if (fs.existsSync(bundledBinary)) {
    try {
      fs.accessSync(bundledBinary, fs.constants.X_OK);
      return bundledBinary;
    } catch {
    }
  }

  paths.push(
    '/usr/local/bin/b4n1web',
    '/usr/bin/b4n1web',
    home + '/.local/bin/b4n1web',
    home + '/.b4n1web/bin/b4n1web',
  );

  const pathEnv = process.env.PATH || '';
  const pathDirs = pathEnv.split(':').filter(p => p);
  for (const dir of pathDirs) {
    paths.push(dir + '/b4n1web');
  }

  for (const filePath of paths) {
    try {
      if (fs.existsSync(filePath)) {
        const stats = fs.statSync(filePath);
        if (stats.isFile() && (stats.mode & 0o111) !== 0) {
          return filePath;
        }
      }
    } catch {
    }
  }
  return null;
}

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

const SDK_VERSION = '0.7.0';

export function checkVersionCompatibility(): string | null {
  const binaryVersion = getB4n1webVersion();
  if (!binaryVersion) {
    return null;
  }

  if (binaryVersion !== SDK_VERSION) {
    console.warn(
      `Warning: Version mismatch: SDK v${SDK_VERSION} requires binary v${SDK_VERSION}, ` +
      `but found v${binaryVersion}. Some features may not work correctly. ` +
      `To update: curl -sL https://web.b4n1.com/install | bash`
    );
  }
  return binaryVersion;
}

export class Page implements PageData {
  url: string;
  markdown: string;
  links: string[];
  screenshot?: string;
  jsOutput?: string;

  constructor(data: PageData) {
    this.url = data.url;
    this.markdown = data.markdown;
    this.links = data.links;
    this.screenshot = data.screenshot;
    this.jsOutput = data.jsOutput;
  }

  getMainContent(): string {
    const lines = this.markdown.split('\n');
    const contentLines = lines.length > 2 ? lines.slice(2) : lines;
    return contentLines.join('\n').trim();
  }

  findLinksByText(text: string): string[] {
    const lowerText = text.toLowerCase();
    return this.links.filter(link => link.toLowerCase().includes(lowerText));
  }
}

export class AgentBrowser {
  private mode: BrowserMode;
  private timeout: number;
  private userAgent: string;
  private binaryPath!: string;
  private currentUrl: string | null = null;
  private sessionId: string;
  private sessionStarted: boolean = false;
  private viewportWidth: number = 1280;
  private viewportHeight: number = 720;
  private emulatedDevice: string | null = null;

  constructor(options: BrowserOptions = {}) {
    this.mode = options.mode ?? BrowserMode.LIGHT;
    this.timeout = options.timeout ?? 30;
    this.userAgent = options.userAgent ?? 'B4n1Web-Agent/1.0';
    this.sessionId = `agent-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;

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
        `For MCP server: curl -sL https://web.b4n1.com/install | bash`
      );
    }
    this.binaryPath = binary;
  }

  private escapeArg(arg: string): string {
    return JSON.stringify(arg);
  }

  private runSessionCommand(subcommand: string, ...args: string[]): string {
    try {
      return execSync(
        `${this.binaryPath} session ${subcommand} ${this.sessionId} ${args.join(' ')}`,
        { timeout: this.timeout * 1000 }
      ).toString().trim();
    } catch (error: any) {
      if (error.message?.includes('timed out')) {
        throw new Error(`Session command timed out after ${this.timeout}s`);
      }
      throw new Error(`Session command "${subcommand}" failed: ${error.message}`);
    }
  }

  private ensureSession(url?: string): void {
    if (!this.sessionStarted) {
      execSync(`${this.binaryPath} session start ${this.sessionId}`, { timeout: 10000 });
      this.sessionStarted = true;
    }
    if (url) {
      execSync(
        `${this.binaryPath} session goto ${this.sessionId} ${this.escapeArg(url)}`,
        { timeout: this.timeout * 1000 }
      );
    }
  }

  async goto(url: string, waitFor?: string): Promise<Page> {
    this.currentUrl = url;
    return new Promise((resolve, reject) => {
      try {
        let cmd = `${this.binaryPath} goto ${url} --mode ${this.mode}`;
        if (waitFor) {
          cmd += ` --wait-for ${JSON.stringify(waitFor)}`;
        }
        const output = execSync(cmd, { timeout: this.timeout * 1000 }).toString();
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

  private parseOutput(url: string, output: string): Page {
    let markdown = '';
    let links: string[] = [];
    let screenshot: string | undefined;
    let jsOutput: string | undefined;

    for (const line of output.split('\n')) {
      if (line.startsWith('URL:')) {
        continue;
      } else if (line.startsWith('Markdown:')) {
        continue;
      } else if (line.startsWith('Links:')) {
        try {
          links = JSON.parse(line.slice(6).trim());
        } catch {
          links = [];
        }
      } else if (line.startsWith('Screenshot:')) {
        screenshot = line.slice(10).trim() || undefined;
      } else if (line.startsWith('js_output:')) {
        jsOutput = line.slice(10).trim() || undefined;
      } else {
        markdown += line + '\n';
      }
    }

    return new Page({
      url,
      markdown: markdown.trim(),
      links,
      screenshot,
      jsOutput,
    });
  }

  async click(selector: string): Promise<void> {
    if (!this.currentUrl) {
      throw new Error('No page loaded. Call goto() first.');
    }
    this.ensureSession(this.currentUrl);
    try {
      execSync(
        `${this.binaryPath} session click ${this.sessionId} ${this.escapeArg(selector)}`,
        { timeout: this.timeout * 1000 }
      );
    } catch (error: any) {
      throw new Error(`Click failed on "${selector}": ${error.message}`);
    }
  }

  async typeText(selector: string, text: string, clearFirst: boolean = false): Promise<void> {
    if (!this.currentUrl) {
      throw new Error('No page loaded. Call goto() first.');
    }
    this.ensureSession(this.currentUrl);
    try {
      let cmd = `${this.binaryPath} session type ${this.sessionId} ${this.escapeArg(selector)} ${this.escapeArg(text)}`;
      if (clearFirst) {
        cmd += ' --clear-first';
      }
      execSync(cmd, { timeout: this.timeout * 1000 });
    } catch (error: any) {
      throw new Error(`Type text failed on "${selector}": ${error.message}`);
    }
  }

  async waitForSelector(selector: string, timeoutMs: number = 10000): Promise<boolean> {
    if (!this.currentUrl) {
      throw new Error('No page loaded. Call goto() first.');
    }
    this.ensureSession(this.currentUrl);
    try {
      const output = execSync(
        `${this.binaryPath} session wait ${this.sessionId} ${this.escapeArg(selector)} --timeout-ms ${timeoutMs}`,
        { timeout: Math.ceil(timeoutMs / 1000) + 5 }
      ).toString().trim();
      return output === 'true';
    } catch {
      return false;
    }
  }

  async screenshot(url?: string): Promise<string> {
    const targetUrl = url || this.currentUrl;
    if (!targetUrl) {
      throw new Error('No URL provided. Pass a URL or call goto() first.');
    }
    this.ensureSession(targetUrl);
    try {
      const output = execSync(
        `${this.binaryPath} session screenshot ${this.sessionId} ${this.escapeArg(targetUrl)}`,
        { timeout: this.timeout * 1000 }
      ).toString().trim();
      return output;
    } catch (error: any) {
      if (error.message?.includes('timed out')) {
        throw new Error(`Screenshot timed out after ${this.timeout}s`);
      }
      throw new Error(`Screenshot failed: ${error.message}`);
    }
  }

  async frames(): Promise<IframeInfo[]> {
    if (!this.currentUrl) {
      throw new Error('No page loaded. Call goto() first.');
    }
    this.ensureSession(this.currentUrl);
    try {
      const output = this.runSessionCommand('frames');
      return JSON.parse(output) as IframeInfo[];
    } catch (error: any) {
      throw new Error(`Failed to list frames: ${error.message}`);
    }
  }

  async iframeText(index: number): Promise<string> {
    if (!this.currentUrl) {
      throw new Error('No page loaded. Call goto() first.');
    }
    this.ensureSession(this.currentUrl);
    try {
      return this.runSessionCommand('iframe-text', String(index));
    } catch (error: any) {
      throw new Error(`Failed to get iframe text at index ${index}: ${error.message}`);
    }
  }

  async setViewport(width: number, height: number): Promise<void> {
    this.viewportWidth = width;
    this.viewportHeight = height;
    if (this.sessionStarted && this.currentUrl) {
      try {
        execSync(
          `${this.binaryPath} session goto ${this.sessionId} ${this.escapeArg(this.currentUrl)} --viewport-width ${width} --viewport-height ${height}`,
          { timeout: this.timeout * 1000 }
        );
      } catch (error: any) {
        throw new Error(`setViewport failed: ${error.message}`);
      }
    }
  }

  async emulateDevice(device: string): Promise<void> {
    this.emulatedDevice = device;
    if (this.sessionStarted && this.currentUrl) {
      try {
        execSync(
          `${this.binaryPath} session goto ${this.sessionId} ${this.escapeArg(this.currentUrl)} --emulate ${this.escapeArg(device)}`,
          { timeout: this.timeout * 1000 }
        );
      } catch (error: any) {
        throw new Error(`emulateDevice failed: ${error.message}`);
      }
    }
  }

  getLinks(): string[] {
    if (!this.currentUrl) {
      throw new Error('No page loaded. Call goto() first.');
    }
    try {
      const output = execSync(
        `${this.binaryPath} get-links --url ${this.currentUrl} --mode ${this.mode}`,
        { timeout: this.timeout * 1000 }
      ).toString().trim();
      try {
        return JSON.parse(output);
      } catch {
        return [];
      }
    } catch {
      return [];
    }
  }

  static async getLinksFromPage(url: string, mode: BrowserMode = BrowserMode.LIGHT): Promise<string[]> {
    const browser = new AgentBrowser({ mode });
    try {
      const page = await browser.goto(url);
      return page.links;
    } finally {
      browser.close();
    }
  }

  static findBinary(): string | null {
    return getB4n1webBinary();
  }

  close(): void {
    if (this.sessionStarted) {
      try {
        execSync(`${this.binaryPath} session close ${this.sessionId}`, { timeout: 5000 });
      } catch {
      }
      this.sessionStarted = false;
    }
  }

  async [Symbol.asyncDispose]() {
    this.close();
  }
}

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
