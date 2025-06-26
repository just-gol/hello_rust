mod front_of_house; // 声明模块：去 src/front_of_house/mod.rs 找

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
}
