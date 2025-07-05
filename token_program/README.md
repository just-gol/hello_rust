
# 🪙 Rust Token Program 模拟项目

这是一个使用 Rust 编写的模拟 Token 系统，适合作为学习 Rust 与未来编写 Solana 智能合约的基础项目。

---

## ✨ 项目功能

- 👤 创建账户（支持 Admin / User 角色）
- 💰 查询账户余额
- 🏗️ Mint 代币（仅管理员可执行）
- 🔁 用户间转账
- 🔒 启用/禁用账户（仅管理员）
- 🪵 日志打印（需配合 `env_logger` 使用）

---

## 📦 项目结构

```
token_program/
├── src/
│   └── lib.rs       # 账户与交易核心逻辑
├── Cargo.toml       # 项目依赖配置
└── README.md        # 当前文档
```

---

## 🛠️ 使用依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
log = "0.4"
env_logger = "0.11.8"
```

并在 `main.rs` 或测试中初始化：

```rust
env_logger::init();
```

---

## 🚀 快速使用示例

```rust
let mut system = TokenSystem::new();
system.create_account("admin", Role::Admin)?;
system.create_account("alice", Role::User)?;
system.mint("admin", "alice", 100)?;
system.transfer("alice", "admin", 20)?;
```

---

## 🧪 单元测试

运行所有测试：

```bash
cargo test
```

已测试功能：

- 创建账户（ADMIN/USER）
- Mint 成功与失败路径
- 转账流程及错误处理
- 禁用/启用后账户行为限制

---

## 📚 学习价值

本项目帮助你掌握以下知识点：

- Rust 所有权和生命周期（HashMap 管理账户）
- 枚举和结构体建模账户权限与状态
- `Result` 错误处理方式
- 使用 `log` 记录操作行为
- 模块化设计，为未来移植到 Solana BPF 合约打基础

