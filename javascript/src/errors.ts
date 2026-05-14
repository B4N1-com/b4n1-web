export class BinaryNotFoundError extends Error {
  constructor(message = 'B4n1Web binary not found. Please install it first:\n  curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.6.2-flat.tar.gz | tar -xz') {
    super(message);
    this.name = 'BinaryNotFoundError';
    Error.captureStackTrace(this, BinaryNotFoundError);
  }
}

export class NavigationError extends Error {
  constructor(message: string, public readonly url: string) {
    super(message);
    this.name = 'NavigationError';
    Error.captureStackTrace(this, NavigationError);
  }
}

export class BinaryError extends Error {
  constructor(message: string, public readonly stderr: string) {
    super(message);
    this.name = 'BinaryError';
    Error.captureStackTrace(this, BinaryError);
  }
}

export class SelectorTimeoutError extends Error {
  constructor(selector: string, timeoutMs: number) {
    super(`Selector "${selector}" not found within ${timeoutMs}ms`);
    this.name = 'SelectorTimeoutError';
    Error.captureStackTrace(this, SelectorTimeoutError);
  }
}
