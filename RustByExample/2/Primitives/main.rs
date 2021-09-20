/**
 *
 * 基本数据类型
 * https://doc.rust-lang.org/rust-by-example/primitives.html
 */

fn main() {
    let logical: bool = true;
    println!("logical:{}", logical);

    let a_float: f64 = 1.0; // 常规类型注释
    let an_integer = 5i32; // 后缀注释
    println!("a_float,an_integer: {:.1} {}", a_float, an_integer);

    let default_float = 3.0; // f64
    let default_integer = 7; // i32
    println!(
        "default_float,default_integer: {:.1} {}",
        default_float, default_integer
    );

    let mut inferred_type = 12; // 自动推断 i64
    println!("inferred_type: {}", inferred_type);
    inferred_type = 4294967296i64;
    println!("inferred_type: {}", inferred_type);

    let mut mutable = 12; // i32
    println!("mutable: {}", mutable);
    mutable = 21;
    println!("mutable: {}", mutable);

    //mutable = true; // error

    // 类型覆盖
    let mutable = true;
    println!("mutable: {}", mutable);
}
