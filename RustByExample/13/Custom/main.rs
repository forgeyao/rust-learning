/**
 *
 * https://doc.rust-lang.org/rust-by-example/attribute/cfg/custom.html
 */

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
