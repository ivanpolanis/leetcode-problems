impl Solution {
  pub fn hamming_weight (n: u32) -> i32 {
    let mut count = 0;  
    let mut number = 0;
    for i in 0..32 {
      number = n >> i;
        if (number & 1) == 1 {count+=1}
      }
    return count;
  }
}