use std::{cell::RefCell, iter::Sum, rc::Rc};

fn main() {
    let a = String::from("value");
    let b = a;
    println!("{}", b);
    println!("{}", a);
}

struct Article {
    title: String,
}

struct Tweet {
    text: String,
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Article {
    fn summarize(&self) -> String {
        todo!()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        todo!()
    }
}

fn f1<A: Summary, B: Summary>(a: A, b: B) {}

fn f2<T: Summary>(a: T, b: T) {}

fn rf_and_ref() -> Rc<RefCell<String>> {
    let a = Rc::new(RefCell::new(String::from("hello")));
    let a1 = Rc::clone(&a);
    Rc::clone(&a)
}

pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }
}
