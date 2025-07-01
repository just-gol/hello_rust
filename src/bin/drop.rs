use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

fn main() {}
#[test]
fn test_drop_1() {
    struct MyBox {
        data: String,
    }

    impl Drop for MyBox {
        fn drop(&mut self) {
            println!("drop....")
        }
    }

    let a1 = MyBox {
        data: "drop1".to_string(),
    };
    let a2 = MyBox {
        data: "drop1".to_string(),
    };
    drop(a1);
    drop(a2);
    println!("end");
}

#[test]
fn test_rc_1() {
    let a = Rc::new(String::from("value"));
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&b), Rc::strong_count(&a));
}

#[test]
fn test_cell_1() {
    let c = Cell::new("value");
    let one = c.get();
    c.set("key");
    let two = c.get();
    println!("{},{}", one, two);
}

#[test]
fn test_refCell_1() {
    let x = RefCell::new(5);
    {
        let mut v = x.borrow_mut();
        *v += 1;
    }
    println!("x={}", x.borrow());
}
