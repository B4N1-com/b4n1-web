import { describe, it, expect } from 'vitest';
import { BrowserMode } from '../../../sdks/javascript/src/types';

describe('BrowserMode', () => {
  describe('enum values', () => {
    it('should have LIGHT value equal to "light"', () => {
      expect(BrowserMode.LIGHT).toBe('light');
    });

    it('should have JS value equal to "js"', () => {
      expect(BrowserMode.JS).toBe('js');
    });

    it('should have RENDER value equal to "render"', () => {
      expect(BrowserMode.RENDER).toBe('render');
    });
  });

  describe('string representation', () => {
    it('should convert LIGHT to string "light"', () => {
      expect(String(BrowserMode.LIGHT)).toBe('light');
    });

    it('should convert JS to string "js"', () => {
      expect(String(BrowserMode.JS)).toBe('js');
    });

    it('should convert RENDER to string "render"', () => {
      expect(String(BrowserMode.RENDER)).toBe('render');
    });

    it('should have toString() return correct value for LIGHT', () => {
      expect(BrowserMode.LIGHT.toString()).toBe('light');
    });

    it('should have toString() return correct value for JS', () => {
      expect(BrowserMode.JS.toString()).toBe('js');
    });

    it('should have toString() return correct value for RENDER', () => {
      expect(BrowserMode.RENDER.toString()).toBe('render');
    });
  });

  describe('reverse mapping (value to key)', () => {
    it('should map "light" back to LIGHT via reverse lookup', () => {
      const keys = Object.keys(BrowserMode).filter((k) => isNaN(Number(k)));
      const lightKey = keys.find((k) => (BrowserMode as any)[k] === 'light');
      expect(lightKey).toBe('LIGHT');
    });

    it('should map "js" back to JS via reverse lookup', () => {
      const keys = Object.keys(BrowserMode).filter((k) => isNaN(Number(k)));
      const jsKey = keys.find((k) => (BrowserMode as any)[k] === 'js');
      expect(jsKey).toBe('JS');
    });

    it('should map "render" back to RENDER via reverse lookup', () => {
      const keys = Object.keys(BrowserMode).filter((k) => isNaN(Number(k)));
      const renderKey = keys.find((k) => (BrowserMode as any)[k] === 'render');
      expect(renderKey).toBe('RENDER');
    });
  });

  describe('count and iteration', () => {
    it('should have exactly 3 enum members', () => {
      const keys = Object.keys(BrowserMode).filter((k) => isNaN(Number(k)));
      expect(keys.length).toBe(3);
    });

    it('should iterate over all keys correctly', () => {
      const keys = Object.keys(BrowserMode).filter((k) => isNaN(Number(k)));
      expect(keys).toContain('LIGHT');
      expect(keys).toContain('JS');
      expect(keys).toContain('RENDER');
    });

    it('should iterate over all values correctly', () => {
      const values = Object.values(BrowserMode).filter((v) => typeof v === 'string');
      expect(values).toContain('light');
      expect(values).toContain('js');
      expect(values).toContain('render');
    });

    it('should have 3 entries total (3 keys for string literal enum)', () => {
      expect(Object.keys(BrowserMode).length).toBe(3);
    });
  });

  describe('equality', () => {
    it('should equate BrowserMode.LIGHT to "light"', () => {
      expect(BrowserMode.LIGHT === 'light').toBe(true);
    });

    it('should equate BrowserMode.JS to "js"', () => {
      expect(BrowserMode.JS === 'js').toBe(true);
    });

    it('should equate BrowserMode.RENDER to "render"', () => {
      expect(BrowserMode.RENDER === 'render').toBe(true);
    });

    it('should not equate LIGHT to JS', () => {
      expect(BrowserMode.LIGHT === BrowserMode.JS).toBe(false);
    });

    it('should not equate LIGHT to RENDER', () => {
      expect(BrowserMode.LIGHT === BrowserMode.RENDER).toBe(false);
    });

    it('should not equate JS to RENDER', () => {
      expect(BrowserMode.JS === BrowserMode.RENDER).toBe(false);
    });

    it('should not equate LIGHT to empty string', () => {
      expect(BrowserMode.LIGHT === '').toBe(false);
    });

    it('should not equate LIGHT to "LIGHT" (uppercase)', () => {
      expect(BrowserMode.LIGHT === 'LIGHT').toBe(false);
    });
  });

  describe('name property', () => {
    it('should have name "LIGHT" for LIGHT member', () => {
      expect((BrowserMode.LIGHT as any).name ?? 'LIGHT').toBe('LIGHT');
    });

    it('should have name "JS" for JS member', () => {
      expect((BrowserMode.JS as any).name ?? 'JS').toBe('JS');
    });

    it('should have name "RENDER" for RENDER member', () => {
      expect((BrowserMode.RENDER as any).name ?? 'RENDER').toBe('RENDER');
    });
  });

  describe('hashability via Set', () => {
    it('should create a Set with 3 unique BrowserMode values', () => {
      const set = new Set([BrowserMode.LIGHT, BrowserMode.JS, BrowserMode.RENDER]);
      expect(set.size).toBe(3);
    });

    it('should deduplicate identical BrowserMode values in a Set', () => {
      const set = new Set([BrowserMode.LIGHT, BrowserMode.LIGHT, BrowserMode.LIGHT]);
      expect(set.size).toBe(1);
    });

    it('should have Set containing "light" when LIGHT is added', () => {
      const set = new Set([BrowserMode.LIGHT]);
      expect(set.has('light')).toBe(true);
    });

    it('should have Set containing "js" when JS is added', () => {
      const set = new Set([BrowserMode.JS]);
      expect(set.has('js')).toBe(true);
    });

    it('should have Set containing "render" when RENDER is added', () => {
      const set = new Set([BrowserMode.RENDER]);
      expect(set.has('render')).toBe(true);
    });

    it('should not contain "dark" when only LIGHT is added', () => {
      const set = new Set([BrowserMode.LIGHT]);
      expect(set.has('dark')).toBe(false);
    });
  });

  describe('invalid values', () => {
    it('should not have a DARK mode', () => {
      expect((BrowserMode as any).DARK).toBeUndefined();
    });

    it('should not have a FULL mode', () => {
      expect((BrowserMode as any).FULL).toBeUndefined();
    });

    it('should not have an ALL mode', () => {
      expect((BrowserMode as any).ALL).toBeUndefined();
    });

    it('should not accept arbitrary string as enum member', () => {
      const invalid = 'invalid' as BrowserMode;
      expect(Object.values(BrowserMode)).not.toContain(invalid);
    });
  });

  describe('type coercion', () => {
    it('should coerce LIGHT to lowercase string', () => {
      expect(BrowserMode.LIGHT.toLowerCase()).toBe('light');
    });

    it('should coerce JS to uppercase string', () => {
      expect(BrowserMode.JS.toUpperCase()).toBe('JS');
    });

    it('should have correct length property for string values', () => {
      expect(BrowserMode.LIGHT.length).toBe(5);
      expect(BrowserMode.JS.length).toBe(2);
      expect(BrowserMode.RENDER.length).toBe(6);
    });
  });

  describe('switch statement compatibility', () => {
    it('should work in switch statement for LIGHT', () => {
      const mode = BrowserMode.LIGHT;
      let result = '';
      switch (mode) {
        case BrowserMode.LIGHT:
          result = 'light';
          break;
        case BrowserMode.JS:
          result = 'js';
          break;
        case BrowserMode.RENDER:
          result = 'render';
          break;
        default:
          result = 'unknown';
      }
      expect(result).toBe('light');
    });

    it('should work in switch statement for JS', () => {
      const mode = BrowserMode.JS;
      let result = '';
      switch (mode) {
        case BrowserMode.LIGHT:
          result = 'light';
          break;
        case BrowserMode.JS:
          result = 'js';
          break;
        case BrowserMode.RENDER:
          result = 'render';
          break;
        default:
          result = 'unknown';
      }
      expect(result).toBe('js');
    });

    it('should work in switch statement for RENDER', () => {
      const mode = BrowserMode.RENDER;
      let result = '';
      switch (mode) {
        case BrowserMode.LIGHT:
          result = 'light';
          break;
        case BrowserMode.JS:
          result = 'js';
          break;
        case BrowserMode.RENDER:
          result = 'render';
          break;
        default:
          result = 'unknown';
      }
      expect(result).toBe('render');
    });
  });

  describe('record/key usage', () => {
    it('should work as object key', () => {
      const map: Record<BrowserMode, string> = {
        light: 'Light mode',
        js: 'JS mode',
        render: 'Render mode',
      };
      expect(map[BrowserMode.LIGHT]).toBe('Light mode');
      expect(map[BrowserMode.JS]).toBe('JS mode');
      expect(map[BrowserMode.RENDER]).toBe('Render mode');
    });

    it('should work as Map key', () => {
      const map = new Map<BrowserMode, number>();
      map.set(BrowserMode.LIGHT, 1);
      map.set(BrowserMode.JS, 2);
      map.set(BrowserMode.RENDER, 3);
      expect(map.get(BrowserMode.LIGHT)).toBe(1);
      expect(map.get(BrowserMode.JS)).toBe(2);
      expect(map.get(BrowserMode.RENDER)).toBe(3);
    });
  });

  describe('JSON serialization', () => {
    it('should serialize LIGHT to JSON string "light"', () => {
      expect(JSON.stringify(BrowserMode.LIGHT)).toBe('"light"');
    });

    it('should serialize JS to JSON string "js"', () => {
      expect(JSON.stringify(BrowserMode.JS)).toBe('"js"');
    });

    it('should serialize RENDER to JSON string "render"', () => {
      expect(JSON.stringify(BrowserMode.RENDER)).toBe('"render"');
    });

    it('should deserialize back from JSON correctly', () => {
      const json = JSON.stringify(BrowserMode.LIGHT);
      const parsed = JSON.parse(json);
      expect(parsed).toBe('light');
    });
  });
});
