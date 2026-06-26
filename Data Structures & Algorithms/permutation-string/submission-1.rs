impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n1 = s1.len();
        let n2 = s2.len();
        if n1 > n2 { return false; }

        // Helper to convert a byte character to an index 0-25
        let to_idx = |b: u8| (b - b'a') as usize;

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        let mut s1_counts = [0; 26];
        let mut s2_counts = [0; 26];

        for i in 0..n1 {
            s1_counts[to_idx(s1_bytes[i])] += 1;
            s2_counts[to_idx(s2_bytes[i])] += 1;
        }

        if s1_counts == s2_counts { return true; }

        for i in n1..n2 {
            // Look how much cleaner this is to read now:
            s2_counts[to_idx(s2_bytes[i])] += 1;
            s2_counts[to_idx(s2_bytes[i - n1])] -= 1;

            if s1_counts == s2_counts { return true; }
        }

        false
    }
}