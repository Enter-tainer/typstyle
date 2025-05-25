/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        mono: ["Monaco", "Menlo", "Ubuntu Mono", "monospace"],
      },
      colors: {
        // Komeiji Koishi themed colors
        koishi: {
          green: {
            50: "#F1F8E9",
            100: "#DCEDC8",
            200: "#C5E1A5",
            300: "#AED581",
            400: "#9CCC65",
            500: "#8BC34A", // Main green
            600: "#7CB342",
            700: "#689F38",
            800: "#558B2F",
            900: "#33691E",
          },
          purple: {
            50: "#F3E5F5",
            100: "#E1BEE7",
            200: "#CE93D8",
            300: "#BA68C8",
            400: "#AB47BC",
            500: "#9C27B0", // Main purple
            600: "#8E24AA",
            700: "#7B1FA2",
            800: "#6A1B9A",
            900: "#4A148C",
          },
          pink: {
            50: "#FCE4EC",
            100: "#F8BBD9",
            200: "#F48FB1",
            300: "#F06292",
            400: "#EC407A",
            500: "#E91E63", // Main pink
            600: "#D81B60",
            700: "#C2185B",
            800: "#AD1457",
            900: "#880E4F",
          },
          gold: {
            50: "#FFF8E1",
            100: "#FFECB3",
            200: "#FFE082",
            300: "#FFD54F",
            400: "#FFCA28",
            500: "#FFC107", // Main gold
            600: "#FFB300",
            700: "#FFA000",
            800: "#FF8F00",
            900: "#FF6F00",
          },
        },
      },
    },
  },
  plugins: [],
};
