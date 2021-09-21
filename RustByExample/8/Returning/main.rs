/**
 *
 * https://doc.rust-lang.org/rust-by-example/flow_control/loop/return.html
 */

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // 循环里返回结果
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
