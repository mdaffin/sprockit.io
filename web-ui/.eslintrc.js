module.exports = {
  root: true,
  env: {
    browser: true,
    node: true
  },
  parserOptions: {
    parser: "babel-eslint"
  },
  extends: [
    "eslint:recommended",
    "plugin:vue/strongly-recommended",
    "plugin:prettier/recommended",
    "plugin:jest/recommended",
    "plugin:jest/style"
  ],
  plugins: ["vue", "jest"],
  rules: {
    "vue/max-attributes-per-line": "off"
  }
};
