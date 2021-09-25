/**
 * 闭包特性
 * 输入和返回类型自动推断
 * 输入参数使用 || 包裹，而不是 {}
 * 闭包体 {} 是可选的
 * 补做外部环境变量的能力
 *
 * https://doc.rust-lang.org/rust-by-example/fn/closures.html
 */

fn main() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 无参数。返回类型是推断出来的 i32
    let one = || 1;
    println!("closure returning one: {}", one());
}
