pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_non_zero_found_at = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(last_non_zero_found_at, i);
                last_non_zero_found_at += 1;
            }
        }
    }
}
