use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut sum_map = HashMap::new();
        sum_map.insert(0, 1);

        for num in nums {
            sum += num;
            if let Some(&c) = sum_map.get(&(sum - k)) {
                count += c;
            }
            *sum_map.entry(sum).or_insert(0) += 1;
        }

        count
    }
}
