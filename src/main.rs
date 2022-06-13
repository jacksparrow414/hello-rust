use std::cmp::{min, max};
fn main() {
    println!("1 - 2 = {}", 1i32 - 2);
    let _X: i32 = 42;
    let pair: (char, i32) = ('a', 17);
    let a = (10, 20);
    a.0;
    // In this example, std is a crate (~ a library), cmp is a module (~ a source file), and min is a function:
    let least = std::cmp::min(3, 8);
    // let v = Vec::new();
    // All of name!(), name![] or name!{} invoke a macro. Macros just expand to regular code.
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

/**
 * 返回类型为i32
 */
fn fair_die_roll() -> i32 {
    return 4;
}
/**
 * 函数写法
 * fn methodName(arg: type) -> returnType{}
 */
fn fair_dice_roll_if(feeling_lucky: bool) -> i32 {
    if feeling_lucky {
        6
    } else {
        4
    }
}
/**
 * 泛型函数
 * fn methodName<T>(arg: T) -> T {}
 */
// fn foobar<T>(arg: T) -> T {
//     // do something with `arg`
// }

fn fair_dice_roll_match(feeling_lucky: bool) -> i32 {
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}

struct Vec2 {
    x: f64, // 64-bit floating point, aka "double precision"
    y: f64,
}
/**
 * 结构体
 * struct structName {
 *   attribute_1: type,
 *   attribute_2: type
 * }
 */
struct Number {
    odd: bool,
    value: i32,
}
/**
 * 为结构体添加方法
 * impl StructName {}
 */
impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}
