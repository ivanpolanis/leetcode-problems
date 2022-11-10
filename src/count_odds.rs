impl Solution {
  pub fn count_odds(low: i32, high: i32) -> i32 {
      return (high-low)/2 + (high % 2 | low % 2)
  }
}