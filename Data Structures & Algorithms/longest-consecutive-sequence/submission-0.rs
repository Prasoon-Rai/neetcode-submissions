impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

        if nums.is_empty() { return 0 }

        let set:HashSet<i32> = nums.into_iter().collect();
        let mut count = 0;

        for num in &set {
            if !set.contains(&(num - 1)) {
                let mut n = num + 1;
                let mut c = 1;
                while set.contains(&n) {
                    n += 1;
                    c += 1
                }
                count = std::cmp::max(c, count);
            }
        }
        count
    }
}