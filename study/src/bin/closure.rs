fn main() {}

#[test]
fn test_closure_1() {
    let add = |a: i32, b: i32| a + b;
    let sum = add(3, 4);
    println!("{}", sum);
}

#[test]
fn test_closure_2() {
    struct Cache<T>
    where
        T: Fn(u32) -> u32,
    {
        query: T,
        value: Option<u32>,
    }

    impl<T> Cache<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(query: T) -> Cache<T> {
            Cache { query, value: None }
        }

        fn value(&mut self, args: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(args);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut method = Cache::new(|x| x + 1);
    let item = method.value(1);
    println!("{}", item);
}

#[test]
fn test_closure_3() {
    fn process<F: Fn()>(f: F) {
        f();
    }

    let f = || println!("这是一个无参只读的闭包函数");
    process(f);
}
#[test]
fn test_closure_4() {
    fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy,
    {
        println!("==={}===", func(3));
        println!("{}", func(4));
    }

    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len())
}
#[test]
fn test_closure_5() {
    fn call_fn_mut<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
        f();
        f();
    }

    let mut count = 1;
    let f = || {
        count += 1;
        println!("count:{}", count);
    };
    call_fn_mut(f);
}
