#[allow(unreachable_code, unused_labels)]

/**
 * 通过标签, 可以在内层循环跳出外层循环. 这个还是挺有用的
 *
 * https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html
 */

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 跳出 inner loop
            //break;

            // 跳出 outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
