/**
 * 各种 print
 * https://doc.rust-lang.org/rust-by-example/hello/print.html
 *
 * format!  输出格式化内容到 String
 * print!   同 format!。不同在于输出到的是控制台(io::stdout)
 * println! 同 print!。不同在于输出结尾包含换行
 * eprint!  同 format!。不同在于输出标准错误(io::stderr)
 * eprintln!同 eprint!。不同在于输出结尾包含换行
 *
 * fmt::Debug: 使用 {:?}. 用于 debug 目的.
 * fmt::Display: 使用 {} . 更友好的格式化输出.
 */

fn main() {
    // {} 会自动替换成后面的参数
    println!("{} days", 31);

    // 位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以在 `:` 后指定特殊格式
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // 指定输出长度 width, 右对齐, 多余部分空格填充
    println!("{number:>width$}", number = 1, width = 6);

    // 多余部分指定填充
    println!("{number:0>width$}", number = 1, width = 6);

    // Rust 会检查参数个数,所以这里会报错
    //println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // 自定义类型更复杂,不能直接用 `{}` 输出
    //println!("This struct `{}` won't print...", Structure(3));
    // {:?}, 同时结构体定义上面加上 #[derive(Debug)]
    println!("This struct `{:?}` won't print...", Structure(3));

    // 控制浮点数位数输出
    // https://doc.rust-lang.org/std/fmt/#precision
    println!("Pi is roughly {pi:.*}", 3, pi = 3.141592);
}
