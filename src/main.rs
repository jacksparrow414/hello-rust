use std::cmp::{min, max};
fn main() {
    let _X: i32 = 42;
    let pair: (char, i32) = ('a', 17);
    let a = (10, 20);
    a.0;
    // In this example, std is a crate (~ a library), cmp is a module (~ a source file), and min is a function:
    let least = std::cmp::min(3, 8);
    // let v = Vec::new();
    println!("Hello, world!");
    let minus_two = Number {
        odd: false,
        value: -2,
    };
    println!("positive? {}", minus_two.is_strictly_positive());
}

fn greet() {
    println!("Hi there!");
}

fn fair_die_roll() -> i32 {
    return 4;
}

struct Vec2 {
    x: f64, // 64-bit floating point, aka "double precision"
    y: f64,
}

struct Number {
    odd: bool,
    value: i32,
}
impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}
