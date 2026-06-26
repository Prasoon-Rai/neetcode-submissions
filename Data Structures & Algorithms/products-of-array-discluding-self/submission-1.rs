impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..nums.len() {
            let mut temp = nums.clone();
            temp.remove(i);
            result.push(temp.iter().product());
        }
        result
    }
}