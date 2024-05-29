pub struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        intervals.sort_unstable_by_key(|interval| interval[0]);

        let mut result = Vec::with_capacity(intervals.len());
        let mut current_interval = intervals[0].clone();

        for interval in intervals.into_iter().skip(1) {
            if interval[0] <= current_interval[1] {
                current_interval[1] = current_interval[1].max(interval[1]);
            } else {
                result.push(std::mem::replace(&mut current_interval, interval));
            }
        }

        result.push(current_interval);
        result
    }
}
