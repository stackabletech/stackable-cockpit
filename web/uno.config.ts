import { defineConfig, presetUno } from 'unocss';

export default defineConfig({
  theme: {
    colors: {
      stbluelight: '#1880BD',
      stbluedark: '#0B689F',
    },
  },
  presets: [presetUno()],
});
