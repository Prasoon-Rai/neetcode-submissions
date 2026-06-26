impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let string = s.to_lowercase();
        let mut value:Vec<_> = string.chars().filter(|c| !c.is_whitespace() && c.is_alphanumeric()).collect();
        let mid_index = value.len() / 2;
        let result;
        if value.len() % 2 != 0 {
            let _removed = value.remove(mid_index);
            let mut right_half = value.split_off(mid_index);
            right_half.reverse();
            let left_half = value;

            if left_half == right_half {
                result = true
            } else { result = false }
        }
        else {
            let mut right_half = value.split_off(mid_index);
            right_half.reverse();
            let left_half = value;
            if left_half == right_half { result = true } else { result =false }
        }
        result
    }
}