pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let n = height.len();
        let mut left = 0;
        let mut right = n - 1;
        let mut left_max = height[left];
        let mut right_max = height[right];
        let mut water_trapped = 0;

        while left < right {
            if left_max < right_max {
                left += 1;
                left_max = left_max.max(height[left]);
                water_trapped += left_max - height[left];
            } else {
                right -= 1;
                right_max = right_max.max(height[right]);
                water_trapped += right_max - height[right];
            }
        }

        water_trapped
    }
}
