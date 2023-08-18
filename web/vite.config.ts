import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import UnoCSS from 'unocss/vite';

import { fileURLToPath, URL } from 'node:url';

export default defineConfig({
  plugins: [solidPlugin(), UnoCSS()],
  server: {
    proxy: {
      // Forward API requests to stackable-cockpitd
      '/api': 'http://127.0.0.1:8000',
    },
  },
  css: {
    modules: {
      localsConvention: 'camelCaseOnly',
    },
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('src', import.meta.url)),
    },
  },
});
