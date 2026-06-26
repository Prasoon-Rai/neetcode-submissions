impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars = s.into_bytes();
        let mut char_map = HashMap::new();
        let mut max = 0;
        let mut left = 0;

        for right in 0..chars.len() {
            match char_map.get(&chars[right]) {
                Some(&idx) => left = left.max(idx + 1),
                None => {}
            }
            char_map.insert(chars[right], right);
            max = std::cmp::max(max, right-left + 1)
        }
        max as i32
    }
}
