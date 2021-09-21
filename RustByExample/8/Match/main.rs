/**
 *
 * https://doc.rust-lang.org/rust-by-example/flow_control/match.html
 */

fn main() {
    let number = 13;

    println!("Tell me about {}", number);

    // 同时命中两个,以来先后顺序
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        14..=19 => println!("A teen"),
        // 不包含所有情况会报错
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
