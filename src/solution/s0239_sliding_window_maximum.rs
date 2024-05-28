use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut deque = VecDeque::new();
        let mut result = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&front) = deque.front() {
                if front + k as usize <= i {
                    deque.pop_front();
                }
            }
            while let Some(&back) = deque.back() {
                if nums[back] <= num {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);
            if i as i32 >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }
        result
    }
}
