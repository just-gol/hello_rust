
# 🗳️ DAO 投票系统（Rust 实现）

这是一个用 Rust 实现的简化 DAO 投票系统，模拟了 DAO 中的基础逻辑，如：创建提案、支持不同角色的加权投票、防止重复投票、提案状态控制、投票记录统计等。

---

## ✨ 功能概览

- 📌 创建提案 / 关闭提案
- 👤 用户角色：管理员（Admin）/ 普通用户（User）
- 🗳️ 支持加权投票（根据角色权重）
- 📊 投票记录统计（赞成、反对、总票数）
- 🔁 防止重复投票
- 🛑 提案关闭后禁止投票
- 🧪 已包含单元测试（`cargo test`）

---

## 📦 核心结构体说明

| 结构体         | 说明                                       |
|----------------|--------------------------------------------|
| `Proposal`     | 提案结构体，包含描述、编号、状态           |
| `Account`      | 账户结构体，包含ID、名称、角色             |
| `Role`         | 用户角色：Admin（管理员）或 User（普通用户）|
| `Statistics`   | 投票统计结构体，记录每个提案的投票详情     |
| `VoteRecord`   | 每个提案的投票记录（赞成、反对、总计）     |
| `VotingResult` | 投票选项枚举：Yes 或 No                   |

---

## 🔨 使用示例

```rust
Proposal::new("提案1".to_string(), 1, Status::Open, &mut proposals);

let mut alice = Account::new(1001, "Alice".to_string(), Role::User(1));
let mut bob = Account::new(1002, "Bob".to_string(), Role::Admin(3));

alice.vote(1, &VotingResult::Yes, &mut statistics, &mut proposals)?;
bob.vote(1, &VotingResult::No, &mut statistics, &mut proposals)?;

let result = alice.query_vote_statistics(&1, &statistics)?;
println!("赞成: {}, 反对: {}, 总票数: {}", result.agree, result.oppose, result.total);
```

---

## ✅ 测试运行

```bash
cargo test
```

项目内已包含基础单元测试，验证提案创建、投票流程、角色权限等逻辑。

---

## 🔮 可拓展方向

- 构建 CLI 命令行投票界面
- 构建 Web 前端或小程序界面
- 集成 Anchor / Solana 链上操作
- 添加提案过期时间
- 导出投票数据为 JSON / CSV
- 日志记录链上审计信息

---

## 🎯 项目目的

本项目是为了打基础学习 Rust 和链上智能合约编写（如 Solana）。它锻炼了所有 Rust 基础能力，如：所有权、借用、枚举、错误处理、测试驱动开发等。

---
