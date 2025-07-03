use std::{env, u32};

use crate::todo::ToList;
mod todo;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut list = ToList::load();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" => {
            let text = args[2..].join(" ");
            list.add(text);
        }
        "list" => {
            list.list();
        }
        "done" => {
            let id = args[2].parse::<u32>().unwrap();
            list.complete(id);
        }
        "del" => {
            let id = args[2].parse::<u32>().unwrap();
            list.deleted(id);
        }
        _ => print_help(),
    }
}
fn print_help() {
    println!("使用方式:");
    println!("  add <任务内容>      添加任务");
    println!("  list                查看任务");
    println!("  done <任务id>       标记完成");
    println!("  del <任务id>        删除任务");
}
