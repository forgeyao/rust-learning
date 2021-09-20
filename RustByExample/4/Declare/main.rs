/**
 * 可以先声明变量, 后初始化
 * https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html
 */

fn main() {
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a a_binding: {}", a_binding);

    let another_binding;

    // 未初始化报错
    //println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
