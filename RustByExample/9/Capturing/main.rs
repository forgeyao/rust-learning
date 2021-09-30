/**
 *
 * https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html
 */

fn main() {
    use std::mem;

    let color = String::from("green");

    // borrow color
    let print = || println!("`color`: {}", color);

    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;

    // mut borrow
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    //let _reborrow = &count;
    inc();

    // 非 copy 类型
    let movable = Box::new(3);

    // move 进闭包
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // movable 已经 drop 了，再请求会报错
    //consume();

    let haystack = vec![1, 2, 3];

    // 强制 move
    // 不带 move 就是 borrow
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // 已经 move 了
    //println!("There're {} elements in vec", haystack.len());
}
