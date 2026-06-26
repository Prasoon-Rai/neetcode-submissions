impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut res:Vec<_> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let second = res.pop().unwrap();
                    let first = res.pop().unwrap();
                    let temp = first + second;
                    res.push(temp);
                }
                "-" => {
                    let second = res.pop().unwrap();
                    let first = res.pop().unwrap();
                    let temp = first - second;
                    res.push(temp);
                }
                "*" => {
                    let second = res.pop().unwrap();
                    let first = res.pop().unwrap();
                    let temp = first * second;
                    res.push(temp);
                }
                "/" => {
                    let second = res.pop().unwrap();
                    let first = res.pop().unwrap();
                    let temp = first / second;
                    res.push(temp);
                }
                _ => res.push(token.parse().unwrap())
            }
        }
        res.iter().sum()
    }
}