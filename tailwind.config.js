/** @type {import("tailwindcss/resolveConfig")} */
const config = {
  darkMode: 'class',
  content: [
    'node_modules/daisyui/dist/**/*.js',
    'node_modules/react-daisyui/dist/**/*.js',
    './src/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
    require('@tailwindcss/aspect-ratio'),
    require('@tailwindcss/container-queries'),
    require('daisyui'),
    require('tailwind-scrollbar-hide'),
  ],
  theme: {},
  daisyui: {
    themes: ['luxury'],
  },
};

module.exports = config;
