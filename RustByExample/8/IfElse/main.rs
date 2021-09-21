/**
 *
 * https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html
 */

fn main() {
    let n = 5;

    // 不需要括号
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is postive", n);
    } else {
        print!("{} is zero", n);
    }

    // if-else 也是表达式
    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        10 * n
    } else {
        println!(", and is a big number, halve the number");

        n / 2
    };

    println!("{} -> {}", n, big_n);
}
