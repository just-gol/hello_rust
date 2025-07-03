use List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    // let a = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Nil)))));
    // let b = Cons(4, a.clone());
    // let c = Cons(2, a.clone());
    // println!("b->{:?}", b);
    // println!("c->{:?}", c);

    let a = Cons(2, Box::new(Cons(3, Box::new(Nil))));
    let b = Cons(1, Box::new(a));
    let c = Cons(0, Box::new(a));
    println!("{:?}", b);
}
// #[derive(Debug)]
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
