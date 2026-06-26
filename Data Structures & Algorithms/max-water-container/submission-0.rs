impl Solution {
    pub fn max_area(mut heights: Vec<i32>) -> i32 {
        let mut global_max = 0;
        let mut left = 0usize;
        let mut right = heights.len() - 1;

        while left < right {
            let height = std::cmp::min(heights[left], heights[right]);
            let distance = (right - left) as i32;
            let local_area= height * distance;
            global_max = std::cmp::max(local_area, global_max);
            if heights[left] < heights[right] {
                left += 1
            } else {
                right -= 1
            }
        }
        global_max
    }
}