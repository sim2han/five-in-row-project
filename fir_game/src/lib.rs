use thiserror::Error;

#[derive(Error, Debug)]
enum FirGameError {
    #[error("invalid index access in baord")]
    InvalidIndexAccessInBoard,
}

#[derive(Copy, Clone, Debug)]
struct FirBoardSize {
    pub x: usize,
    pub y: usize,
}

impl FirBoardSize {
    pub fn sqaure(n: usize) -> Self {
        FirBoardSize { x: n, y: n }
    }

    pub fn rectangle(x: usize, y: usize) -> Self {
        FirBoardSize { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
enum SqaureState {
    Empty,
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
enum Order {
    Black,
    White,
}

#[derive(Clone, Debug)]
struct FirGameState {
    size: FirBoardSize,
    board: Vec<SqaureState>,
}

impl FirGameState {
    pub fn empty_board(size: FirBoardSize) -> Self {
        FirGameState {
            size,
            board: vec![SqaureState::Empty; size.x * size.y],
        }
    }

    pub fn get_size(&self) -> FirBoardSize {
        self.size
    }

    pub fn get_square(&self, x: usize, y: usize) -> Result<SqaureState, FirGameError> {
        // -> Result<>
        if 0 <= x && x < self.size.x && 0 <= y && y <= self.size.y {
            Ok(self.board[y * self.size.x + x])
        } else {
            Err(FirGameError::InvalidIndexAccessInBoard)
        }
    }
}

/// Game Player of five in row.
struct FirGame {}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
