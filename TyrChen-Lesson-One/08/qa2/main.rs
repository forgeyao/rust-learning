/**
 *
 * 如何修改才能使其编译通过，避免同时有只读引用和可变引用？
 */
fn main() {
    let mut arr = vec![1, 2, 3];
    let last = arr.last(); // immutable borrow
    println!("last: {:?}", last); // immutable borrow
    arr.push(4); // mut borrow
                 //println!("last: {:?}", last); // immutable borrow
}
