/**
 * https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 * 单元测试例子
 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
