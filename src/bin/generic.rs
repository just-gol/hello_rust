use std::fmt::Display;

fn main(){
  // let arr = vec![1,2,3,4,5];
  // println!("{}",largest(&arr));


  let p: Point<i32> = Point{x:1,y:2};
  println!("{},{}",p.x,p.y);


  let person = Person{age:18,height:170.2};
  println!("{},{}",person.age,person.height);

  println!("{}",p.x);

  create_and_print::<f64>();
}

// 泛型函数
fn largest<T: PartialOrd+Copy>(list:&[T])->T{
  let mut largest = list[0];
  for &item in list{
    if item > largest{
      largest = item;
    }
  }
  largest
}

// 泛型结构体
struct Point<T>{
  x:T,
  y:T,
}

// 多泛型
struct Person<T,U>{
  age:T,
  height:U,
}

// 泛型方法
impl<T> Point<T>{
  fn x(&self)->&T{
    &self.x
  }
}

fn create_and_print<T>()where T:From<i32> +Display{
  let a:T= 100.into();
  println!("a is:{}",a)
}