/**
 * #[cfg(...)]   编译时
 * cfg!(...)  运行时
 *
 * https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
 *
 * target_os 支持的系统
 * https://doc.rust-lang.org/reference/conditional-compilation.html
 */

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
