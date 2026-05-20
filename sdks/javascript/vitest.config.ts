import { defineConfig } from 'vitest/config';
import path from 'path';

export default defineConfig({
  root: '../../internal/tests/javascript',
  resolve: {
    alias: {
      '@sdk': path.resolve(__dirname, '..', 'src'),
    },
  },
  test: {
    globals: false,
  },
});
