use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen = HashSet::new();

        for (i, row) in board.iter().enumerate() {          
            for (j, col) in row.iter().copied().enumerate() {
                if col == '.' {
                    continue;
                }

                let row_key = format!("row{}_val{}", i, col);
                let col_key = format!("col{}_val{}", j, col);
                let box_key = format!("box{}-{}_val{}", i / 3, j / 3, col);

                if !seen.insert(row_key) || !seen.insert(col_key) || !seen.insert(box_key) {
                    return false;
                }
            }
            
        }

        true
    }
}
