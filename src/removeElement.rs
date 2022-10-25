impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut act:usize = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[act] = nums[i];
                curr +=1;
            }
        }
        return curr as i32;
    }
}
