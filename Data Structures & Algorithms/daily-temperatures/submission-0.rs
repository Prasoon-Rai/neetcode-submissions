impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        for i in 0..temperatures.len() {

            let mut count = 0;
            let mut found = false;

            for j in (i + 1)..temperatures.len() {
                if temperatures[j] > temperatures[i] {
                    res.push(count + 1);
                    found = true;
                    break
                }
                count += 1;
            }

            if !found {
                res.push(0)
            }
        }
        res
    }
}