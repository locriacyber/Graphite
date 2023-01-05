/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,svelte,ts}"],
  theme: {
    extend: {},
    colors: {
      transparent: "transparent",
      current: "currentColor",

      black: "#000",
      nearblack: "#111",
      mildblack: "#222",
      darkgray: "#333",
      dimgray: "#444",
      dullgray: "#555",
      lowergray: "#666",
      middlegray: "#777",
      uppergray: "#888",
      palegray: "#999",
      softgray: "#aaa",
      lightgray: "#bbb",
      brightgray: "#ccc",
      mildwhite: "#ddd",
      nearwhite: "#eee",
      white: "#fff",

      "data-general": {
        DEFAULT: "#c5c5c5",
        dim: "#767676",
      },
      "data-vector": {
        DEFAULT: "#65bbe5",
        dim: "#4b778c",
      },
      "data-raster": {
        DEFAULT: "#e4bb72",
        dim: "#8b7752",
      },
      "data-mask": "#8d85c7",
      "data-number": {
        DEFAULT: "#d6536e",
        dim: "#803242",
      },
      "data-vec2": {
        DEFAULT: "#cc00ff",
        dim: "#71008d",
      },
      "data-color": {
        DEFAULT: "#70a898",
        dim: "#43645b",
      },
    },
  },
  plugins: [],
}
