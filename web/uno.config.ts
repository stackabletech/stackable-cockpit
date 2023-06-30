import { defineConfig, presetUno } from 'unocss';

export default defineConfig({
  theme: {
    colors: {
      stackable: {
        blue: {
          light: '#1880BD',
          dark: '#0B689F',
        },
      },
    },
  },
  presets: [presetUno()],
});
