use std::cmp::{min, max};
/**
 * 单元类型就是 (),不占用任何内存，也就是0字节
 * main 函数就返回这个单元类型 ()，你不能说 main 函数无返回值
 * 
 * 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数
 */
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
    define_loop();
}
//Rust 提供了一个非常简洁的方式，用来生成连续的数值，
//例如 1..5，生成从 1 到 4 的连续数字，不包含 5 ；
//1..=5，生成从 1 到 5 的连续数字，包含 5，它的用途很简单，常常用于循环中：

//序列只允许用于数字或字符类型
//显示返回()单元类型，或者不声明
fn define_loop() -> (){
    for i in 0..5 {
        println!("{}", i);
    }
    for i in 0..=5 {
        println!("{}", i)
    }
    for i in 'a'..='z' {
        println!("{}",i);
    }
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
 * 函数写法,函数名规范为蛇形命名aa_bb
 * fn method_name(arg: type) -> returnType{}
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
 * fn method_name<T>(arg: T) -> T {}
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
