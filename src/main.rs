use std::{cmp::{min, max}, fmt::Display};
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
    print_user();
    println!("positive? {}", minus_two.is_strictly_positive());
    basic_type();
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
/**
 * Rust 基本类型都是通过【自动拷贝】的方式来赋值的
 * Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。
 * 如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用
 */
fn basic_type() {
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
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
/**
 * 元组不能过长，超过12就报错了
 */
fn my_tuple() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}
struct Vec2 {
    x: f64, // 64-bit floating point, aka "double precision"
    y: f64,
}
/**
 * 结构体
 * #[derive(Debug)] 使用prinln!()打印结构体时需要这一行
 * struct struct_name {
 *   attribute_1: type,
 *   attribute_2: type
 * }
 */
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
/**
 * 初始化实例时，每个字段都需要进行初始化
 */
fn build_user() -> User {
    User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }
}

fn print_user() -> () {
    let user = User {
        email: String::from("emial"),
        username: String::from("jack"),
        active:false,
        sign_in_count:2,
    };
    // 打印结构体
    println!("user is {:#?}", user)
}
/**
 *  当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化
 *  
 */
fn update_user() -> User {
    let user1 = User {
        email: String::from("emial"),
        username: String::from("jack"),
        active:false,
        sign_in_count:2,
    };
    // 因为 user2 仅仅在 email 上与 user1 不同，因此我们只需要对 email 进行赋值，剩下的通过结构体更新语法 ..user1 即可完成
    let user2 = User {
        email: String::from("email2"),
        // 这里user1.username的所有权被转移给user2，因为其他两个类型是基础类型，自动使用Copy而不是Move
        ..user1
    };
    // 基础类型通过Copy,可以继续使用
    println!("{}", user1.active);
    // 所有权已经转移，无法再通过user1访问
    // println!("{}", user1.username);
    return user2
}

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
/**
 * 这种是一种特征派生语法，被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能
 */
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
/**
 * &self 其实是 self: &Self 的简写（注意大小写）。
 * 在一个 impl 块内，Self 指代被实现方法的结构体类型，self 指代此类型的实例
 * 
 * self 依然有所有权的概念：
 * self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
 * &self 表示该方法对 Rectangle 的不可变借用
 * &mut self 表示可变借用
 * 
 * self 会拿走当前结构体实例(调用对象)的所有权，
 * 而 &self 却只会借用一个不可变引用，
 * &mut self 会借用一个可变引用
 */
impl Rectangle {
    // 定义在 impl 中且没有 self 的函数被称之为关联函数：因为它没有 self，
    // 不能用 f.read() 的形式调用，因此它是一个函数而不是方法，它又在impl 中，与结构体紧密关联，因此称为关联函数
    fn new_instance(w: u32, h: u32) -> Rectangle {
        Rectangle { width: w, height: h }
    }

    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }
}

/**
 * 可以看到，字符串字面量的类型是&str,这里没有显示声明，vscode自动给的提示
 * 
 */

fn my_str() {
    // 告诉编译器忽略未使用的变量，不抛出warning警告
    #![allow(unused_variables)]
    let s = "this is string slice";
    // 声明字符串
    let mut str = String::from("hello");
    str.push_str("rust");

    // 使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice)类型
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;


    // 我们可以使用 String::from 或 to_string 将 &str 转换成 String 类型

    // 新建一个字符串
    let my_str = String::new();
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
/**
 * 通过 :: 操作符来访问 枚举中的成员
 */
fn build_enum() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);
}
/**
 * 数组元素类型固定，长度固定，因此它存储在栈上
 * let array_name: [type; size] = [elements...]
 * 
 * 在实际开发中，使用最多的是数组切片[T]，我们往往通过引用的方式去使用&[T]，因为后者有固定的类型大小
 */
fn build_array() {
    let a = [1,2,3,4,5];
    // 5个元素都是3的简单写法
    let b = [3;5];
    // 创建数组时可以忽略类型，或者不指定像上面一样
    let arr: [_; 3] = ['a', 'b', 'c'];
    // char类型每个字符占4个字节
    assert!(std::mem::size_of_val(&arr) == 12);
}

/**
 * 切片是对字符串或者数组某一部分的引用
 * 对于字符串而言，切片就是对 String 类型中某一部分的引用
 * let slice_name = &s[start_index..end_index]
 * 其中开始索引是切片中第一个元素的索引位置，而终止索引是最后一个元素后面的索引位置
 */
fn build_slice() {
    let s = String::from("hello world");
    // 创建切片
    let hello = &s[0..5];
    let world = &s[6..11];
}
/**
 * 可以使用let name = if condition{}
 */
fn build_loop() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    // 无限循环,实际开发中搭配break使用
    loop {
        println!("again!");
    }
}
/**
 * 特征类似于接口
 * trait trait_name {
 *   method_name(parameters) -> return_type;
 * }
 */
pub trait Summary {
    fn summarize(&self) -> String;
    // 类似于Java接口，可以自带默认实现
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}
/**
 * 为类型实现特征
 * imple trait_name for struct_name {
 *   method_name(parameters) -> return_type {
 *      impl
 *   }
 * }
 */
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}
/**
 * 孤儿原则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
 * 
 * 防止代码不会相互破坏。有点类似于Java的双亲委派机制
 */
pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
/**
 * 特征约束.类似于传递Java中接口和泛型，是由实现了Summary特征的struct才可以
 */
pub fn notify<T: Summary>(item1: &T, item2: &T) {

}
/**
 * 多重约束.参数可以实现多个特征
 */
pub fn notify_mutli<T: Summary + Display>(item: &T) {}

/**
 * 类似于上面的声明，一旦参数多了比较麻烦，看起来也不直观
 * 可以使用where语法来对参数的特征进行限制
 */
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

}
/**
 * 返回类型为特征类型
 * fn method_name() -> impl trait_name {
 *   实现了该特征的结构体
 *   impled_trait_strut {
 *   }
 * }
 * 
 * 但是这种写法有一个限制是只能返回同一类型的结构体，返回不同类型的不允许。这点就比不了Java的接口了
 */
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}







