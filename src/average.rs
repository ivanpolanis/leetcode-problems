impl Solution {
  pub fn average(salary: Vec<i32>) -> f64 {
      let mut average = 0;
      let mut min = i32::MAX;
      let mut max = i32::MIN;
      for i in 0..salary.len() {
        if salary[i] < min {min=salary[i]}
        if salary[i] < max {max=salary[i]}
        average+=salary[i]
      }
      return (average-min-max) as f64 / (salary.len() - 2) as f64
  }
}