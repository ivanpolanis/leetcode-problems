impl Solution {
    pub fn remove_element(numbers: &mut Vec<i32>, val: i32) -> i32 {
        let mut act:usize = 0;
        for i in 0..numbers.len() {
            if numbers[i] != val {
                numbers[act] = numbers[i];
                current +=1;
            }
        }
        return current as i32;
    }
}
