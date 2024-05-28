pub struct Solution {}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut p_count = [0; 26];
        let mut s_count = [0; 26];
        let s_bytes = s.as_bytes();
        let p_len = p.len();
        let s_len = s.len();

        if s_len < p_len {
            return ans;
        }

        for &c in p.as_bytes() {
            p_count[(c - b'a') as usize] += 1;
        }

        for i in 0..p_len {
            s_count[(s_bytes[i] - b'a') as usize] += 1;
        }

        if s_count == p_count {
            ans.push(0);
        }

        for i in p_len..s_len {
            s_count[(s_bytes[i] - b'a') as usize] += 1;
            s_count[(s_bytes[i - p_len] - b'a') as usize] -= 1;

            if s_count == p_count {
                ans.push((i - p_len + 1) as i32);
            }
        }

        ans
    }
}
