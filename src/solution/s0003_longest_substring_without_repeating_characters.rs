use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_index_map = HashMap::new();
        let mut max_len = 0;
        let mut start = 0;

        for (i, c) in s.chars().enumerate() {
            if let Some(&index) = char_index_map.get(&c) {
                start = start.max(index + 1);
            }
            char_index_map.insert(c, i);
            max_len = max_len.max(i - start + 1);
        }

        max_len as i32
    }
}
 