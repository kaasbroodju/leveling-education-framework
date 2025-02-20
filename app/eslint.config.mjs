import unusedImports from "eslint-plugin-unused-imports";
import prettier from "eslint-plugin-prettier";
import path from "node:path";
import { fileURLToPath } from "node:url";
import js from "@eslint/js";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const compat = new FlatCompat({
  baseDirectory: __dirname,
  recommendedConfig: js.configs.recommended,
  allConfig: js.configs.all,
});

export default [
  ...compat.extends(
    "next/core-web-vitals",
    "next/typescript",
    "plugin:prettier/recommended",
  ),
  {
    plugins: {
      "unused-imports": unusedImports,
      prettier,
    },

    rules: {
      "unused-imports/no-unused-imports": "error", // Ensure this is enabled for error detection
      "unused-imports/no-unused-vars": [
        "warn",
        {
          vars: "all",
          args: "none",
          ignoreRestSiblings: false,
        },
      ],
      "prettier/prettier": "error",
    },
    ignores: ["node_modules/*", ".next/*"],
  },
];
