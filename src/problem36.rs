pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut columns: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                let item = board[i][j];
                if item != '.' {
                    if !rows[i].contains(&item)
                        && !columns[j].contains(&item)
                        && !boxes[(i / 3) * 3 + j / 3].contains(&item)
                    {
                        rows[i].insert(item);
                        columns[j].insert(item);
                        boxes[(i / 3) * 3 + j / 3].insert(item);
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
