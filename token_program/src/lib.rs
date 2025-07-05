use log::{error, info};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Account {
    pub balance: u64,
    pub role: Role,
    pub is_active: bool,
}

pub struct TokenSystem {
    pub accounts: HashMap<String, Account>,
}

impl TokenSystem {
    pub fn new() -> Self {
        TokenSystem {
            accounts: HashMap::new(),
        }
    }

    pub fn create_account(&mut self, name: &str, role: Role) -> Result<(), String> {
        if self.accounts.contains_key(name) {
            error!("账号已存在:{}", name);
            return Err("账号已存在".to_string());
        }
        self.accounts.insert(
            name.to_string(),
            Account {
                balance: 0,
                role,
                is_active: true,
            },
        );
        Ok(())
    }

    pub fn get_balance(&self, name: &str) -> Result<u64, String> {
        match self.accounts.get(name) {
            Some(acc) if acc.is_active => Ok(acc.balance),
            Some(_) => Err("账号封禁".to_string()),
            None => Err("账号不存在".to_string()),
        }
    }

    pub fn mint(&mut self, caller: &str, to: &str, amount: u64) -> Result<(), String> {
        let admin = self.accounts.get(caller).ok_or("调用者不存在")?;
        if admin.role != Role::ADMIN {
            return Err("非管理员不可以mint".to_string());
        }

        let target = self.accounts.get_mut(to).ok_or("目标账号不存在")?;
        if !target.is_active {
            return Err("账号禁用".to_string());
        }

        target.balance += amount;
        Ok(())
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let sender = self.accounts.get_mut(from).ok_or("发送者账户不存在")?;
        if !sender.is_active {
            return Err("发送者被禁用".to_string());
        }
        if sender.balance < amount {
            return Err("发送者金额小于转账金额".to_string());
        }
        sender.balance -= amount;

        let receiver = self.accounts.get_mut(to).ok_or("目标账户不存在")?;
        if !receiver.is_active {
            return Err("目标账户被禁用".to_string());
        }
        receiver.balance += amount;
        Ok(())
    }

    pub fn disable_account(&mut self, caller: &str, target: &str) -> Result<(), String> {
        let admin = self.accounts.get(caller).ok_or("调用者不存在")?;
        if admin.role != Role::ADMIN {
            return Err("非管理员不能禁用账户".to_string());
        }

        let acc = self.accounts.get_mut(target).ok_or("目标账户不存在")?;
        acc.is_active = false;
        Ok(())
    }

    pub fn enable_account(&mut self, caller: &str, target: &str) -> Result<(), String> {
        let admin = self.accounts.get(caller).ok_or("调用者不存在")?;
        if admin.role != Role::ADMIN {
            return Err("非管理员不能启用账户".to_string());
        }

        let acc = self.accounts.get_mut(target).ok_or("目标账户不存在")?;
        acc.is_active = true;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Role {
    ADMIN,
    USER,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_account_creation_and_mint() {
        let mut sys = TokenSystem::new();
        sys.create_account("admin", Role::ADMIN).unwrap();
        sys.create_account("user1", Role::USER).unwrap();

        assert!(sys.mint("user1", "user1", 50).is_err()); // 非管理员 mint
        assert!(sys.mint("admin", "user1", 100).is_ok());

        assert_eq!(sys.get_balance("user1").unwrap(), 100);
    }

    #[test]
    fn test_transfer_logic() {
        let mut sys = TokenSystem::new();
        sys.create_account("admin", Role::ADMIN).unwrap();
        sys.create_account("a", Role::USER).unwrap();
        sys.create_account("b", Role::USER).unwrap();
        sys.mint("admin", "a", 100).unwrap();

        assert!(sys.transfer("a", "b", 30).is_ok());
        assert_eq!(sys.get_balance("a").unwrap(), 70);
        assert_eq!(sys.get_balance("b").unwrap(), 30);

        assert!(sys.transfer("a", "b", 999).is_err()); // 超额转账
    }

    #[test]
    fn test_disable_and_enable() {
        let mut sys = TokenSystem::new();
        sys.create_account("admin", Role::ADMIN).unwrap();
        sys.create_account("u", Role::USER).unwrap();
        sys.mint("admin", "u", 100).unwrap();

        sys.disable_account("admin", "u").unwrap();
        assert!(sys.transfer("u", "admin", 10).is_err()); // 禁用不能转账

        sys.enable_account("admin", "u").unwrap();
        assert!(sys.transfer("u", "admin", 10).is_ok()); // 恢复后转账成功
    }
}
