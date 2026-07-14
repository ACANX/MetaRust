# MetaRust

一个提供常用辅助工具和函数的 Rust 工具库。

## 特性

- **Rust 2021 版本** — 现代 Rust 编程，严格的类型安全
- **零依赖** — 轻量、快速
- **充分测试** — 全面的测试覆盖

## 使用方式

添加到你的 `Cargo.toml`：

```toml
[dependencies]
metarust = { git = "https://github.com/ACANX/MetaRust" }
```

```rust
use metarust::greet;

let message = greet("世界");
println!("{}", message);
```

## 开发指南

```bash
# 构建
cargo build

# 运行测试
cargo test

# 运行示例
cargo run --example basic

# 发布构建
cargo build --release
```

## 许可证

Apache License 2.0
