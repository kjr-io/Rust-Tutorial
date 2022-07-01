#[allow(unused_assignments)]
fn main() {
    // Variable Declaration without Assignment nor Data Type
    let a;

    // Assignment
    a = 10;
    let b: i32 = 15;
    let (c, d) = (20, 30); // c = 20, d = 30

    // Mutable Variable
    let mut e = 40;
    e = 50;

    // Array with Three Data Items of i32
    let arr:[i32; 3] = [1, 2, 3];

    // Tuple
    let tup: (u8, f64, i32) = (4, 5.2, 6);

    // Change the Vriable to Print Line
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);
    println!("{:?}", arr);
    println!("{:?}", tup);

}
