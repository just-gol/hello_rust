fn main() {
    let car = Car::new("car".to_string());
    let c = car.color();
    let p = car.process(&car.name);
    println!("{:?}", c);
    println!("{:?}", p);

    let house = House { name: "house" };
    let h = house.describe();
    println!("{h}");
}

trait Vehicle: Paint + AnotherTrait {}

trait Paint {
    fn color(&self) -> String;

    fn new(name: String) -> Self;
}

trait AnotherTrait {
    fn process<'a>(&self, part: &'a str) -> &'a str;
}

struct Car {
    name: String,
}

impl Paint for Car {
    fn color(&self) -> String {
        // 添加 &self，变成实例方法
        String::from("value")
    }

    fn new(name: String) -> Self {
        Car { name }
    }
}

impl AnotherTrait for Car {
    fn process<'a>(&self, part: &'a str) -> &'a str {
        part
    }
}

struct House<'a> {
    name: &'a str,
}

trait HoustTrait<'a> {
    fn describe(&self) -> &'a str;
}
impl<'a> HoustTrait<'a> for House<'a> {
    fn describe(&self) -> &'a str {
        self.name
    }
}
