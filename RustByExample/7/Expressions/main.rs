#[allow(unused_variables, path_statements, unused_must_use)]

/**
 *
 * https://doc.rust-lang.org/rust-by-example/expression.html
 */

fn main() {
    //变量绑定
    let x = 5;

    // 表达式
    x;
    x + 1;
    15;

    let x = 5u32;

    // Blocks 也是表达式
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // 下面表达式结果复之给 y
        x_cube + x_squared + x
    };

    let z = {
        // 带分号表达式 返回 `()`, 也即是 unit 类型
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
