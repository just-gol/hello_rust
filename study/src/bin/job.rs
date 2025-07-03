fn main() {
    let book = Book {
        title: "rust",
        author: "lsy",
    };
    println!("{}-{}", book.title, book.author);
}

/***
 * 定义一个结构体 Book，其中包含字段 title（字符串切片）和 author（字符串切片）。然后定义一个生命周期参数，确保该结构体的字段不会在结构体引用之后失效。

要求：

定义一个 Book 结构体，其中包含 title 和 author 字段，它们是字符串切片。

定义一个生命周期参数，确保 Book 的生命周期符合借用规则。

在 main 函数中创建一个 Book 实例并访问它的字段。
 */
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}
