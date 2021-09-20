// 屏蔽没用到类型  warning
// 必须放在第一行,不能放到文档注释前
#![allow(dead_code)]

/**
 *
 * https://doc.rust-lang.org/rust-by-example/custom_types/enum/enum_use.html
 */

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldier fight!"),
    }
}
