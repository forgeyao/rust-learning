/**
 * 基本类型转换可以用 casting
 * 自定义类型直接地址转换通过  traits
 * 一般使用 From 和 Into traits
 *
 * https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
 */
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into(); // 需要指定 num 的类型, 不然编译器无法识别
    println!("My number is {:?}", num);
}
