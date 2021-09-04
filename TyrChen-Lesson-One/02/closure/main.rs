// 闭包对上下文环境捕获
fn main() {
    let a = "Hello";
    let b = "Tyr";

    let c = |msg: &str| {
        println!("{} {}: {}", a, b, msg);
    };

    c("How are you");
}
