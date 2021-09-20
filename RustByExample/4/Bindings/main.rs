/**
 * 变量类型可以显示声明
 * 编译器也可以自动推断得出
 * https://doc.rust-lang.org/rust-by-example/variable_bindings.html
 */

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 未使用的变量, 编译器会警告
    // 用下划线开头, 编译器会忽略该告警
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}
