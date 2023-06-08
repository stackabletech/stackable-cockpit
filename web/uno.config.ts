import { defineConfig, presetUno } from 'unocss';

export default defineConfig({
  theme: {
    colors: {
      stblue: '#1880bd',
      'gray-400': '#9CA3AF',
      'gray-600': '#4B5563',
      'gray-700': '#374151',
      'gray-800': '#1F2937',
      'gray-900': '#111827',
      'stblue-light': '#1880BD',
      'stblue-dark': '#0B689F',
    },
  },
  presets: [presetUno()],
});
