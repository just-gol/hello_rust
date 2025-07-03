use std::{arch::x86_64, collections::HashMap};

fn main() {
    let mut books: Vec<&str> = Vec::new();

    let mut map: HashMap<&str, u32> = HashMap::new();

    add("一路向北", &mut books, &mut map);
    add("一路向xi", &mut books, &mut map);

    println!("{:?}", map);
    update("一路向北", &mut map, 20);
    println!("{:?}", map);

    // del("一路向北", &mut books, &mut map);
    // println!("{:?}{:?}", books, map);
    let num = queryStore("一路向北", &mut map);
    println!("{}", num);

    let str = maxNum(&mut books, &mut map);
    println!("{}", str);
}

fn add<'a>(name: &'a str, bookList: &mut Vec<&'a str>, m: &mut HashMap<&'a str, u32>) {
    bookList.push(name);
    m.entry(name).and_modify(|v| *v += 1).or_insert(1);
}

fn del<'a>(name: &'a str, bookList: &mut Vec<&'a str>, m: &mut HashMap<&'a str, u32>) {
    bookList.retain(|&book| book != name);
    m.remove(name);
}

fn queryStore<'a>(name: &'a str, m: &mut HashMap<&'a str, u32>) -> u32 {
    if let Some(v) = m.get(&name) { *v } else { 0 }
}

fn update<'a>(name: &'a str, m: &mut HashMap<&'a str, u32>, num: u32) {
    m.insert(name, num);
}

fn maxNum<'a>(bookList: &mut Vec<&'a str>, m: &mut HashMap<&'a str, u32>) -> &'a str {
    let mut book = bookList[0];
    let mut max_count = m.get(book).unwrap_or(&0);
    for item in bookList {
        let count = m.get(item).unwrap_or(&0);
        if max_count < count {
            book = item;
            max_count = count;
        }
    }
    book
}

#[test]
fn test_1() {
    let arr = vec!["apple", "banana", "apple", "orange", "banana"];
    let mut map = HashMap::new();
    for item in arr {
        map.entry(item).and_modify(|x| *x += 1).or_insert(1);
    }

    println!("{:?}", map);
}
