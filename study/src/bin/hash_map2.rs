use std::{arch::x86_64, collections::HashMap};

fn main() {
    let mut lib = Library {
        books: vec![],
        map: HashMap::new(),
    };
    Library::add_book(&mut lib, "1本书");
    Library::add_book(&mut lib, "1本书");
    Library::add_book(&mut lib, "2本书");
    Library::add_book(&mut lib, "3本书");
    println!("add->{:?}", lib.map);
    Library::borrow_book(&mut lib, "1本书");
    println!("borrow->{:?}", lib.map);
    let v = Library::query(&lib, "1本书");
    println!("查询:{}", v);
}

struct Library<'a> {
    books: Vec<&'a str>,
    map: HashMap<&'a str, u32>,
}

impl<'a> Library<'a> {
    fn add_book(&mut self, name: &'a str) {
        self.books.push(name);
        self.map.entry(name).and_modify(|x| *x += 1).or_insert(1);
    }

    fn borrow_book(&mut self, name: &'a str) -> bool {
        let book = self.map.get(name).unwrap_or(&0);
        if *book != 0 {
            self.map.entry(name).and_modify(|x| *x -= 1).or_insert(0);
            true
        } else {
            false
        }
    }

    fn query(&self, name: &str) -> u32 {
        *self.map.get(name).unwrap_or(&0)
    }
}
