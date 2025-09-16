# Airbnb ESLint 规则集成

本项目已集成 Airbnb 风格的 ESLint 规则，但针对现有项目进行了适配调整。

## 已集成的核心 Airbnb 规则

### 变量和声明
- `no-var`: 禁止使用 `var`，强制使用 `let` 或 `const`
- `prefer-const`: 推荐使用 `const` 声明不会重新赋值的变量
- `no-duplicate-imports`: 禁止重复导入同一模块

### 代码质量
- `eqeqeq`: 强制使用严格相等 (`===` 和 `!==`)
- `curly`: 强制所有控制语句使用大括号
- `prefer-arrow-callback`: 推荐使用箭头函数作为回调

### 调试和开发
- `no-debugger`: 警告使用 `debugger` 语句
- `no-console`: 开发阶段允许（已关闭）

## 项目适配调整

由于项目使用 Vue 3 + TypeScript + Vite，对标准 Airbnb 规则进行了以下调整：

### TypeScript 相关
- `@typescript-eslint/no-unused-vars`: 改为警告级别
- `@typescript-eslint/no-explicit-any`: 开发阶段允许使用 `any`
- `@typescript-eslint/no-empty-object-type`: 允许空对象类型

### Vue 相关
- `vue/multi-word-component-names`: 关闭（允许单词组件名）
- `vue/no-unused-vars`: 改为警告级别

### 导入相关
- `import/extensions`: 关闭（与 Vite 别名冲突）
- `import/no-unresolved`: 关闭（Vue 文件导入解析问题）
- `import/order`: 暂时关闭（避免大量重构）

## 当前状态

项目目前有一些 ESLint 警告和错误，主要包括：
- 未使用的变量和导入
- 一些遗留的代码质量问题

这些问题可以逐步修复，不会影响项目的正常运行。

## 使用建议

1. **新代码**：严格遵循 Airbnb 规范
2. **现有代码**：逐步重构，优先修复错误级别的问题
3. **团队协作**：在 PR 中关注新增的 ESLint 问题

## 进一步优化

如需更严格的 Airbnb 规范，可以：
1. 逐步修复现有的 ESLint 问题
2. 启用更多格式化规则
3. 集成 `eslint-config-airbnb-typescript` 获得完整支持

## 命令

```bash
# 检查代码规范
npm run lint

# 自动修复可修复的问题
npm run lint -- --fix

# 格式化代码
npm run format
```