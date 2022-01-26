use std::fmt;
use std::io;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct LightsOutPuzzle {
    board: Vec<Vec<bool>>,
    rows: usize,
    columns: usize,
}

impl LightsOutPuzzle {
    pub fn new(rows: usize, columns: usize) -> Self {
        LightsOutPuzzle {
            board: vec![vec![false; columns]; rows],
            rows,
            columns,
        }
    }

    fn get_board(&'static self) -> &'static Vec<Vec<bool>> {
        &self.board
    }

    fn make_move(mut self, coord: &(usize, usize)) -> Result<(), LocationError> {
        if coord.0 >= self.rows || coord.1 >= self.columns {
            return Err(LocationError {
                coordinates: *coord,
                message: String::from("Bad coordinates for a move"),
            })
        }
        self.board[coord.0][coord.1] ^= self.board[coord.0][coord.1];
        if coord.0 > 0 {
            self.board[coord.0 - 1][coord.1] ^= self.board[coord.0 - 1][coord.1];
        }
        if coord.0 < self.rows - 1 {
            self.board[coord.0 + 1][coord.1] ^= self.board[coord.0 + 1][coord.1];
        }
        if coord.1 > 0 {
            self.board[coord.0][coord.1 - 1] ^= self.board[coord.0][coord.1 - 1];
        }
        if coord.1 < self.columns - 1 {
            self.board[coord.0][coord.1 + 1] ^= self.board[coord.0][coord.1 + 1];
        }
        Ok(())
    }

    fn solved(&self) -> bool {
        self.board.iter().flatten().all(|x| *x)
    }

    // fn successors(&self) -> Vec<(usize, usize)> {
    // }
}

impl fmt::Display for LightsOutPuzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::from("\n");
        for i in 0..self.rows {
            for _ in 0..self.columns - 1 { res.push('-'); }
            res.push('\n');
            for j in 0..self.columns - 1 {
                if self.board[i][j] { res.push_str("💡│"); }
                else { res.push_str("🛑│"); }
            }
            res.push('\n');
        }
        for _ in 0..self.columns - 1 { res.push('-'); }
        Ok(())
    }
}

#[derive(Debug)]
struct LocationError {
    coordinates: (usize, usize),
    message: String,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
