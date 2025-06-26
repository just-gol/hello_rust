fn main(){
  let p = Person{
    name:"xiaoming".to_string(),
    age:18
  };
  notify(&p);
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T:Summary>(item:&T){
  println!("Breaking news! {}", item.summarize());
}

pub trait Summary{
  fn summarize(&self)->&String;
}

struct Person{
  name:String,
  age:u8,
}

impl Summary for Person{
  fn summarize(&self)->&String{
      &self.name
  }
}

