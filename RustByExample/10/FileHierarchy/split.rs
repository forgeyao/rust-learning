/**
 *
 * https://doc.rust-lang.org/rust-by-example/mod/split.html
 */
// 会找 my.rs 或 my/mod.rs
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
