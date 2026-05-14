export enum BrowserMode {
  LIGHT = 'light',
  JS = 'js',
  RENDER = 'render',
}

export interface BrowserOptions {
  mode?: BrowserMode;
  timeout?: number;
  userAgent?: string;
}

export interface PageData {
  url: string;
  markdown: string;
  links: string[];
  screenshot?: string;
  jsOutput?: string;
}

export interface NavigateResult {
  url: string;
  success: boolean;
  markdown?: string;
  links?: string[];
  error?: string;
}

export interface SecurityShieldOptions {
  dbPath?: string;
  cacheDays?: number;
}

export interface SecurityCheckResult {
  isSafe: boolean;
  needsApiCheck: boolean;
}

export class BinaryNotFoundError extends Error {
  constructor(message = 'B4n1Web binary not found. Please install it first:\n  curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.6.2-flat.tar.gz | tar -xz') {
    super(message);
    this.name = 'BinaryNotFoundError';
  }
}
