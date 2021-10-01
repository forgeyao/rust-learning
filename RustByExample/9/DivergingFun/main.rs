//#![feature(never_type)]

/**
 * 发散函数没有返回
 * 用空类型 ! 标记, 不同于 uint
 * loop 返回的也是空类型
 *
 * https://doc.rust-lang.org/rust-by-example/fn/diverging.html
 */

fn some_fn() {
    ()
}

fn main() {
    let a: () = some_fn();
    println!("This function returns and you can see this line.");

    //let x: ! = panic!("This call never returns.");
    //println!("You will never see this line!");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                // continue 不返回 u32 不影响, 因为 continue 不会返回
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}
