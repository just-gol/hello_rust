
# 🧾 Rust 模拟账户系统（lib.rs + 测试 + 日志）

这是一个使用 `lib.rs` 实现的可测试、带日志的账户系统，适用于 Solana 合约开发前的逻辑建模。

---

## 📁 项目结构

```
bank_ledger_lib/
├── src/
│   ├── bank.rs          # 核心逻辑 + 测试
│   └── main.rs         # 应用入口
├── Cargo.toml
```

---

## 📦 依赖配置（Cargo.toml）

```toml
[dependencies]
log = "0.4"
env_logger = "0.10"
```

---

## 📄 lib.rs

```rust
use std::collections::HashMap;
use log::{info, error};

#[derive(Debug)]
pub struct Account {
    pub owner: String,
    pub balance: u64,
}

#[derive(Debug)]
pub struct TransferRecord {
    pub record: Vec<Transfer>,
}

#[derive(Debug)]
pub struct Transfer {
    pub source: String,
    pub target: String,
    pub amount: u64,
}

impl Account {
    pub fn add(address: &str, owner: &str, balance: u64, map: &mut HashMap<String, Account>) -> Result<(), String> {
        if map.contains_key(address) {
            error!("账户 {} 已存在", address);
            return Err(format!("账户 {} 已存在", address));
        }
        let account = Account {
            owner: owner.to_string(),
            balance,
        };
        map.insert(address.to_string(), account);
        Ok(())
    }

    pub fn query(address: &str, map: &HashMap<String, Account>) -> Result<u64, String> {
        match map.get(address) {
            Some(v) => {
                info!("查询余额 {}: {}", address, v.balance);
                Ok(v.balance)
            }
            None => {
                error!("账户 {} 不存在", address);
                Err(format!("账户 {} 不存在", address))
            }
        }
    }

    pub fn transfer(
        source: &str,
        target: &str,
        amount: u64,
        map: &mut HashMap<String, Account>,
        transfer_list: &mut TransferRecord,
    ) -> Result<(), String> {
        if !map.contains_key(source) || !map.contains_key(target) {
            error!("地址不存在 source: {}, target: {}", source, target);
            return Err("地址不存在".to_string());
        }

        let source_account = map.get_mut(source).unwrap();
        if source_account.balance < amount {
            error!(
                "余额不足：{} 当前余额 {}, 转账金额 {}",
                source, source_account.balance, amount
            );
            return Err("余额不足".to_string());
        }

        source_account.balance -= amount;
        map.get_mut(target).unwrap().balance += amount;

        transfer_list.record.push(Transfer {
            source: source.to_string(),
            target: target.to_string(),
            amount,
        });

        info!("💸 转账成功：{} -> {} 金额 {}", source, target, amount);
        Ok(())
    }
}
```

---

## ✅ 单元测试（lib.rs 内部）

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_account_success() {
        let mut map = HashMap::new();
        let res = Account::add("alice", "Alice", 100, &mut map);
        assert!(res.is_ok());
        assert_eq!(map.get("alice").unwrap().balance, 100);
    }

    #[test]
    fn test_transfer_success() {
        let mut map = HashMap::new();
        let mut record = TransferRecord { record: vec![] };
        Account::add("alice", "Alice", 100, &mut map).unwrap();
        Account::add("bob", "Bob", 0, &mut map).unwrap();
        let res = Account::transfer("alice", "bob", 60, &mut map, &mut record);
        assert!(res.is_ok());
        assert_eq!(map["alice"].balance, 40);
        assert_eq!(map["bob"].balance, 60);
    }

    // 更多测试...
}
```

---

## 🚀 main.rs 示例运行

```rust
use std::collections::HashMap;
use bank_ledger_lib::*; // 假设 lib 名

fn main() {
    env_logger::init();

    let mut map = HashMap::new();
    let mut record = TransferRecord { record: vec![] };

    Account::add("alice", "Alice", 100, &mut map).unwrap();
    Account::add("bob", "Bob", 0, &mut map).unwrap();
    Account::transfer("alice", "bob", 50, &mut map, &mut record).unwrap();

    println!("Bob 余额: {}", Account::query("bob", &map).unwrap());
}
```

---

## 🧪 运行测试

```bash
cargo test
```

## 📢 启用日志

```bash
RUST_LOG=info cargo run
```

---

