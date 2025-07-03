use serde_json;
use std::{fs, path::Path};
// 定义任务结构体

// 该结构体需要进行序列化,添加宏
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    // 任务id
    pub id: u32,
    // 任务内容
    pub text: String,
    // 任务是否完成
    pub done: bool,
}

// 创建任务集合
pub struct ToList {
    pub items: Vec<Todo>,
}

// 创建文件
const FILE_PATH: &str = "todo.json";

//创建方法
impl ToList {
    pub fn load() -> ToList {
        // 判断文件是否存在
        if Path::new(FILE_PATH).exists() {
            let data = fs::read_to_string(FILE_PATH).unwrap();
            let items = serde_json::from_str(&data).unwrap();
            ToList { items }
        } else {
            ToList { items: vec![] }
        }
    }

    pub fn add(&mut self, text: String) {
        let len = self.items.len();
        // 创建任务id
        let id = len as u32 + 1;
        self.items.push(Todo {
            id,
            text,
            done: false,
        });
        // 写入文件中
        self.save();
    }
    pub fn save(&self) {
        // 序列化
        let str = serde_json::to_string_pretty(&self.items);
        // 写入文件
        fs::write(FILE_PATH, str.unwrap()).unwrap();
    }

    pub fn list(&self) {
        for item in &self.items {
            println!(
                "任务id:{},内容:{},完成状态:{}",
                item.id, item.text, item.done
            );
        }
    }

    pub fn complete(&mut self, id: u32) {
        let item = self.items.iter_mut().find(|item| item.id == id);
        if let Some(i) = item {
            i.done = true;
            // 更新文件
            self.save();
        }
    }

    pub fn deleted(&mut self, id: u32) {
        let items = &mut self.items;
        items.retain(|x| x.id != id);
        self.save();
    }
}
