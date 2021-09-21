/**
 * 想转换任何类型成 String, 需要实现 ToString traits
 * 更简单的做法是实现 fmt::Display, 它自动提供了 ToString
 *
 * String 转换成其他类型, 可以使用 parse 函数
 * 需要指定类型协助推断或者指定解析成的类型
 *
 * 可以把 String 转换成任何实现 FromStr trait 的类型
 *
 * https://doc.rust-lang.org/rust-by-example/conversion/string.html
 */
use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap(); // 指定类型, 协助推断出解析的类型
    let tubo_parsed = "10".parse::<i32>().unwrap(); // 指定解析类型 `::<>` 语法

    let sum = parsed + tubo_parsed;
    println!("Sum: {:?}", sum);
}
