import js from '@eslint/js';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import pluginVue from 'eslint-plugin-vue';
import pluginImport from 'eslint-plugin-import';

import { defineConfig } from 'eslint/config';

export default defineConfig([
  {
    ignores: [
      'node_modules/**',
      'dist/**',
      'build/**',
      '*.min.js',
      'coverage/**',
      '.nuxt/**',
      '.output/**',
      '.vite/**'
    ]
  },
  {
    files: ['**/*.{js,mjs,cjs,ts,mts,cts,vue}'],
    plugins: {
      js,
      import: pluginImport
    },
    extends: ['js/recommended'],
    languageOptions: {
      globals: {
        ...globals.browser,
        getApp: 'readonly' // 全局函数
      }
    },
    rules: {
      // Airbnb 风格核心规则（适配现有项目）
      'no-var': 'error',
      'prefer-const': 'warn',
      'no-unused-vars': 'off', // 由 TypeScript 处理
      'no-console': 'off', // 开发阶段允许 console
      'no-debugger': 'warn',
      'eqeqeq': ['warn', 'always'],
      'curly': ['warn', 'all'],
      'no-duplicate-imports': 'warn',
      'prefer-arrow-callback': 'warn',
      'import/order': 'off', // 暂时关闭导入顺序检查
      'import/no-unresolved': 'off',
      'import/extensions': 'off',
      // 格式化规则
      'semi': ['error', 'always'], // 要求分号
      'object-curly-spacing': ['error', 'always'], // 大括号内需要空格
      'array-bracket-spacing': ['error', 'never'], // 数组括号内不要空格
      'block-spacing': ['error', 'always'], // 块级作用域大括号内需要空格
      'space-before-blocks': ['error', 'always'] // 块级作用域前需要空格
    }
  },
  ...tseslint.configs.recommended,
  ...pluginVue.configs['flat/essential'],
  {
    files: ['**/*.ts', '**/*.tsx'],
    rules: {
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/explicit-function-return-type': 'off',
      '@typescript-eslint/no-explicit-any': 'off', // 开发阶段允许 any
      '@typescript-eslint/no-empty-object-type': 'off'
    }
  },
  {
    files: ['**/*.vue'],
    languageOptions: {
      parserOptions: {
        parser: tseslint.parser
      }
    },
    rules: {
      'vue/multi-word-component-names': 'off',
      'vue/no-unused-vars': 'warn',
      '@typescript-eslint/no-unused-vars': 'warn',
      '@typescript-eslint/no-explicit-any': 'off'
    }
  },
]);
