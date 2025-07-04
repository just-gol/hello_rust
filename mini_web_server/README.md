# 🌐 Rust 多线程迷你 Web 服务器

这是一个使用 Rust 编写的多线程版 Web 服务器，支持基本的 HTTP 请求处理。每个客户端连接都在独立线程中处理，实现了最基础的并发能力。

---

## ✨ 功能特性

- 🚀 监听本地端口，处理浏览器请求
- 📄 支持返回静态 HTML 文件
- ❌ 简单 404 页面支持
- 🧵 每个请求独立线程处理（非阻塞）

---

## 🧱 项目结构

```
mini_web_server/
├── src/
│   └── main.rs         # 主程序入口
├── hello.html          # 首页内容
├── 404.html            # 404 页面内容
└── Cargo.toml          # Rust 项目配置
```

---

## 📦 依赖说明

仅使用标准库，无需额外依赖：

```toml
[package]
name = "mini_web_server"
version = "0.1.0"
edition = "2024"

[dependencies]
```

---

## 🔨 使用说明

### 1. 运行服务器

```bash
cargo run
```

默认监听地址为：

```
http://0.0.0.0:8888
```

### 2. 测试访问

在浏览器中访问：

- ✅ [http://localhost:8888/](http://localhost:8888/) 返回 hello 页面
- ❌ [http://localhost:8888/xyz](http://localhost:8888/xyz) 返回 404 页面

或使用 curl：

```bash
curl http://localhost:8888/
curl http://localhost:8888/unknown
```

---

## 🧠 核心原理

- 使用 `TcpListener` 监听端口
- 每个请求用 `thread::spawn` 创建线程处理
- 简单解析 HTTP 请求头中的路径
- 返回静态文件内容作为响应

---

## 📚 学习来源

灵感来自《The Rust Programming Language》第 20 章：构建 Web Server。

