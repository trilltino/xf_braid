const path = require('path');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    path.join(__dirname, "src/**/*.rs"),
    path.join(__dirname, "**/*.html"),
  ],
  theme: {
    extend: {
      colors: {
        // GTK-inspired dark theme
        'gtk-bg': '#151515',
        'gtk-hover': '#3b3b3b',
        'gtk-card': '#1f1f1f',
        'gtk-border': '#2a2a2a',
        'gtk-accent': '#3b86e2',
        'gtk-accent-hover': '#2a75d1',
        'gtk-accent-orange': '#FF7800',
      },
      fontFamily: {
        'display': ['"Red Hat Display"', '-apple-system', 'BlinkMacSystemFont', '"Segoe UI"', 'Roboto', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
