/**
 *
 * 为 structure 实现 fmt::Display, 每个字段都可能报错抛异常
 * 用 ? 来应对这个问题
 * https://doc.rust-lang.org/rust-by-example/hello/print/print_display/testcase_list.html
 */
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
