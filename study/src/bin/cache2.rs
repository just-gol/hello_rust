use std::{collections::HashMap, hash::Hash};

fn main() {
    let mut page_cache = PageCache::new(|user_id: &String, article_id: &u32| -> String {
        println!(
            "Rendering page for user {} and article {}",
            user_id, article_id
        );
        format!(
            "Rendered HTML content for user {} and article {}",
            user_id, article_id
        )
    });

    let key1 = String::from("key1");
    let key2 = String::from("key2");
    println!("{}", page_cache.get_page(key1, 42));
    println!("{}", page_cache.get_page(key2, 42));
    // println!("{}", page_cache.get_page(22, 42));
    // println!("{}", page_cache.get_page(33, 42));
}

struct PageCache<U, A, F>
where
    U: Eq + Hash + Clone,
    A: Eq + Hash + Clone,
    F: Fn(&U, &A) -> String,
{
    render: F,
    cache: HashMap<(U, A), String>,
}

impl<U, A, F> PageCache<U, A, F>
where
    U: Eq + Hash + Clone,
    A: Eq + Hash + Clone,
    F: Fn(&U, &A) -> String,
{
    fn new(render: F) -> Self {
        Self {
            render,
            cache: HashMap::new(),
        }
    }

    fn get_page(&mut self, user_id: U, article: A) -> String {
        let key = (user_id.clone(), article.clone());
        let value = self.cache.get(&key);
        if let Some(v) = value {
            v.clone()
        } else {
            let html = (self.render)(&user_id, &article);
            self.cache.insert(key, html.clone());
            html.clone()
        }
    }
}
