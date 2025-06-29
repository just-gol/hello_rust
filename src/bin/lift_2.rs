fn main() {
    let s: &'static str = "hello world";
    method(s);
    println!("{}", s);
}

fn method(a: &'static str) {
    println!("{}", a);
}

#[test]
fn test_lifttime_mut() {
    fn insert_vale<'a, 'b>(my_vec: &mut Vec<&'a i32>, value: &'b i32)
    where
        'b: 'a,
    {
        my_vec.push(value);
    }

    let x = 1;
    let mut my_vec = vec![&x];
    let y = 2;
    insert_vale(&mut my_vec, &y);
}
