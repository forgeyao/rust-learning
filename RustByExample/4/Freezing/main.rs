/**
 *
 * https://doc.rust-lang.org/rust-by-example/variable_bindings/freeze.html
 */

fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // 外层 _mutable_integer 被冻结不能修改 内部 _mutable_integer 是不可修改的
        //_mutable_integer = 50;
    }

    _mutable_integer = 3;
}
