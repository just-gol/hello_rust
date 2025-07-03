use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;
use mini_grep::{search, search_case_insensitive};
use std::{env, fs};
fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    dotenv().ok();

    // 获取命令参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: cargo run -- <文件名> <关键词>");
        return;
    }

    let file_name = &args[1];
    let key_word = &args[2];

    // 读取文件
    let content = fs::read_to_string(file_name).unwrap();

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    log::info!("ignore_case:{}", ignore_case);

    if ignore_case {
        for v in search(key_word, &content) {
            println!("{}", v);
        }
    } else {
        for v in search_case_insensitive(key_word, &content) {
            println!("{}", v);
        }
    }
}
