impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = Vec::new();
        for str in strs {
            res.push("😇".to_string());
            res.push(str.to_string());
        }
        res.concat()
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut list: Vec<String> = s.split('😇').map(String::from).collect();
        list.remove(0);
        list
    }
}