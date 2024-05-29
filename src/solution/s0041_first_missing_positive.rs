pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        let n = nums.len();
        
        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let correct_index = nums[i] as usize - 1;
                nums.swap(i, correct_index);
            }
        }
        
        for i in 0..n {
            if nums[i] != (i as i32) + 1 {
                return (i as i32) + 1;
            }
        }
        
        (n as i32) + 1
    }
}
