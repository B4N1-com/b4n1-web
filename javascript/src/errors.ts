/**
 * B4n1Web Errors
 */

/**
 * Error thrown when B4n1Web binary is not found
 */
export class BinaryNotFoundError extends Error {
  constructor(message = 'B4n1Web binary not found. Please install it first:\n  curl -sL https://web.b4n1.com/install | bash') {
    super(message);
    this.name = 'BinaryNotFoundError';
    Error.captureStackTrace(this, BinaryNotFoundError);
  }
}

/**
 * Error thrown when navigation fails
 */
export class NavigationError extends Error {
  constructor(message: string, public readonly url: string) {
    super(message);
    this.name = 'NavigationError';
    Error.captureStackTrace(this, NavigationError);
  }
}

/**
 * Error thrown when binary execution fails
 */
export class BinaryError extends Error {
  constructor(message: string, public readonly stderr: string) {
    super(message);
    this.name = 'BinaryError';
    Error.captureStackTrace(this, BinaryError);
  }
}
