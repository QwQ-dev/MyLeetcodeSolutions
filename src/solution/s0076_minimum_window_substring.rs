pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut s_cnt = vec![0; 128];
        let mut t_cnt = vec![0; 128];

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        for &byte in t_bytes {
            t_cnt[byte as usize] += 1;
        }

        let mut left = 0;
        let mut distance = 0;
        let t_len = t.len();
        let mut ans_left = 0;
        let mut ans_right = s_bytes.len() + 1;

        for (right, &byte) in s_bytes.iter().enumerate() {
            let byte = byte as usize;
            if s_cnt[byte] < t_cnt[byte] {
                distance += 1;
            }
            s_cnt[byte] += 1;
            while distance == t_len {
                if right - left < ans_right - ans_left {
                    ans_left = left;
                    ans_right = right;
                }
                let left_byte = s_bytes[left] as usize;
                if s_cnt[left_byte] == t_cnt[left_byte] {
                    distance -= 1;
                }
                s_cnt[left_byte] -= 1;
                left += 1;
            }
        }

        if ans_right == s_bytes.len() + 1 {
            String::new()
        } else {
            String::from_utf8_lossy(&s_bytes[ans_left..=ans_right]).to_string()
        }
    }
}
