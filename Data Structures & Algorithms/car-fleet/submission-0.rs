impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut times:Vec<f64> = vec![0.0; position.len()];
        let mut res:Vec<f64> = Vec::new();
        let mut cars:Vec<(i32, i32)> = (0..speed.len()).map(|x| (position[x], speed[x])).collect();
        cars.sort_by_key(|a| std::cmp::Reverse(a.0));
        for i in 0..times.len() {
            times[i] = (target - cars[i].0) as f64 / cars[i].1 as f64;
        }
        for val in times {
            if val > *res.last().unwrap_or(&0.0) {
                res.push(val);
            } else {
                continue
            }
        }
        res.len() as i32
    }
}