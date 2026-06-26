impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        let mut area_glob = 0;

        let mut prefix  = height.clone();
        for j in 1..prefix.len() {
            prefix[j] = prefix[j].max(prefix[j-1]);
        }

        let mut suffix = height.clone();
        for k in (0..suffix.len() - 1).rev() {
            suffix[k] = suffix[k].max(suffix[k + 1]);
        }

        for i in 0..height.len() {
            let area = std::cmp::min(prefix[i], suffix[i]) - height[i];
            area_glob += area;
        }
        area_glob
    }
}