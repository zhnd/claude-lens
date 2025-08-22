# Claude Lens 构建指南

## 自动化前端构建

Claude Lens 使用 `build.rs` 脚本实现自动化前端构建集成。

### 功能特性

1. **环境检查**：自动检测 Node.js (>= 18) 和 npm 是否安装
2. **智能构建**：只在前端文件变更时重新构建
3. **缓存优化**：避免重复构建，提升开发效率
4. **错误处理**：前端构建失败不会阻止 Rust 编译
5. **开发友好**：支持跳过前端构建的环境变量

### 构建命令

```bash
# 正常构建（包含前端）
cargo build

# 跳过前端构建
SKIP_WEB_BUILD=1 cargo build

# 强制重新构建前端
rm web/.build_cache && cargo build
```

### 构建流程

1. 检查 Node.js 和 npm 环境
2. 验证 `web/package.json` 存在
3. 检查是否需要重新构建（基于文件时间戳）
4. 运行 `npm install`（如果需要）
5. 运行 `npm run build`
6. 验证构建产物
7. 更新构建缓存

### 触发重新构建的条件

- `web/src/` 目录中的文件变更
- `web/public/` 目录中的文件变更
- `web/package.json` 变更
- `web/package-lock.json` 变更
- `web/vite.config.js` 变更
- `web/tsconfig.json` 变更
- `web/index.html` 变更
- 构建缓存文件不存在

### 环境要求

- Node.js >= 18
- npm（通常随 Node.js 一起安装）

### 故障排除

如果遇到前端构建问题：

1. 手动构建前端验证：
   ```bash
   cd web
   npm install
   npm run build
   ```

2. 清理缓存重新构建：
   ```bash
   rm web/.build_cache
   rm -rf web/node_modules
   cargo build
   ```

3. 跳过前端构建：
   ```bash
   SKIP_WEB_BUILD=1 cargo build
   ```

### 集成到 CI/CD

在 CI/CD 环境中，建议：

```yaml
# GitHub Actions 示例
- name: Setup Node.js
  uses: actions/setup-node@v3
  with:
    node-version: '18'

- name: Build Rust project with frontend
  run: cargo build --release
```

### 开发模式

开发时可以分别启动前端和后端：

```bash
# 终端 1：启动后端
SKIP_WEB_BUILD=1 cargo run

# 终端 2：启动前端开发服务器
cd web
npm run dev
```

前端开发服务器会代理 API 请求到后端服务器。