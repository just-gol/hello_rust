use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    let a = String::from("xxx");
    let ac = Rc::new(a);
    let bc = &ac;

    let aa = RefCell::new(1);
    let mut bb = aa.borrow_mut();
    let c = *bb += 1;
    println!("{:?}", ac);
    println!("{:?}", bc);

    // let a = Rc::new(Node {
    //     value: 2,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![]),
    // });

    // let b = Rc::new(Node {
    //     value: 1,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![a.clone()]),
    // });

    // *a.parent.borrow_mut() = Rc::downgrade(&b);

    // println!(
    //     "leaf parent = {:?}",
    //     a.parent.borrow().upgrade().map(|n| n.value)
    // );

    // let a = Rc::new(RefCell::new(Node {
    //     value: 1,
    //     next: None,
    //     prev: None,
    // }));

    // let b = Rc::new(RefCell::new(Node {
    //     value: 2,
    //     next: None,
    //     prev: None,
    // }));

    // a.borrow_mut().next = Some(Rc::clone(&b));
    // b.borrow_mut().prev = Some(Rc::downgrade(&a));

    // if let Some(prev_weak) = &b.borrow().prev {
    //     if let Some(rc) = prev_weak.upgrade() {
    //         let node = rc.borrow();
    //         println!("{:?}", node);
    //     }
    // }
}
#[derive(Debug)]
struct User {
    name: String,                      // 用户名称
    friends: RefCell<Vec<Weak<User>>>, // 好友列表
}

// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,      // 父节点存引用
//     children: RefCell<Vec<Rc<Node>>>, // 子节点存Node
// }
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}
