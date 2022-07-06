fn unique_paths(m: i32, n: i32) -> i32 {
    if m == 1 || n == 1 {
        1
    } else {
        if n < m {
            unique_paths(m, m);
        }
        m + n
    }
}

fn main() {
    let ans = unique_paths(3, 2);
    println!("{}", ans);
}