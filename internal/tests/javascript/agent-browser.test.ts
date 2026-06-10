import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { AgentBrowser, Page } from '../../../sdks/javascript/src/browser';
import { BrowserMode } from '../../../sdks/javascript/src/types';
import { BinaryNotFoundError } from '../../../sdks/javascript/src/errors';
import * as fsMod from 'fs';
import * as childProcess from 'child_process';

vi.mock('fs', () => ({
  existsSync: vi.fn(),
  accessSync: vi.fn(),
  statSync: vi.fn(),
  constants: { X_OK: 1 },
}));

vi.mock('child_process', () => ({
  execSync: vi.fn(),
}));

describe('AgentBrowser', () => {
  const mockedFs = vi.mocked(fsMod);
  const mockedExecSync = vi.mocked(childProcess.execSync);

  beforeEach(() => {
    vi.clearAllMocks();
    mockedFs.existsSync.mockReturnValue(true);
    mockedFs.accessSync.mockReturnValue(undefined);
    mockedFs.statSync.mockReturnValue({
      isFile: () => true,
      mode: 0o755,
    } as fsMod.Stats);

    process.env.PATH = '/usr/local/bin:/usr/bin';
    process.env.HOME = '/home/testuser';
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('construction', () => {
    it('should construct with default options', () => {
      const browser = new AgentBrowser();
      expect(browser).toBeDefined();
    });

    it('should construct with LIGHT mode', () => {
      const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
      expect(browser).toBeDefined();
    });

    it('should construct with JS mode', () => {
      const browser = new AgentBrowser({ mode: BrowserMode.JS });
      expect(browser).toBeDefined();
    });

    it('should construct with RENDER mode', () => {
      const browser = new AgentBrowser({ mode: BrowserMode.RENDER });
      expect(browser).toBeDefined();
    });

    it('should construct with custom timeout', () => {
      const browser = new AgentBrowser({ timeout: 60 });
      expect(browser).toBeDefined();
    });

    it('should construct with zero timeout', () => {
      const browser = new AgentBrowser({ timeout: 0 });
      expect(browser).toBeDefined();
    });

    it('should construct with custom user-agent', () => {
      const browser = new AgentBrowser({ userAgent: 'CustomAgent/2.0' });
      expect(browser).toBeDefined();
    });

    it('should construct with empty user-agent', () => {
      const browser = new AgentBrowser({ userAgent: '' });
      expect(browser).toBeDefined();
    });

    it('should construct with very long user-agent', () => {
      const longUA = 'A'.repeat(10000);
      const browser = new AgentBrowser({ userAgent: longUA });
      expect(browser).toBeDefined();
    });

    it('should construct with unicode user-agent', () => {
      const browser = new AgentBrowser({ userAgent: 'Mozilla/5.0 \u65e5\u672c\u8a9e' });
      expect(browser).toBeDefined();
    });

    it('should throw BinaryNotFoundError when binary is not found', () => {
      mockedFs.existsSync.mockReturnValue(false);
      try {
        new AgentBrowser();
        fail('Expected BinaryNotFoundError to be thrown');
      } catch (e) {
        expect(e.name).toBe('BinaryNotFoundError');
      }
    });

    it('should throw BinaryNotFoundError with install info in message', () => {
      mockedFs.existsSync.mockReturnValue(false);
      expect(() => new AgentBrowser()).toThrow(/curl.*b4n1\.com.*install/);
    });

    it('should throw BinaryNotFoundError when no paths have executable', () => {
      mockedFs.existsSync.mockReturnValue(true);
      mockedFs.accessSync.mockImplementation(() => {
        throw new Error('not executable');
      });
      mockedFs.statSync.mockReturnValue({
        isFile: () => true,
        mode: 0o644,
      } as fsMod.Stats);
      try {
        new AgentBrowser();
        fail('Expected BinaryNotFoundError to be thrown');
      } catch (e) {
        expect(e.name).toBe('BinaryNotFoundError');
      }
    });
  });

  describe('goto', () => {
    it('should call binary with correct arguments in LIGHT mode', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nLinks: []\n'));
      const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
      await browser.goto('https://example.com');
      expect(mockedExecSync).toHaveBeenCalledWith(
        expect.stringContaining('goto https://example.com --mode light'),
        expect.objectContaining({ timeout: 30000 }),
      );
    });

    it('should call binary with correct arguments in JS mode', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nLinks: []\n'));
      const browser = new AgentBrowser({ mode: BrowserMode.JS });
      await browser.goto('https://example.com');
      expect(mockedExecSync).toHaveBeenCalledWith(
        expect.stringContaining('--mode js'),
        expect.any(Object),
      );
    });

    it('should call binary with correct arguments in RENDER mode', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nLinks: []\n'));
      const browser = new AgentBrowser({ mode: BrowserMode.RENDER });
      await browser.goto('https://example.com');
      expect(mockedExecSync).toHaveBeenCalledWith(
        expect.stringContaining('--mode render'),
        expect.any(Object),
      );
    });

    it('should handle URL with special characters', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nLinks: []\n'));
      const browser = new AgentBrowser();
      await browser.goto('https://example.com/path?a=1&b=2#section');
      expect(mockedExecSync).toHaveBeenCalledWith(
        expect.stringContaining('goto https://example.com/path?a=1&b=2#section --mode light'),
        expect.any(Object),
      );
    });

    it('should return Page with no links when output has empty links', async () => {
      mockedExecSync.mockReturnValue(
        Buffer.from('URL: https://example.com\nMarkdown:\nHello World\nLinks: []\n'),
      );
      const browser = new AgentBrowser();
      const page = await browser.goto('https://example.com');
      expect(page).toBeInstanceOf(Page);
      expect(page.url).toBe('https://example.com');
      expect(page.markdown).toBe('Hello World');
      expect(page.links).toEqual([]);
    });

    it('should return Page with many links', async () => {
      const linksArray = Array(50).fill(null).map((_, i) => `https://example.com/${i}`);
      mockedExecSync.mockReturnValue(
        Buffer.from(`Markdown:\nContent\nLinks: ${JSON.stringify(linksArray)}\n`),
      );
      const browser = new AgentBrowser();
      const page = await browser.goto('https://example.com');
      expect(page.links.length).toBe(50);
      expect(page.links[0]).toBe('https://example.com/0');
      expect(page.links[49]).toBe('https://example.com/49');
    });

    it('should return Page with empty markdown', async () => {
      mockedExecSync.mockReturnValue(
        Buffer.from('URL: https://example.com\nMarkdown:\nLinks: []\n'),
      );
      const browser = new AgentBrowser();
      const page = await browser.goto('https://example.com');
      expect(page.markdown).toBe('');
    });

    it('should handle malformed binary output', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('garbage output with no structure'));
      const browser = new AgentBrowser();
      const page = await browser.goto('https://example.com');
      expect(page.markdown).toBe('garbage output with no structure');
      expect(page.links).toEqual([]);
    });

    it('should handle output with Screenshot line', async () => {
      mockedExecSync.mockReturnValue(
        Buffer.from('Markdown:\nContent\nLinks: []\nScreenshot: base64data\n'),
      );
      const browser = new AgentBrowser();
      const page = await browser.goto('https://example.com');
      expect(page.screenshot).toBe('base64data');
    });

    it('should handle output with empty Screenshot line', async () => {
      mockedExecSync.mockReturnValue(
        Buffer.from('Markdown:\nContent\nLinks: []\nScreenshot: \n'),
      );
      const browser = new AgentBrowser();
      const page = await browser.goto('https://example.com');
      expect(page.screenshot).toBeUndefined();
    });

    it('should throw error when binary errors', async () => {
      mockedExecSync.mockImplementation(() => {
        throw new Error('binary crashed');
      });
      const browser = new AgentBrowser();
      await expect(browser.goto('https://example.com')).rejects.toThrow(
        'Binary error: binary crashed',
      );
    });

    it('should throw timeout error when binary times out', async () => {
      const timeoutError = new Error('timed out');
      mockedExecSync.mockImplementation(() => {
        throw timeoutError;
      });
      const browser = new AgentBrowser({ timeout: 1 });
      await expect(browser.goto('https://example.com')).rejects.toThrow(
        'Binary timed out after 1s',
      );
    });

    it('should use correct binary path', async () => {
      mockedFs.existsSync.mockImplementation((p: any) => p === '/usr/local/bin/b4n1web');
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nContent\nLinks: []\n'));
      const browser = new AgentBrowser();
      await browser.goto('https://example.com');
      expect(mockedExecSync).toHaveBeenCalledWith(
        expect.stringContaining('/usr/local/bin/b4n1web'),
        expect.any(Object),
      );
    });

    it('should pass URL as argument', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nContent\nLinks: []\n'));
      const browser = new AgentBrowser();
      await browser.goto('https://specific-url.com');
      expect(mockedExecSync).toHaveBeenCalledWith(
        expect.stringContaining('goto https://specific-url.com'),
        expect.any(Object),
      );
    });

    it('should return a Page instance', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nContent\nLinks: []\n'));
      const browser = new AgentBrowser();
      const page = await browser.goto('https://example.com');
      expect(page).toBeInstanceOf(Page);
    });

    it('should parse screenshot from output', async () => {
      mockedExecSync.mockReturnValue(
        Buffer.from('Markdown:\nContent\nLinks: []\nScreenshot: abc123base64\n'),
      );
      const browser = new AgentBrowser();
      const page = await browser.goto('https://example.com');
      expect(page.screenshot).toBe('abc123base64');
    });

    it('should use custom timeout for execSync', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nContent\nLinks: []\n'));
      const browser = new AgentBrowser({ timeout: 120 });
      await browser.goto('https://example.com');
      expect(mockedExecSync).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({ timeout: 120000 }),
      );
    });
  });

  describe('close', () => {
    it('should not throw when called once', () => {
      const browser = new AgentBrowser();
      expect(() => browser.close()).not.toThrow();
    });

    it('should not throw when called multiple times', () => {
      const browser = new AgentBrowser();
      expect(() => {
        browser.close();
        browser.close();
        browser.close();
      }).not.toThrow();
    });

    it('should be safe to call after goto', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nContent\nLinks: []\n'));
      const browser = new AgentBrowser();
      await browser.goto('https://example.com');
      expect(() => browser.close()).not.toThrow();
    });
  });

  describe('context manager (asyncDispose)', () => {
    it('should enter and exit async context', async () => {
      const browser = new AgentBrowser();
      // Simulate async context by calling close in finally block
      try {
        expect(browser[Symbol.asyncDispose]).toBeDefined();
        await browser[Symbol.asyncDispose]();
      } finally {
        // Ensure cleanup
        if (!browser.sessionStarted) {
          // Already closed
        }
      }
    });

    it('should close on dispose', async () => {
      const browser = new AgentBrowser();
      const closeSpy = vi.spyOn(browser, 'close');
      await browser[Symbol.asyncDispose]();
      expect(closeSpy).toHaveBeenCalled();
    });

    it('should work with using pattern', async () => {
      const browser = new AgentBrowser();
      // Simulate the using pattern by calling close in a finally block
      try {
        expect(browser).toBeDefined();
      } finally {
        browser.close();
      }
    });

    it('should propagate exceptions from context', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nContent\nLinks: []\n'));
      const browser = new AgentBrowser();
      let threw = false;
      try {
        // Simulate the using pattern by calling [Symbol.asyncDispose] in finally block
        try {
          await browser.goto('https://example.com');
          throw new Error('Test error');
        } finally {
          await browser[Symbol.asyncDispose]();
        }
      } catch (e: any) {
        threw = true;
        expect(e.message).toBe('Test error');
      }
      expect(threw).toBe(true);
    });

    it('should close even when exception is thrown', async () => {
      mockedExecSync.mockReturnValue(Buffer.from('Markdown:\nContent\nLinks: []\n'));
      const browser = new AgentBrowser();
      const closeSpy = vi.spyOn(browser, 'close');
      try {
        // Simulate the using pattern by calling [Symbol.asyncDispose] in finally block
        try {
          await browser.goto('https://example.com');
          throw new Error('Test error');
        } finally {
          await browser[Symbol.asyncDispose]();
        }
      } catch {
        // expected
      }
      expect(closeSpy).toHaveBeenCalled();
    });
  });
});
