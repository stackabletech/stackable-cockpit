import { defineConfig, presetUno } from 'unocss';

export default defineConfig({
  theme: {
    colors: {
      'gray-400': '#9CA3AF',
      'gray-600': '#4B5563',
      'gray-700': '#374151',
      'gray-800': '#1F2937',
      'gray-900': '#111827',
      stbluelight: '#1880BD',
      stbluedark: '#0B689F',
    },
  },
  presets: [presetUno()],
});
