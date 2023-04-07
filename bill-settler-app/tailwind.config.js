/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        dark: {
          100: "#333333",
          200: "#2d2d2d",
          300: "#262626",
          400: "#1f1f1f",
          500: "#181818",
          600: "#111111",
          700: "#0a0a0a",
          800: "#030303",
          900: "#000000",
        },
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
