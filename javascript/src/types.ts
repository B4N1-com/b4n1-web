/**
 * B4n1Web TypeScript Types
 */

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
  constructor(message = 'B4n1Web binary not found. Please install it first:\n  curl -sL https://web.b4n1.com/install | bash') {
    super(message);
    this.name = 'BinaryNotFoundError';
  }
}
