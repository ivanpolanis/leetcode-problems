impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            for j in (i+1)..numbers.len(){
                if numbers[i] + numbers[j] == target {
                  return vec![i as i32,j as i32]
              }
            }
        }
        return vec![];
    }
  }