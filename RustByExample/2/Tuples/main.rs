/**
 *
 * https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
 */
use std::fmt::{Display, Formatter, Result};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "({}, {})", self.0, self.1)?;
        write!(f, "({}, {})", self.2, self.3)
    }
}

fn transpose(martix: Matrix) -> Matrix {
    Matrix(martix.0, martix.2, martix.1, martix.3)
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    println!("{}, {}, {}, {}", a, b, c, d);

    let martix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", martix);
    println!("Matrix:");
    println!("{}", martix);
    println!("Transpose:");
    println!("{}", transpose(martix));
}
