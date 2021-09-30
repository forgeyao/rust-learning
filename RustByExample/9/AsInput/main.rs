/**
 * 闭包捕捉变量，不用说明变量使用方式
 *
 * 但闭包作为函数参数，必须指明
 *
 * Fn: &T
 * FnMut: &mut T
 * FnOnce: T
 *
 * 编译会选择最小限制的方式
 * 如果指定 FnOnce, 闭包可能使用 &T, &mut T, T
 *
 * https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html
 */

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // to_owned creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // greeting 引用, 要求 Fn
        println!("I said {}.", greeting);

        // farewell 可变引用，要求 FnMut
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // drop farewell 使得 farewell 所有权转移, 要求 FnOnce
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to3(double));
}
