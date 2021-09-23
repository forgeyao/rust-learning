#![allow(irrefutable_let_patterns)]

/**
 *
 * https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
 */

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    // match 必须包含所有情况
    // if let 可以只包含需要的情况,更简洁
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emotion {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }

    // if let 用于 enum
    {
        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        if let Foo::Bar = a {
            println!("a is foobar");
        }

        if let Foo::Bar = b {
            println!("b is foobar");
        }

        if let Foo::Qux(value) = c {
            println!("c is {}", value);
        }

        if let Foo::Qux(_value @ 100) = c {
            println!("c is one hundred");
        }
    }

    // if let 与 if 差别
    {
        enum Foo {
            Bar,
        }

        let a = Foo::Bar;

        // 需要实现 std::cmp::PartialEq 才能使用 if
        //if Foo::Bar == a {
        if let Foo::Bar = a {
            println!("a is foobar");
        }
    }
}
