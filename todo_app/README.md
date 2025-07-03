# 📝 Rust Todo CLI 应用

一个用 Rust 编写的简单命令行 Todo 应用，支持添加任务、查看任务、标记完成、删除任务，任务数据保存在本地 JSON 文件中。

---

## 🚀 项目功能

- ✅ 添加新任务
- 📋 查看所有任务
- ✔️ 标记任务完成
- 🗑 删除任务
- 💾 自动保存到 `todo.json`

---

## 🧱 项目结构

```
todo_app/
├── src/
│   ├── main.rs      // 命令行入口
│   └── todo.rs      // TodoList 和任务逻辑
├── todo.json        // 自动生成的任务数据文件
└── Cargo.toml       // Rust 项目配置
```

---

## 📦 依赖说明

在 `Cargo.toml` 中添加：

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 💻 使用方法

进入项目根目录后，使用 `cargo run --` 执行命令：

```bash
cargo run -- add 读《Rust 权威指南》
cargo run -- list
cargo run -- done 1
cargo run -- del 2
```

---

## 🧪 示例输出

```bash
$ cargo run -- list
[1]   [ ] 读《Rust 权威指南》
[2]   [x] 买牛奶
```

---

## 📂 数据保存格式（todo.json）

```json
[
  {
    "id": 1,
    "text": "读《Rust 权威指南》",
    "done": false
  },
  {
    "id": 2,
    "text": "买牛奶",
    "done": true
  }
]
```

---

## 🔧 待开发功能（扩展建议）

- [ ] 任务优先级管理
- [ ] 支持截止时间（due date）
- [ ] 导出任务为 Markdown/TXT
- [ ] 命令行颜色美化（使用 `colored` crate）

---

## 📜 License

MIT License. 欢迎自由学习和修改。

---

## 🙌 致谢

本项目用于 Rust 学习与实战练习。如果你也是 Rust 初学者，欢迎参考、改造，并加入更多自己的想法。