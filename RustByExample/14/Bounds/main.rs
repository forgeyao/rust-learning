// https://doc.rust-lang.org/rust-by-example/generics/bounds.html
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.lenght * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    lenght: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    lenght: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        lenght: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        lenght: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    println!("Area: {}", area(&_triangle));
}
