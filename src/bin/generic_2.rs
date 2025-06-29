use std::usize;

fn main() {
    // let p1 = Point { x: 1, y: 2 };
    // let p2 = Point { x: 1.1, y: 2.2 };
    // let p3 = p1.mixup(p2);
    // println!("{}-{}", p3.x, p3.y);
    let a = [1, 2, 3];
    arr(a);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<T1, U1>(self, other: Point<T1, U1>) -> Point<T, U1> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
