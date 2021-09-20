/**
 * 3种 struct
 * tuple struct
 * C 风格 struct
 * unit struct
 * https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
 */

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct
struct Unit;

// tuple struct
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    (rect.top_left.x - rect.bottom_right.x) * (rect.bottom_right.y - rect.top_left.y)
}

fn square(point: Point, f: f32) -> Rectangle {
    Rectangle {
        top_left: Point { x: point.x, y: f },
        bottom_right: Point { x: f, y: point.y },
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contarins {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = square(Point { x: 1.0, y: 2.0 }, 3.0);
    println!("rectangle area: {:.2}", rect_area(rect));
}
