/**
 * 使用过程中类型推断, 而非初始化时
 * https://doc.rust-lang.org/rust-by-example/types/inference.html
 */

fn main() {
    let elem = 5u8;

    // 创建 vec, 但这时还不知道具体类型
    let mut vec = Vec::new();

    // 此时确认 vec 类型 u8
    vec.push(elem);

    println!("{:?}", vec);
}
