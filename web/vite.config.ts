import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import UnoCSS from 'unocss/vite';

export default defineConfig({
  plugins: [solidPlugin(), UnoCSS()],
  server: {
    proxy: {
      // Forward API requests to stackabled
      '/api': 'http://127.0.0.1:8000',
    },
  },
  css: {
    modules: {
      localsConvention: 'camelCaseOnly',
    },
  },
});
