/**
 * 注释
 * https://doc.rust-lang.org/rust-by-example/hello/comment.html
 *
 * 单行注释 `//`
 * 多行注释 `/*   */`
 * 文档注释.注释的内容可以生产文档  `///`
 *          针对封闭的项目文档注释可以用 `//!`
 */
fn main() {
    // 定义整型数 x
    let x = 5 + /* 90 + */ 5;
    /*
     * 输出结果
     */
    println!("Is `x` 10 or 100 ? x = {}", x);
}
