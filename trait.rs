#[allow(dead_code)]
struct Solution {
    m: i32,
    n: i32
}

trait UniquePaths {
    fn unique_paths(m: i32, n: i32) -> i32;
}

#[allow(unused_variables)]
impl UniquePaths for Solution {
    fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            1
        } else {
            let dpm = vec![1; m as usize];
            let mut dpn = vec![1; n as usize];
            for i in 1..dpm.len() {
                for j in 1..dpn.len() {
                    dpn[j] += dpn[j-1];
                }
            }
            *dpn.last().unwrap()
        }   
    }
}

fn main() {
    let ex = Solution::unique_paths(2, 3);

    println!("Number of Unique Paths: {}", ex)
}