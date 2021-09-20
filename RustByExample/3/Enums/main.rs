/**
 *
 * https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
 */

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPree(char), // tuple 风格
    Paste(String),
    Click { x: i64, y: i64 }, // C 风格 struct
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPree(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y, // Self 是别名
            Self::Subtract => x - y,
        }
    }
}

// 别名
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let pressed = WebEvent::KeyPree('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    let y = Operations::Subtract;
    println!("x: {:?}", x);
    println!("add: {}", x.run(1, 2));
    println!("sub: {}", y.run(1, 2));
}
