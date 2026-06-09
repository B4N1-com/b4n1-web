import * as os from 'os';
import { execSync } from 'child_process';
import { BrowserMode, type BrowserOptions, BinaryNotFoundError } from './types';
import { Page } from './page';
import { getB4n1webBinary } from './binary';

export { getB4n1webBinary, getB4n1webVersion, checkVersionCompatibility } from './binary';
export { Page } from './page';

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
      const home = os.homedir();
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
    let inMarkdown = false;
    let inJsOutput = false;
    let hasStructuredData = false;

    for (const line of output.split('\n')) {
      if (line.startsWith('URL:')) {
        hasStructuredData = true;
        inMarkdown = false;
        inJsOutput = false;
      } else if (line.startsWith('Markdown:')) {
        hasStructuredData = true;
        inMarkdown = true;
        inJsOutput = false;
        const content = line.substring(9).trim();
        if (content) markdown += content + '\n';
      } else if (line.startsWith('Links:')) {
        hasStructuredData = true;
        inMarkdown = false;
        inJsOutput = false;
        try {
          links = JSON.parse(line.substring(6).trim());
        } catch {
          links = [];
        }
      } else if (line.startsWith('Screenshot:')) {
        hasStructuredData = true;
        inMarkdown = false;
        inJsOutput = false;
        const s = line.substring(11).trim();
        if (s) screenshot = s;
      } else if (line.startsWith('JS Output:')) {
        hasStructuredData = true;
        inMarkdown = false;
        inJsOutput = true;
        const content = line.substring(10).trim();
        if (content) jsOutput = (jsOutput || '') + content + '\n';
      } else {
        if (inMarkdown) {
          markdown += line + '\n';
        } else if (inJsOutput) {
          jsOutput = (jsOutput || '') + line + '\n';
        }
      }
    }

    if (!hasStructuredData && output.trim()) {
      markdown = output.trim();
    } else {
      markdown = markdown.trim();
      if (jsOutput) jsOutput = jsOutput.trim();
    }

    return new Page({
      url,
      markdown,
      links,
      screenshot,
      jsOutput,
    });
  }

  async [Symbol.asyncDispose](): Promise<void> {
    this.close();
  }

  async click(selector: string): Promise<string> {
    this.ensureSession(this.currentUrl || undefined);
    return this.runSessionCommand('click', this.escapeArg(selector));
  }

  async typeText(selector: string, text: string, clearFirst: boolean = false): Promise<string> {
    this.ensureSession(this.currentUrl || undefined);
    return this.runSessionCommand(
      'type-text',
      this.escapeArg(selector),
      this.escapeArg(text),
      clearFirst ? '--clear' : ''
    );
  }

  async waitForSelector(selector: string, timeoutMs: number = 5000): Promise<string> {
    this.ensureSession(this.currentUrl || undefined);
    return this.runSessionCommand('wait-for', this.escapeArg(selector), timeoutMs.toString());
  }

  async screenshot(url: string, fullPage: boolean = false): Promise<string> {
    this.ensureSession(url);
    return this.runSessionCommand('screenshot', fullPage ? '--full-page' : '');
  }

  async setViewport(width: number, height: number): Promise<void> {
    this.viewportWidth = width;
    this.viewportHeight = height;
    this.ensureSession();
    this.runSessionCommand('set-viewport', width.toString(), height.toString());
  }

  async setUserAgent(ua: string): Promise<void> {
    this.userAgent = ua;
    this.ensureSession();
    this.runSessionCommand('set-user-agent', this.escapeArg(ua));
  }

  async emulateDevice(device: string): Promise<void> {
    this.emulatedDevice = device;
    this.ensureSession();
    this.runSessionCommand('emulate-device', this.escapeArg(device));
  }

  close(): void {
    if (this.sessionStarted) {
      try {
        execSync(`${this.binaryPath} session close ${this.sessionId}`, { timeout: 5000 });
      } catch {
        // Ignore errors on close
      }
      this.sessionStarted = false;
    }
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
