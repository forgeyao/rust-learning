/**
 *
 *
 * https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
 */

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

fn main() {
    let open_box = my::OpenBox {
        contents: "public information",
    };

    println!("The open box contains: {}", open_box.contents);

    // contents 是私有的
    //let closed_box = my::ClosedBox { contents: "clasified information", };

    // 私有字段通过 pub 函数访问
    let _closed_box = my::ClosedBox::new("clasified information");
    //println!("The closed box contains: {}", _closed_box.contents);
}
