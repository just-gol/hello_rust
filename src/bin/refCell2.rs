use std::cell::RefCell;
use std::rc::{Rc, Weak};
fn main() {
    let a = User::new("小明");
    let b = User::new("小花");
    let c = User::new("小化");
    User::add(&a, &b);
    User::add(&a, &c);

    User::show(&a);
    User::show(&b);
    User::show(&c);
}
#[derive(Debug)]
struct User {
    name: String,
    friends: RefCell<Vec<Weak<User>>>,
}

impl User {
    fn new(name: &str) -> Rc<User> {
        Rc::new(User {
            name: name.to_string(),
            friends: RefCell::new(vec![]),
        })
    }

    fn add(a: &Rc<User>, b: &Rc<User>) {
        a.friends.borrow_mut().push(Rc::downgrade(b));
        b.friends.borrow_mut().push(Rc::downgrade(a));
    }

    fn show(user: &Rc<User>) {
        println!("{}的好友", user.name);
        // 获取好友列表
        // 获取每一个好友
        for v in user.friends.borrow().iter() {
            if let Some(f1) = v.upgrade() {
                println!("{}", f1.name)
            }
        }
    }
}
