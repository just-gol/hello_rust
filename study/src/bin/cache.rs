fn main() {
    let mut x = Cache::new(|x| x + 1);
    x.get_cache(1);
    x.get_cache(2);
}

struct Cache<T>
where
    T: Fn(i32) -> i32,
{
    query: T,
    value: Option<i32>,
}

impl<T> Cache<T>
where
    T: Fn(i32) -> i32,
{
    fn new(query: T) -> Cache<T> {
        Cache { query, value: None }
    }

    fn get_cache(&mut self, c: i32) -> i32 {
        match self.value {
            Some(v) => {
                println!("缓存中获取{}", v);
                v
            }
            None => {
                let new_value = (self.query)(c);
                println!("第一次获取:{}", new_value);
                self.value = Some(new_value);
                new_value
            }
        }
    }
}
