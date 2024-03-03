pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        /*
        死->死：0  死细胞周围不是三个活细胞
        死->活：2  死细胞周围正好有三个活细胞
        活->死：3  活细胞周围八个位置不是两个或三个活细胞
        活->活：1  活细胞周围八个位置有两个或三个活细胞
         */
        let n = board.len();
        let m = board[0].len();
        let dx = vec![-1, -1, -1, 0, 0, 1, 1, 1];
        let dy = vec![-1, 0, 1, -1, 1, -1, 0, 1];
        for i in 0..n {
            for j in 0..m {
                let mut count = 0;
                for k in 0..8 {
                    let nx = i as i32 + dx[k];
                    let ny = j as i32 + dy[k];
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                        continue;
                    } else {
                        count += board[nx as usize][ny as usize] & 1;
                    }
                }
                match board[i][j] {
                    0 => {
                        if count == 3 {
                            board[i][j] = 2;
                        }
                    }
                    1 => {
                        if count == 2 || count == 3 {
                            board[i][j] = 3;
                        }
                    }
                    _ => return,
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                board[i][j] >>= 1;
            }
        }
        
    }
}
