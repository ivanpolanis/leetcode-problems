impl Solution {
  pub fn is_happy(n: i32) -> bool {
      let mut happy = 0;
      let mut k = n;
      while k > 0 {
        happy += i32::pow(k%10, 2);
        k /= 10;
      }
      if happy == 1 {
        return true;
      }
      if happy > 9 {
        return is_happy(happy)
      } else {
        return false
      }
  }
}