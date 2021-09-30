/**
 * 当定义一个闭包时, 编译器隐含创建一个匿名结构存储捕捉的变量
 * 同时 实现 Fn, FnMut, FnOnce 其中一个 traits
 *
 * https://doc.rust-lang.org/rust-by-example/fn/closures/anonymity.html
 */
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;

    let print = || println!("{}", x);

    apply(print);
}
