fn main() {
    let mut item = Fibonacci { a: 0, b: 1 };
    for v in item.take(10) {
        println!("{}", v);
    }
}

struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        self.a = self.b;
        self.b = current + self.b;
        Some(current)
    }
}
