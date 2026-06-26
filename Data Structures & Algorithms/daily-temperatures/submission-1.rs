impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut output = vec![0; temperatures.len()];
        let mut stack:Vec<(i32, usize)> = Vec::new();

        for (i, t) in temperatures.iter().enumerate() {
            while let Some(&(top_t, top_i)) = stack.last() {
                if t > &top_t {
                    stack.pop();
                    output[top_i] = (i - top_i) as i32;
                } else {
                    break;
                }
            }
            stack.push((*t, i));
        }
        output
    }
}