import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    coverage: {
      enabled: true,
      provider: 'c8',
      reporter: ['html', 'text'],
    },
  },
});
