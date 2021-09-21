/**
 * loop 无限循环(死循环)
 *
 * https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
 */

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }
}
