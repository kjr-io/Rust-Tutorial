#[allow(dead_code)]
struct Solution {
    m:i32,
    n:i32
}

#[allow(unused_variables)]
impl Solution {
    fn unique_paths(&self) -> i32 {
        if self.m ==1 || self.n == 1 {
            1
        } else {
            let dpm = vec![1; self.m as usize];
            let mut dpn = vec![1; self.n as usize];
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
    let ans = Solution {
        m:4,
        n:4
    };

    println!("Number of Unique Paths: {}", ans.unique_paths());
}