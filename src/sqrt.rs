impl Solution {
  pub fn my_sqrt(x: i32) -> i32 {
      let mut n = 0;
      while n*n <= x {
        n+=1;
      }
      return n;
  }
}