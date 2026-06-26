impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [[false; 10]; 9];
        let mut column = [[false; 10]; 9];
        let mut boxes = [[false; 10]; 9];

        for r in 0..9 {
            for c in 0..9 {
                let num_char = board[r][c];
                if num_char == '.' {
                    continue
                }
                let val = (num_char as u8 - b'0') as usize;

                let b = (r / 3) * 3 + (c / 3);

                if rows[r][val] || column[c][val] || boxes[b][val] {
                    return false
                }

                rows[r][val] = true;
                column[c][val] = true;
                boxes[b][val] = true;
            }
        }
        true
    }
}