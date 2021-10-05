const colors = require('./node_modules/tailwindcss/colors');

module.exports = {　
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      spacing: {
         '120': '30rem',
         '100': '25rem',
         '264': '66rem',
      },
      colors: {
        teal: colors.teal,
        lime: colors.lime,
      } 
    }
  },
  variants: {
    backgroundColor: ['responsive', 'hover', 'focus', 'active'],
  },
  plugins: [],
}
