use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    // let five = Rc::new(5);
    // let weak_five = Rc::downgrade(&five);
    // let strong_five = weak_five.upgrade();
    // assert_eq!(*strong_five.unwrap(), 5);
    // drop(five);
    // let s = weak_five.upgrade();
    // assert_eq!(s, None);

    let r1 = Rc::new(Owner {
        name: String::from("xxx"),
        gadgets: RefCell::new(vec![]),
    });

    let g = Rc::new(Gadget {
        id: 1,
        owner: r1.clone(),
    });

    r1.gadgets.borrow_mut().push(Rc::downgrade(&g));

    println!("{:?}", r1);
}
#[derive(Debug)]
struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

struct Gadget {
    id: u32,
    owner: Rc<Owner>,
}
