import { describe, it, expect } from 'vitest';
import { Page } from '../../../sdks/javascript/src/browser';
import type { PageData } from '../../../sdks/javascript/src/types';

describe('Page', () => {
  describe('construction', () => {
    it('should construct with minimal data', () => {
      const data: PageData = { url: 'https://example.com', markdown: '# Hello', links: [] };
      const page = new Page(data);
      expect(page.url).toBe('https://example.com');
      expect(page.markdown).toBe('# Hello');
      expect(page.links).toEqual([]);
      expect(page.screenshot).toBeUndefined();
    });

    it('should construct with full data including screenshot', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Hello\nWorld',
        links: ['https://a.com', 'https://b.com'],
        screenshot: 'base64data',
      };
      const page = new Page(data);
      expect(page.url).toBe('https://example.com');
      expect(page.markdown).toBe('# Hello\nWorld');
      expect(page.links).toEqual(['https://a.com', 'https://b.com']);
      expect(page.screenshot).toBe('base64data');
    });

    it('should construct with unicode content', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# 你好世界\n🎉 Hello World',
        links: ['https://example.com/\u65e5\u672c\u8a9e'],
      };
      const page = new Page(data);
      expect(page.url).toBe('https://example.com');
      expect(page.markdown).toBe('# 你好世界\n🎉 Hello World');
      expect(page.links).toEqual(['https://example.com/\u65e5\u672c\u8a9e']);
    });

    it('should construct with empty markdown', () => {
      const data: PageData = { url: 'https://example.com', markdown: '', links: [] };
      const page = new Page(data);
      expect(page.markdown).toBe('');
    });

    it('should construct with very long markdown', () => {
      const longMarkdown = Array(1000).fill('Line of content').join('\n');
      const data: PageData = { url: 'https://example.com', markdown: longMarkdown, links: [] };
      const page = new Page(data);
      expect(page.markdown).toBe(longMarkdown);
      expect(page.markdown.split('\n').length).toBe(1000);
    });

    it('should construct with many links', () => {
      const links = Array(100).fill(null).map((_, i) => `https://example.com/${i}`);
      const data: PageData = { url: 'https://example.com', markdown: '# Test', links };
      const page = new Page(data);
      expect(page.links.length).toBe(100);
      expect(page.links[0]).toBe('https://example.com/0');
      expect(page.links[99]).toBe('https://example.com/99');
    });

    it('should construct with duplicate links', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://a.com', 'https://a.com', 'https://a.com'],
      };
      const page = new Page(data);
      expect(page.links).toEqual(['https://a.com', 'https://a.com', 'https://a.com']);
    });

    it('should construct with malformed links', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['not-a-url', '://missing-scheme', 'http://', ''],
      };
      const page = new Page(data);
      expect(page.links).toEqual(['not-a-url', '://missing-scheme', 'http://', '']);
    });

    it('should construct with empty link strings', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['', '', ''],
      };
      const page = new Page(data);
      expect(page.links).toEqual(['', '', '']);
    });

    it('should preserve object references for links array', () => {
      const links = ['https://a.com'];
      const data: PageData = { url: 'https://example.com', markdown: '# Test', links };
      const page = new Page(data);
      expect(page.links).toBe(links);
    });
  });

  describe('getMainContent', () => {
    it('should skip header lines when more than 2 lines exist', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Header\n## Subheader\nActual content here',
        links: [],
      };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('Actual content here');
    });

    it('should return all content when only 2 lines exist', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Header\nLine two',
        links: [],
      };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('# Header\nLine two');
    });

    it('should return trimmed content for single line markdown', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '  Single line  ',
        links: [],
      };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('Single line');
    });

    it('should return trimmed content for two lines', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '  Line one  \n  Line two  ',
        links: [],
      };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('Line one  \n  Line two');
    });

    it('should handle empty markdown', () => {
      const data: PageData = { url: 'https://example.com', markdown: '', links: [] };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('');
    });

    it('should handle trailing whitespace', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: 'Header\nSubheader\nContent   \n  ',
        links: [],
      };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('Content');
    });

    it('should preserve internal formatting in content', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: 'H1\nH2\nLine 1\n\nLine 3\n\n\nLine 6',
        links: [],
      };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('Line 1\n\nLine 3\n\n\nLine 6');
    });

    it('should handle many lines correctly', () => {
      const lines = ['H1', 'H2', ...Array(50).fill(null).map((_, i) => `Line ${i}`)];
      const data: PageData = {
        url: 'https://example.com',
        markdown: lines.join('\n'),
        links: [],
      };
      const page = new Page(data);
      const content = page.getMainContent();
      expect(content).toBe(Array(50).fill(null).map((_, i) => `Line ${i}`).join('\n'));
    });

    it('should handle exactly 3 lines by skipping the first 2', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: 'A\nB\nC',
        links: [],
      };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('C');
    });

    it('should handle markdown with only newlines', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '\n\n\n',
        links: [],
      };
      const page = new Page(data);
      expect(page.getMainContent()).toBe('');
    });
  });

  describe('findLinksByText', () => {
    it('should find links containing exact text', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://example.com/docs', 'https://example.com/about'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('docs')).toEqual(['https://example.com/docs']);
    });

    it('should find links with partial text match', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://example.com/documentation', 'https://example.com/about'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('doc')).toEqual(['https://example.com/documentation']);
    });

    it('should return empty array for no match', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://example.com/docs', 'https://example.com/about'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('nonexistent')).toEqual([]);
    });

    it('should be case insensitive', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://example.com/DOCS', 'https://example.com/About'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('docs')).toEqual(['https://example.com/DOCS']);
      expect(page.findLinksByText('ABOUT')).toEqual(['https://example.com/About']);
      expect(page.findLinksByText('DoCs')).toEqual(['https://example.com/DOCS']);
    });

    it('should return multiple matches', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: [
          'https://example.com/docs/api',
          'https://example.com/about',
          'https://example.com/docs/guide',
        ],
      };
      const page = new Page(data);
      expect(page.findLinksByText('docs')).toEqual([
        'https://example.com/docs/api',
        'https://example.com/docs/guide',
      ]);
    });

    it('should return all links for empty search string', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://a.com', 'https://b.com', 'https://c.com'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('')).toEqual(['https://a.com', 'https://b.com', 'https://c.com']);
    });

    it('should handle special characters in search text', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://example.com/a?b=1&c=2', 'https://example.com/about'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('?b=1')).toEqual(['https://example.com/a?b=1&c=2']);
      expect(page.findLinksByText('c=2')).toEqual(['https://example.com/a?b=1&c=2']);
    });

    it('should handle unicode text in search', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://example.com/\u65e5\u672c\u8a9e/\u30c6\u30b9\u30c8', 'https://example.com/about'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('\u65e5\u672c\u8a9e')).toEqual(['https://example.com/\u65e5\u672c\u8a9e/\u30c6\u30b9\u30c8']);
    });

    it('should handle empty link list', () => {
      const data: PageData = { url: 'https://example.com', markdown: '# Test', links: [] };
      const page = new Page(data);
      expect(page.findLinksByText('anything')).toEqual([]);
      expect(page.findLinksByText('')).toEqual([]);
    });

    it('should preserve order of links in result', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://z.com', 'https://a.com/docs', 'https://b.com/docs', 'https://c.com'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('docs')).toEqual([
        'https://a.com/docs',
        'https://b.com/docs',
      ]);
    });

    it('should handle newline in link text', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://example.com/line1\nline2', 'https://example.com/about'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('line1')).toEqual(['https://example.com/line1\nline2']);
      expect(page.findLinksByText('line2')).toEqual(['https://example.com/line1\nline2']);
    });

    it('should handle links with empty strings in the list', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['', 'https://example.com/test', ''],
      };
      const page = new Page(data);
      expect(page.findLinksByText('')).toEqual(['', 'https://example.com/test', '']);
    });

    it('should handle regex-like search text literally', () => {
      const data: PageData = {
        url: 'https://example.com',
        markdown: '# Test',
        links: ['https://example.com/a.b', 'https://example.com/axb'],
      };
      const page = new Page(data);
      expect(page.findLinksByText('.')).toEqual([
        'https://example.com/a.b',
        'https://example.com/axb',
      ]);
    });
  });
});
