/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.{html,rs}"],
  },
  darkMode: "media", // 'media' or 'class'
  theme: {
    extend: {
      fontFamily: {
        cursive: ["GreyQo", "sans-serif"],
        // Add more custom font families as needed
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
