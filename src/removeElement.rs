impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        };
        if nums.len() == 1 && nums[0] == val {
            return 0;
        }

        let mut right: usize = nums.len() - 1;
        let mut left = 0;
        while left < right {
            let num = nums[right];
            if nums[left] == val && nums[left] != num {
                nums[right] = nums[left];
                nums[left] = num;
                left += 1;
                right -= 1;
            } else if nums[left] == num {
                right -= 1;
            } else {
                left += 1;
            }
        }
        return (right + 1) as i32;
    }
}
