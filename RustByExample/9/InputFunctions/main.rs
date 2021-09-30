/**
 *  闭包可以做函数参数，函数也可以
 *
 * https://doc.rust-lang.org/rust-by-example/fn/closures/input_functions.html
 */

fn call_me<F: Fn()>(f: F) {
    f();
}

fn fuction() {
    println!("I'm a fuction!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(fuction);
}
