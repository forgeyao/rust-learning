/**
 * 匿名闭包类型是未知的， 需要通过 impl trait 来返回闭包
 * 必须使用 move，因为引用在函数退出时会释放
 *
 * https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html
 */

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
