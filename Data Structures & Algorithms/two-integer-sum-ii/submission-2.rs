impl Solution {
    pub fn two_sum(mut numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut i = 0;
        let mut len = numbers.len();
        while i < numbers.len() {
            if (numbers[i] + numbers[len - 1]) > target {
                len -= 1;
            }
            if (numbers[i] + numbers[len - 1]) < target {
                i += 1;
            }
            if (numbers[i] + numbers[len - 1]) == target {
                result.push((i + 1) as i32);
                result.push((len) as i32);
                break
            }
        }
        result
    }
}