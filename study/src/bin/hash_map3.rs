use std::collections::HashMap;

fn main() {
    let mut lib = Library {
        books: vec![],
        map: HashMap::new(),
    };

    lib.add_book("1本书");
    lib.add_book("1本书");
    lib.add_book("2本书");
    lib.add_book("3本书");

    println!("add -> {:?}", lib.map);

    lib.borrow_book("1本书");
    println!("borrow -> {:?}", lib.map);

    let v = lib.query("1本书");
    println!("查询: {}", v);
}

struct Library {
    books: Vec<String>,
    map: HashMap<String, u32>,
}

impl Library {
    fn add_book(&mut self, name: &str) {
        self.books.push(name.to_string());
        self.map
            .entry(name.to_string())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    fn borrow_book(&mut self, name: &str) -> bool {
        if let Some(count) = self.map.get_mut(name) {
            if *count > 0 {
                *count -= 1;
                return true;
            }
        }
        false
    }

    fn query(&self, name: &str) -> u32 {
        *self.map.get(name).unwrap_or(&0)
    }
}
