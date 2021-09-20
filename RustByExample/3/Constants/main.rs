/**
 * 有两种常量
 * const: 不可以修改
 * static: 可以是 mut, 'static 生命周期
 * https://doc.rust-lang.org/rust-by-example/custom_types/constants.html
 */

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 常量不能修改
    //THRESHOLD = 5;
}
