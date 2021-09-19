/**
 *
 * std 库里的类型已经实现 print trait, 其他类型需要手动实现
 * 可以直接 derive fmt::Debug trait
 * fmt::Debug 输出标记 {:?}, 更好看的 {:#?}
 * https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
 */

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Perter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);
    // {:#?} }Pretty print
    println!("{:#?}", peter);
}
