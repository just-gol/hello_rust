use chrono::{Duration, Utc};
use log::error;
use std::collections::{HashMap, HashSet};

// 用户
pub struct User {
    pub name: String,
    pub token: u32,
    pub role: Role,
}

// 用户角色
#[derive(PartialEq)]
pub enum Role {
    Admin,
    User,
}

// 账户锁定状态
#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    Lock,
    Norma,
}

// 被锁定的账户信息
pub struct Account {
    pub time: i64,
    pub token: u32,
    pub status: Status,
}

// 管理账户锁定逻辑
pub struct AccountManager {
    pub accounts: HashMap<String, Account>,
}

// 查询锁定状态结构
#[derive(Debug)]
pub struct LockStatus {
    pub lock_time: i64,
    pub status: Status,
}

impl AccountManager {
    pub fn lock(&mut self, token: u32, user: &mut User) -> Result<(), String> {
        if self.accounts.contains_key(&user.name) {
            error!("用户已存在");
            return Err("用户已存在".to_string());
        }

        if user.token < token {
            error!("token 不足");
            return Err("token 不足".to_string());
        }

        user.token -= token;

        let ten_days_later = Utc::now() + Duration::days(10);
        let timestamp = ten_days_later.timestamp();

        let account = Account {
            time: timestamp,
            token,
            status: Status::Lock,
        };

        self.accounts.insert(user.name.clone(), account);
        Ok(())
    }

    pub fn un_lock(&mut self, user: &mut User, current_time: i64) -> Result<(), String> {
        let account = self
            .accounts
            .get_mut(&user.name)
            .ok_or("用户不存在".to_string())?;

        if account.status != Status::Lock {
            error!("用户未锁定");
            return Err("用户未锁定".to_string());
        }

        if account.time > current_time {
            error!("用户锁定时间未到");
            return Err("用户锁定时间未到".to_string());
        }

        user.token += account.token;
        account.token = 0;
        account.status = Status::Norma;

        Ok(())
    }

    pub fn query_lock_status(&self, name: &str) -> Result<LockStatus, String> {
        let account = self.accounts.get(name).ok_or("用户不存在".to_string())?;
        Ok(LockStatus {
            lock_time: account.time,
            status: account.status.clone(),
        })
    }
}

impl User {
    pub fn new(name: String, user_set: &mut HashSet<String>, role: Role) -> Result<User, String> {
        if user_set.contains(&name) {
            return Err("用户已存在".to_string());
        }
        user_set.insert(name.clone());
        Ok(User {
            name,
            token: 0,
            role,
        })
    }

    pub fn mint(&self, amount: u32, to: &mut User) -> Result<(), String> {
        if self.role != Role::Admin {
            return Err("只有管理员才能铸造代币".to_string());
        }
        to.token += amount;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn init_logger() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Info)
            .try_init();
    }

    #[test]
    fn test_user_create_and_admin_mint() {
        init_logger();
        let mut user_set = HashSet::new();

        let admin = User::new("Alice".to_string(), &mut user_set, Role::Admin).unwrap();
        let mut bob = User::new("Bob".to_string(), &mut user_set, Role::User).unwrap();

        assert_eq!(bob.token, 0);
        let result = admin.mint(100, &mut bob);
        assert!(result.is_ok());
        assert_eq!(bob.token, 100);
    }

    #[test]
    fn test_lock_tokens_after_admin_mint() {
        let mut user_set = HashSet::new();
        let admin = User::new("Admin".to_string(), &mut user_set, Role::Admin).unwrap();
        let mut bob = User::new("Bob".to_string(), &mut user_set, Role::User).unwrap();

        // 通过管理员发币
        admin.mint(80, &mut bob).unwrap();
        assert_eq!(bob.token, 80);

        let mut manager = AccountManager {
            accounts: HashMap::new(),
        };

        let result = manager.lock(50, &mut bob);
        assert!(result.is_ok());
        assert_eq!(bob.token, 30); // 剩余 token
    }

    #[test]
    fn test_unlock_tokens_after_mint_and_lock() {
        let mut user_set = HashSet::new();
        let admin = User::new("Admin".to_string(), &mut user_set, Role::Admin).unwrap();
        let mut bob = User::new("Bob".to_string(), &mut user_set, Role::User).unwrap();

        admin.mint(60, &mut bob).unwrap();

        let mut manager = AccountManager {
            accounts: HashMap::new(),
        };

        manager.lock(60, &mut bob).unwrap();

        let unlock_time = Utc::now().timestamp() + 10_000; // 模拟已超过锁仓期
        let result = manager.un_lock(&mut bob, unlock_time);

        assert!(result.is_ok());
        assert_eq!(bob.token, 60);
        assert_eq!(
            manager.query_lock_status("Bob").unwrap().status,
            Status::Norma
        );
    }
}
