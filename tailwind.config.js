/** @type {import('tailwindcss').Config} */

const plugin = require("tailwindcss/plugin");
const colors = require("tailwindcss/colors");
const defaultTheme = require("tailwindcss/defaultTheme");
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  darkMode: "class",
  theme: {
    extend: {
      boxShadow: {
        gitter:
          "rgba(136, 172, 243, 0.25) 0px 10px 30px, rgba(0, 0, 0, 0.03) 0px 1px 1px, rgba(0, 51, 167, 0.1) 0px 10px 20px;",
      },
      fontFamily: {
        mono: ['"IBM Plex Mono"', "ui-monospace"], // Ensure fonts with spaces have " " surrounding it.
        sans: [
          '"IBM Plex Sans"',
          "system-ui",
          "-apple-system",
          "BlinkMacSystemFont",
          '"Segoe UI"',
          "Roboto",
          '"Helvetica Neue"',
          "Arial",
          '"Noto Sans"',
          "sans-serif",
          '"Apple Color Emoji"',
          '"Segoe UI Emoji"',
          '"Segoe UI Symbol"',
        ], // Ensure fonts with spaces have " " surrounding it.
      },
      fontSize: {
        xs: "0.75rem",
        sm: "0.875rem",
        base: "1rem",
        lg: "1.125rem",
        xl: "1.25rem",
        "2xl": "1.5rem",
        "3xl": "1.875rem",
        "4xl": "2.25rem",
        "5xl": "3rem",
        "6xl": "4rem",
        "7xl": "5rem",
        "8xl": "6rem",
        "9xl": "7rem",
        "10xl": "8rem",
        "11xl": "9rem",
        "12xl": "10rem",
        "13xl": "11rem",
        "14xl": "12rem",
        "15xl": "13rem",
        "16xl": "14rem",
        "17xl": "15rem",
        "18xl": "16rem",
        "19xl": "17rem",
        "20xl": "18rem",
      },
      colors: {
        primary: {
          light: "#ac92ec",
          dark: "#967adc",
        },

        secondary: {
          light: "#4fc1e9",
          dark: "#3bafda",
        },

        success: {
          light: "#48cfad",
          dark: "#37bc9b",
        },

        info: {
          light: "#a0d468",
          dark: "#8cc152",
        },

        warning: {
          light: "#ffce54",
          dark: "#fcbb42",
        },

        dengeros: {
          light: "#ed5565",
          dark: "#da4453",
        },

        dark: {
          light: "#656d78",
          dark: "#434a54",
        },

        light: {
          light: "#f5f7fa",
          dark: "#e6e9ed",
        },
      },
      typography: {
        DEFAULT: {
          css: {
            code: {
              color: "#7c74da",
            },
            pre: {
              backgroundColor: "#1c2128",
              borderWidth: "1px",
              borderColor: "#2d333b",
            },
          },
        },
      },
      fontFamily: {
        mono: ['"IBM Plex Mono"', ...defaultTheme.fontFamily.mono], // Ensure fonts with spaces have " " surrounding it.
        sans: ['"IBM Plex Sans"', ...defaultTheme.fontFamily.sans], // Ensure fonts with spaces have " " surrounding it.
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/forms"),
    require("@tailwindcss/typography"),
    require("@tailwindcss/line-clamp"),
    require("@tailwindcss/aspect-ratio"),
  ],
};
