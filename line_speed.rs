fn main() {
    let line_speed = 70;
    if line_speed < 20 {
        println!("Slow");
    } else if line_speed < 100 {
        println!("Medium");
    } else {
        println!("Fast");
    }
}