use log::{error, info};
use std::collections::HashMap;

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
    pub fn add(
        address: &str,
        owner: &str,
        balance: u64,
        map: &mut HashMap<String, Account>,
    ) -> Result<(), String> {
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

        info!("转账成功：{} -> {} 金额 {}", source, target, amount);
        Ok(())
    }
}

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
}
