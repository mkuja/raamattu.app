/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
      "./src/**/*.rs",
      "./index.html",
      "./src/**/*.html",
      "./src/**/*.css",
  ],
  darkMode: "media", // 'media' or 'class'
  theme: {
    colors: {
      hilight: "#e4a0c8",
      inactive: "#c8d2e4",
      primary: "#7be0b4",
      secondary: "#a4d9e5",
      rim: "#4fd1e8"
    },
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
