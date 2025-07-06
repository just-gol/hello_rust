# 🔐 Rust 锁仓账户管理系统

这是一个使用 Rust 编写的模拟「锁仓 + 权限管理」系统，具备以下功能：

- 用户创建（含权限）
- 管理员铸造代币（Mint）
- 用户锁仓 Token（Lock）
- 到期自动解锁（Unlock）
- 查询锁仓状态
- 日志记录和错误处理
- 完整单元测试

---

## 🧱 模块结构优化建议（已实测项目可运行）

建议将系统拆分为以下模块，避免业务逻辑全部堆叠在一个文件中：

```
src/
├── main.rs              # 项目入口，负责调用逻辑，不写业务
├── user.rs              # 用户结构和权限控制逻辑（含 mint 方法）
├── account_manager.rs   # 锁仓/解锁管理逻辑
├── types.rs             # 枚举类型（Status、Role）等定义
├── lib.rs               # 对外统一暴露模块
tests/
└── integration_test.rs  # 跨模块测试（可选）
```

### ✅ 拆分优势

| 优点 | 描述 |
|------|------|
| 可维护性强 | 每个模块职责单一，逻辑清晰 |
| 可扩展性好 | 后期增加新功能（如 token 交易、授权）更方便 |
| 可测试性强 | 各模块可独立测试，定位问题更快 |
| 更贴近 Solana 合约结构 | Solana 项目本身就是 lib 模块结构 |

---

## 🚀 示例功能说明

### 1. 用户创建

```rust
let mut user_set = HashSet::new();
let admin = User::new("Alice".to_string(), &mut user_set, Role::Admin)?;
```

### 2. Mint（管理员发币）

```rust
admin.mint(100, &mut user)?;
```

### 3. Token 锁仓

```rust
let mut manager = AccountManager::new();
manager.lock(50, &mut user)?;
```

### 4. 解锁

```rust
manager.un_lock(&mut user, Utc::now().timestamp() + 10000)?;
```

### 5. 查询锁仓状态

```rust
let status = manager.query_lock_status("Bob")?;
```

---

## 🧪 单元测试示例

完整测试覆盖包括：用户创建、mint、锁仓、解锁、状态查询、错误路径等。

```rust
#[test]
fn test_lock_and_unlock() {
    let admin = User::new("Admin".to_string(), &mut set, Role::Admin).unwrap();
    let mut bob = User::new("Bob".to_string(), &mut set, Role::User).unwrap();
    admin.mint(100, &mut bob).unwrap();

    let mut manager = AccountManager::new();
    manager.lock(80, &mut bob).unwrap();
    assert_eq!(bob.token, 20);

    let future_time = Utc::now().timestamp() + 10000;
    manager.un_lock(&mut bob, future_time).unwrap();
    assert_eq!(bob.token, 100);
}
```

---

## 📦 依赖说明

```toml
[dependencies]
chrono = "0.4"
log = "0.4"
env_logger = "0.10"
```

---

## 🛠 运行方法

```bash
# 初始化日志并运行测试
RUST_LOG=info cargo test
```

---

## ✅ 推荐：将此作为写 Solana 合约前的实战演练项目之一

- 权限控制 ✅
- 状态管理 ✅
- 生命周期模拟 ✅
- 代币行为抽象 ✅