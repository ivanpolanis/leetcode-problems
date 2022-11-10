impl Solution {
  pub fn subtract_product_and_sum(n: i32) -> i32 {
      let mut prod = 1;
      let mut sum = 0;
      let mut number = n;
      while number > 0 {
        let digit = number%10;
        prod*=digit;
        sum+=digit;
        number/=10;
      }
      return prod-sum;
  }
}