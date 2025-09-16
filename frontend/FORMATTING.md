# 代码格式化配置

本项目已配置自动代码格式化功能，使用 Prettier 和 ESLint 来保持代码风格的一致性。

## VS Code 自动格式化

### 安装推荐扩展

项目已配置推荐扩展，打开项目时 VS Code 会提示安装：
- **Prettier - Code formatter** (`esbenp.prettier-vscode`)
- **ESLint** (`dbaeumer.vscode-eslint`)
- **Vue Language Features (Volar)** (`Vue.volar`)

### 保存时自动格式化

项目已配置 `.vscode/settings.json`，启用了以下功能：
- 保存时自动格式化 (`editor.formatOnSave: true`)
- 保存时自动修复 ESLint 错误 (`editor.codeActionsOnSave`)
- 为不同文件类型指定格式化器

## 命令行格式化

### 可用命令

```bash
# 格式化所有文件
npm run format

# 检查格式化（不修改文件）
npm run format:check

# 运行 ESLint 检查
npm run lint
```

### Git 钩子

项目配置了 pre-commit 钩子，在提交前会自动：
1. 检查代码格式 (`npm run format:check`)
2. 运行 ESLint 检查 (`npm run lint`)

如果检查失败，提交会被阻止。

## 配置文件

- `.prettierrc` - Prettier 格式化规则
- `.prettierignore` - 忽略格式化的文件
- `eslint.config.js` - ESLint 规则（已集成 Prettier）
- `.vscode/settings.json` - VS Code 编辑器设置
- `.husky/pre-commit` - Git 提交前钩子

## 格式化规则

主要的 Prettier 配置：
- 不使用分号 (`semi: false`)
- 使用单引号 (`singleQuote: true`)
- 尾随逗号仅在多行时使用 (`trailingComma: 'es5'`)
- 制表符宽度为 2 (`tabWidth: 2`)
- 打印宽度为 80 字符 (`printWidth: 80`)

## 故障排除

### 格式化不生效
1. 确保安装了推荐的 VS Code 扩展
2. 检查 VS Code 设置是否正确加载
3. 重启 VS Code

### 提交被阻止
1. 运行 `npm run format` 修复格式问题
2. 运行 `npm run lint` 检查并修复 ESLint 错误
3. 重新提交