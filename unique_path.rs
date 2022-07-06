fn unique_paths(m: i32, n: i32) -> i32 {
    if m ==1 || n == 1 {
        1
    } else {
        m + n
    }
}

fn main() {
    let ans = unique_paths(2, 3);
    println!("{}", ans);
}