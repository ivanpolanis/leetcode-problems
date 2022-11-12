impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut factors: Vec<i32> = vec![];
        for i in 1..(n + 1) {
            if n % i == 0 {
                factors.push(i)
            }
        }
        if factors.len() >= k as usize {
            return factors[(k - 1) as usize];
        }
        return -1;
    }
}
