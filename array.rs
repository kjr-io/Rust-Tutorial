fn main() {
    // Create an Array
    let a = [1, 2, 3];

    // Create Array with Types
    let b:[u16; 3] = [4, 5, 6];

    // Access an Array
    let one = a[0]; // 1

    let c = [1;5];
    // This is the Same As
    let d = [1, 1, 1, 1, 1];

    println!("one: {}, b: {:?}, c: {:?}, d: {:?}", one, b, c, d);
}