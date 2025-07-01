fn main() {
    //     let x = 1;
    //     draw1(Box::new(x));

    //     let arr: Vec<Box<dyn Animal>> = vec![
    //         Box::new(Dog {
    //             eat: "骨头".to_string(),
    //         }),
    //         Box::new(Car {
    //             eat: "鱼".to_string(),
    //         }),
    //     ];
    //     for item in arr {
    //         println!("{}", item.eat());
    //     }

    //     let result = eat1(&Dog {
    //         eat: "骨头".to_string(),
    //     });
    //     println!("{}", result);
    // }
    let result = MyInt(32).convert();
    println!("{}", result);
}

trait Animal {
    fn eat(&self) -> &String;
}

// 狗
struct Dog {
    eat: String,
}
// 猫
struct Car {
    eat: String,
}

impl Animal for Dog {
    fn eat(&self) -> &String {
        &self.eat
    }
}

impl Animal for Car {
    fn eat(&self) -> &String {
        &self.eat
    }
}

fn eat1(item: &dyn Animal) -> String {
    item.eat().clone()
}

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8:{}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64:{}", self)
    }
}

fn draw1(x: Box<dyn Draw>) {
    let str = x.draw();
    println!("{}", str);
}

trait Converter {
    type Output;
    fn convert(&self) -> Self::Output;
}

struct MyInt(u32);

impl Converter for MyInt {
    type Output = u32;
    fn convert(&self) -> Self::Output {
        self.0
    }
}
