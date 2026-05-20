import { defineConfig } from 'vitest/config';
import path from 'path';

export default defineConfig({
  resolve: {
    alias: {
      '@sdk': path.resolve(__dirname, '../../sdks/javascript/src'),
    },
  },
  test: {
    globals: false,
  },
});
