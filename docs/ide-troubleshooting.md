# IDE 检查优化指南

## 已添加的配置优化

### 1. **缓存和性能优化**
- ✅ 启用增量编译（`cargo.buildScripts.enable`）
- ✅ 启用宏展开属性（提高 proc-macro 准确性）
- ✅ 独立 target 目录（避免冲突）

### 2. **诊断设置**
- ✅ 使用 clippy（更严格的检查）
- ✅ 保存时自动检查
- ✅ 所有 targets 和 features

## 日常使用建议

### 1. 遇到 "proc-macro panicked" 错误时

#### 快速修复
```bash
# 方法1：清理 rust-analyzer 缓存
rm -rf target/rust-analyzer

# 方法2：完整清理
cargo clean
cargo check
```

#### 在 VS Code 中
1. 按 `Cmd+Shift+P`（Mac）或 `Ctrl+Shift+P`（Windows/Linux）
2. 运行：`rust-analyzer: Restart server`
3. 或：`Developer: Reload Window`

### 2. 日常维护

#### 每周清理一次
```bash
# 清理所有编译产物和缓存
cargo clean
# 重新构建
cargo build
```

#### 遇到奇怪错误时
```bash
# 1. 清理缓存
rm -rf target/

# 2. 更新依赖
cargo update

# 3. 重新检查
cargo check
```

### 3. 提升检查速度

#### 配置多个检查级别
在 `Cargo.toml` 中添加：
```toml
[profile.dev]
debug = true
opt-level = 0  # 不优化，加快编译
```

#### 使用 `.rustfmt.toml` 统一格式
```toml
edition = "2021"
max_width = 100
```

### 4. 诊断问题

#### 查看 rust-analyzer 日志
1. 打开命令面板：`Cmd+Shift+P`
2. 运行：`rust-analyzer: Show RA syntax tree`
3. 查看 RA 状态：`rust-analyzer: View rust-analyzer status`

#### 如果经常出现 proc-macro 错误
```toml
# 在 Cargo.toml 中显式禁用有问题的 proc-macro
[dependencies]
async-trait = { version = "0.1", default-features = false }
```

## 最佳实践

### 1. 依赖管理
```bash
# 定期更新依赖
cargo update

# 检查过时依赖
cargo outdated  # 需要安装: cargo install cargo-outdated
```

### 2. 代码组织
- ✅ 模块化：每个文件职责单一
- ✅ 导入规范：使用 `mod` 组织代码
- ✅ 类型明确：避免过度使用泛型推断

### 3. 错误处理
```rust
// 推荐：明确的错误类型
pub enum MyError {
    Database(diesel::result::Error),
    Business(String),
}

// 避免：过度使用 Box<dyn Error>
```

### 4. 保持编译通过
- ✅ 每次修改后运行 `cargo check`
- ✅ 提交前运行 `cargo clippy -- -D warnings`
- ✅ 定期运行 `cargo fmt`

## 常用命令速查

```bash
# 快速检查（不生成可执行文件）
cargo check

# 完整检查（生成可执行文件）
cargo build

# 代码格式检查
cargo fmt -- --check

# 代码风格检查
cargo clippy

# 运行测试
cargo test

# 清理缓存
cargo clean
```

## 遇到问题时

### 常见错误及解决方案

1. **"Cannot create expander for X.dylib"**
   ```bash
   rm -rf target/rust-analyzer
   cargo check
   ```

2. **"Type annotations needed"**
   - 显式添加类型：`ApiResponse::<()>`
   - 或添加类型标注

3. **"Undefined proc macro"**
   ```bash
   cargo clean
   cargo build
   ```

4. **IDE 显示错误但 cargo check 通过**
   - 重启 rust-analyzer 服务器
   - 检查工作区路径是否正确

### 查看详细信息
```bash
# 详细信息输出
RUST_LOG=debug cargo check

# 查看依赖树
cargo tree

# 查看特性
cargo tree --features all
```

## VS Code 快捷操作

- `Cmd+Shift+P` → `rust-analyzer: Restart server` - 重启服务器
- `Cmd+Shift+P` → `rust-analyzer: Show Rust Analyzer status` - 查看状态
- `Cmd+Shift+P` → `Developer: Reload Window` - 重载窗口
- `Ctrl+Shift+P` → `rust-analyzer: Stop server` - 停止服务器（如果有问题）

## 参考链接

- [rust-analyzer 官方文档](https://rust-analyzer.github.io/)
- [Rust 官方书](https://doc.rust-lang.org/book/)
- [Clippy 规则](https://rust-lang.github.io/rust-clippy/)

