/**
 *
 * https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html
 */

fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    // 离开作用域
    //println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // 覆盖
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
