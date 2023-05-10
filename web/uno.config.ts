import { defineConfig, presetUno } from 'unocss';

export default defineConfig({
  theme: {
    colors: {
      stblue: '#1880bd',
    },
    fontFamily: {
      sans: 'Inter',
    },
  },
  presets: [presetUno()],
});
