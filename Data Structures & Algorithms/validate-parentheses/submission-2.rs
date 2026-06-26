impl Solution {
    pub fn is_valid(s: String) -> bool {
        let val = s.into_bytes();

        if val.len() % 2 != 0 { return false }

        let mut res:Vec<u8> = Vec::new();

        for c in val {
            match c {
                40 | 123 | 91 => res.push(c),
                41 if res.last() == Some(&40) => { res.pop(); }
                125 if res.last() == Some(&123) => { res.pop(); }
                93 if res.last() == Some(&91) => { res.pop(); }
                _ => return false
            }
        }
        res.is_empty()
    }
}