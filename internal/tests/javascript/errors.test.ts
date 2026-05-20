import { describe, it, expect, vi, beforeEach } from 'vitest';
import { BinaryNotFoundError, NavigationError, BinaryError } from '../../../sdks/javascript/src/errors';
import { AgentBrowser } from '../../../sdks/javascript/src/browser';
import * as fsMod from 'fs';

vi.mock('fs', () => ({
  existsSync: vi.fn(),
  accessSync: vi.fn(),
  statSync: vi.fn(),
  constants: { X_OK: 1 },
}));

vi.mock('child_process', () => ({
  execSync: vi.fn(),
}));

describe('Errors', () => {
  const mockedFs = vi.mocked(fsMod);

  beforeEach(() => {
    vi.clearAllMocks();
    process.env.PATH = '/usr/local/bin:/usr/bin';
    process.env.HOME = '/home/testuser';
  });

  describe('BinaryNotFoundError', () => {
    it('should construct with default message', () => {
      const error = new BinaryNotFoundError();
      expect(error).toBeDefined();
      expect(error).toBeInstanceOf(Error);
      expect(error.name).toBe('BinaryNotFoundError');
    });

    it('should construct with custom message', () => {
      const error = new BinaryNotFoundError('Custom error message');
      expect(error.message).toBe('Custom error message');
    });

    it('should have correct message content for default', () => {
      const error = new BinaryNotFoundError();
      expect(error.message).toContain('B4n1Web binary not found');
      expect(error.message).toContain('curl');
      expect(error.message).toContain('https://web.b4n1.com/install');
    });

    it('should be an instanceof Error', () => {
      const error = new BinaryNotFoundError();
      expect(error).toBeInstanceOf(Error);
    });

    it('should be an instanceof BinaryNotFoundError', () => {
      const error = new BinaryNotFoundError();
      expect(error).toBeInstanceOf(BinaryNotFoundError);
    });

    it('should be thrown by AgentBrowser when binary not found', () => {
      mockedFs.existsSync.mockReturnValue(false);
      try {
        new AgentBrowser();
        fail('Expected BinaryNotFoundError to be thrown');
      } catch (e) {
        expect(e.name).toBe('BinaryNotFoundError');
        expect(e.message).toContain('b4n1web binary not found');
      }
    });

    it('should have install info in message when thrown by AgentBrowser', () => {
      mockedFs.existsSync.mockReturnValue(false);
      try {
        new AgentBrowser();
        fail('Expected BinaryNotFoundError to be thrown');
      } catch (e: any) {
        expect(e.name).toBe('BinaryNotFoundError');
        expect(e.message).toContain('curl -sL https://web.b4n1.com/install | bash');
      }
    });

    it('should have a stack trace', () => {
      const error = new BinaryNotFoundError();
      expect(error.stack).toBeDefined();
      expect(error.stack).toContain('BinaryNotFoundError');
    });

    it('should have correct constructor name', () => {
      const error = new BinaryNotFoundError();
      expect(error.constructor.name).toBe('BinaryNotFoundError');
    });

    it('should include all checked paths in default message', () => {
      mockedFs.existsSync.mockReturnValue(false);
      try {
        new AgentBrowser();
      } catch (e: any) {
        expect(e.message).toContain('/usr/local/bin/b4n1web');
        expect(e.message).toContain('/usr/bin/b4n1web');
      }
    });
  });

  describe('NavigationError', () => {
    it('should construct with message and url', () => {
      const error = new NavigationError('Navigation failed', 'https://example.com');
      expect(error).toBeDefined();
      expect(error.message).toBe('Navigation failed');
      expect(error.url).toBe('https://example.com');
    });

    it('should be an instanceof Error', () => {
      const error = new NavigationError('test', 'https://example.com');
      expect(error).toBeInstanceOf(Error);
    });

    it('should be an instanceof NavigationError', () => {
      const error = new NavigationError('test', 'https://example.com');
      expect(error).toBeInstanceOf(NavigationError);
    });

    it('should have correct name property', () => {
      const error = new NavigationError('test', 'https://example.com');
      expect(error.name).toBe('NavigationError');
    });

    it('should have a stack trace', () => {
      const error = new NavigationError('test', 'https://example.com');
      expect(error.stack).toBeDefined();
    });

    it('should preserve url as readonly property', () => {
      const error = new NavigationError('test', 'https://evil.com');
      expect(error.url).toBe('https://evil.com');
    });

    it('should handle empty message', () => {
      const error = new NavigationError('', 'https://example.com');
      expect(error.message).toBe('');
      expect(error.url).toBe('https://example.com');
    });

    it('should handle empty url', () => {
      const error = new NavigationError('Failed', '');
      expect(error.message).toBe('Failed');
      expect(error.url).toBe('');
    });

    it('should handle very long message', () => {
      const longMessage = 'a'.repeat(10000);
      const error = new NavigationError(longMessage, 'https://example.com');
      expect(error.message.length).toBe(10000);
    });
  });

  describe('BinaryError', () => {
    it('should construct with message and stderr', () => {
      const error = new BinaryError('Binary crashed', 'stderr output');
      expect(error).toBeDefined();
      expect(error.message).toBe('Binary crashed');
      expect(error.stderr).toBe('stderr output');
    });

    it('should be an instanceof Error', () => {
      const error = new BinaryError('test', 'stderr');
      expect(error).toBeInstanceOf(Error);
    });

    it('should be an instanceof BinaryError', () => {
      const error = new BinaryError('test', 'stderr');
      expect(error).toBeInstanceOf(BinaryError);
    });

    it('should have correct name property', () => {
      const error = new BinaryError('test', 'stderr');
      expect(error.name).toBe('BinaryError');
    });

    it('should have a stack trace', () => {
      const error = new BinaryError('test', 'stderr');
      expect(error.stack).toBeDefined();
    });

    it('should preserve stderr as readonly property', () => {
      const error = new BinaryError('test', 'error details here');
      expect(error.stderr).toBe('error details here');
    });

    it('should handle empty message', () => {
      const error = new BinaryError('', 'stderr');
      expect(error.message).toBe('');
      expect(error.stderr).toBe('stderr');
    });

    it('should handle empty stderr', () => {
      const error = new BinaryError('Binary failed', '');
      expect(error.message).toBe('Binary failed');
      expect(error.stderr).toBe('');
    });

    it('should handle multiline stderr', () => {
      const stderr = 'Error line 1\nError line 2\nError line 3';
      const error = new BinaryError('Binary failed', stderr);
      expect(error.stderr).toBe('Error line 1\nError line 2\nError line 3');
    });

    it('should handle very long stderr', () => {
      const longStderr = 'x'.repeat(10000);
      const error = new BinaryError('test', longStderr);
      expect(error.stderr.length).toBe(10000);
    });
  });

  describe('all error types exported', () => {
    it('should export BinaryNotFoundError', () => {
      expect(BinaryNotFoundError).toBeDefined();
      expect(typeof BinaryNotFoundError).toBe('function');
    });

    it('should export NavigationError', () => {
      expect(NavigationError).toBeDefined();
      expect(typeof NavigationError).toBe('function');
    });

    it('should export BinaryError', () => {
      expect(BinaryError).toBeDefined();
      expect(typeof BinaryError).toBe('function');
    });

    it('should allow creating instances of all error types', () => {
      const e1 = new BinaryNotFoundError();
      const e2 = new NavigationError('nav error', 'https://example.com');
      const e3 = new BinaryError('binary error', 'stderr');

      expect(e1).toBeInstanceOf(Error);
      expect(e2).toBeInstanceOf(Error);
      expect(e3).toBeInstanceOf(Error);
    });

    it('should have distinct error names', () => {
      const e1 = new BinaryNotFoundError();
      const e2 = new NavigationError('nav error', 'https://example.com');
      const e3 = new BinaryError('binary error', 'stderr');

      expect(e1.name).toBe('BinaryNotFoundError');
      expect(e2.name).toBe('NavigationError');
      expect(e3.name).toBe('BinaryError');
    });

    it('should be distinguishable via instanceof', () => {
      const e1 = new BinaryNotFoundError();
      const e2 = new NavigationError('nav error', 'https://example.com');
      const e3 = new BinaryError('binary error', 'stderr');

      expect(e1).toBeInstanceOf(BinaryNotFoundError);
      expect(e1).not.toBeInstanceOf(NavigationError);
      expect(e1).not.toBeInstanceOf(BinaryError);

      expect(e2).toBeInstanceOf(NavigationError);
      expect(e2).not.toBeInstanceOf(BinaryNotFoundError);
      expect(e2).not.toBeInstanceOf(BinaryError);

      expect(e3).toBeInstanceOf(BinaryError);
      expect(e3).not.toBeInstanceOf(BinaryNotFoundError);
      expect(e3).not.toBeInstanceOf(NavigationError);
    });
  });

  describe('error throwing patterns', () => {
    it('should be throwable with throw keyword', () => {
      expect(() => {
        throw new BinaryNotFoundError();
      }).toThrow(BinaryNotFoundError);

      expect(() => {
        throw new NavigationError('test', 'https://example.com');
      }).toThrow(NavigationError);

      expect(() => {
        throw new BinaryError('test', 'stderr');
      }).toThrow(BinaryError);
    });

    it('should be catchable with try/catch', () => {
      let caught = false;
      try {
        throw new BinaryNotFoundError('test');
      } catch (e: any) {
        caught = true;
        expect(e.message).toBe('test');
      }
      expect(caught).toBe(true);
    });

    it('should preserve message in catch block', () => {
      try {
        throw new NavigationError('nav failed', 'https://test.com');
      } catch (e: any) {
        expect(e.message).toBe('nav failed');
        expect(e.url).toBe('https://test.com');
      }
    });

    it('should preserve BinaryError stderr in catch block', () => {
      try {
        throw new BinaryError('binary failed', 'detailed stderr output');
      } catch (e: any) {
        expect(e.message).toBe('binary failed');
        expect(e.stderr).toBe('detailed stderr output');
      }
    });
  });
});
