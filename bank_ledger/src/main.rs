mod bank;
use bank::{Account, TransferRecord};
use std::collections::HashMap;
fn main() {
    env_logger::init();

    let mut accounts: HashMap<String, Account> = HashMap::new();
    let mut record = TransferRecord { record: Vec::new() };

    Account::add("Alice", "Alice", 100, &mut accounts).unwrap();
    Account::add("Bob", "Bob", 50, &mut accounts).unwrap();

    match Account::query("Alice", &accounts) {
        Ok(bal) => println!("Alice 余额: {}", bal),
        Err(e) => println!("{}", e),
    }

    match Account::query("Bob", &accounts) {
        Ok(bal) => println!("Bob 余额: {}", bal),
        Err(e) => println!("{}", e),
    }

    if let Err(e) = Account::transfer("Alice", "Bob", 30, &mut accounts, &mut record) {
        println!("{}", e);
    }

    println!("转账后:");

    match Account::query("Alice", &accounts) {
        Ok(bal) => println!("Alice 余额: {}", bal),
        Err(e) => println!("{}", e),
    }

    match Account::query("Bob", &accounts) {
        Ok(bal) => println!("Bob 余额: {}", bal),
        Err(e) => println!("{}", e),
    }

    println!("📜 转账记录: {:#?}", record);
}
