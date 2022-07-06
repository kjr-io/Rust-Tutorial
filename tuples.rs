fn main() {
    let tup = (500, 6.2, 4);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    let tup2 = (1, 2, 3);

    // Access Tuple
    let one = tup2.0;
    println!("{}", one)
}