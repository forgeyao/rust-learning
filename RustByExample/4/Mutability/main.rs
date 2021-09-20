/**
 *
 * https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
 */

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 变量默认不能修改
    //_immutable_binding += 1;
}
