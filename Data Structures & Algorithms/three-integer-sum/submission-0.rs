impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort();
        for i in 0..nums.len() {
            let mut first = i + 1;
            let mut last = nums.len() - 1;

            while first < last {
                let total = nums[i] + nums[first] + nums[last];

                if total > 0 {
                    last -= 1;
                }
                else if total < 0 {
                    first += 1;
                }
                else {
                    result.push(vec![nums[i], nums[first], nums[last]]);
                    last -= 1;
                    first += 1;
                }
            }
        }
        result.sort_unstable();
        result.dedup();
        result
    }
}