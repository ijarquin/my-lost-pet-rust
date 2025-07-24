/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html"
  ],
  theme: {
    screens: {
      sm: '480px',
      md: '768px',
      lg: '1024px',
      xl: '1440px',
    },
    extend: {
      colors: {
        softBlue: 'hsl(231, 69%, 60%)',
        softRed: 'hsl(0, 94%, 66%)',
        grayishBlue: 'hsl(229, 8%, 60%)',
        lightGrayishBlue: 'hsl(210, 4%, 67%)',
        veryDarkBlue: 'hsl(229, 31%, 21%)',
      },
      fontFamily: {
        sans: [
          'Rubik' , 'sans-serif'
        ]
      },
      letterSpacing: {
        widest: '.3em'
      }
    },
  },
  plugins: [],
};
