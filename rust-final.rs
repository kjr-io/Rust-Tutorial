#[allow(unused_variables)]
fn unique_paths(m: i32, n: i32) -> i32 {
    if m == 1 || n == 1 {
        1
    }
    else {
        if n < m {
            unique_paths(n, m);
        }
        let dpm = vec![1; m as usize];
        let mut dpn = vec![1; n as usize];
        for i in 1..dpm.len() {
            for j in 1..dpn.len() {
                dpn[j] += dpn[j-1]
            }
        }

        *dpn.last().unwrap()
    }
}

fn main() {
    let ans = unique_paths(4, 4);
    println!("{}", ans);
}