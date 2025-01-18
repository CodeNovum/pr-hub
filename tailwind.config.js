import daisyui from "daisyui";

export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  safelist: [
    { pattern: /gap-/ },
    { pattern: /p-/ },
    { pattern: /max-w-/ },
    { pattern: /max-h-/ },
    { pattern: /w-/ },
    { pattern: /h-/ },
  ],
  darkMode: "media",
  theme: {
    extend: {},
  },
  plugins: [daisyui],
  daisyui: {
    themes: ["light", "dark"],
  },
};
