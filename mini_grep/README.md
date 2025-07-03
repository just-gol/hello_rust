# 🔍 Rust Mini-Grep：简易文件搜索工具

这是一个用 Rust 编写的命令行文件搜索工具，模仿 Unix 工具 `grep` 的核心功能。

支持根据关键词搜索文本文件，并支持**忽略大小写**的匹配（通过环境变量控制）。

---

## ✨ 功能特性

- ✅ 按关键字搜索文件中的文本
- 🔡 支持区分大小写与忽略大小写搜索
- 🌍 使用环境变量 `IGNORE_CASE=true` 控制是否忽略大小写
- 📦 基于标准库，无外部依赖

---

## 🧱 项目结构

```
mini_grep/
├── src/
│   ├── main.rs         // 程序入口，参数解析
│   └── lib.rs          // 核心搜索逻辑封装
├── poem.txt            // 示例文本文件
└── Cargo.toml          // 项目配置
```

---

## 🚀 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/just-gol/hello_rust.git
cd mini_grep
```

### 2. 编译运行

#### ✅ 默认区分大小写搜索：

```bash
cargo run -- poem.txt Rust
```

#### 🔁 忽略大小写搜索：

```bash
IGNORE_CASE=true cargo run -- poem.txt rust
```

---

## ⚙️ 使用说明

程序运行格式：

```bash
cargo run -- <文件名> <搜索关键词>
```

示例：

```bash
cargo run -- poem.txt dream
```

设置忽略大小写（Linux/macOS）：

```bash
export IGNORE_CASE=true
cargo run -- poem.txt Dream
```

Windows PowerShell：

```powershell
$env:IGNORE_CASE = "true"
cargo run -- poem.txt Dream
```

---

## 🧪 示例输出

搜索 "rust"（不忽略大小写）：

```
Rust:
safe, fast, productive.
Pick three.
```

---

## 🧠 背后原理

- 使用 `std::env::args()` 读取命令行参数
- 使用 `std::fs::read_to_string` 读取文件
- 使用 `.lines().filter().collect()` 搜索匹配行
- 可选从 `std::env::var("IGNORE_CASE")` 控制匹配行为

---

## 📚 学习来源

本项目灵感来自 [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) 第 12 章。


