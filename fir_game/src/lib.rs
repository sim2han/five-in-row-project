use std::io::Empty;

mod prelude {}

mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum FirError {
        #[error("invalid index access in baord")]
        InvalidIndexAccessInBoard,
        #[error("target square already used")]
        TargetSquareAlreadyUsed,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone, Debug)]
struct FirBoardSize {
    pub x: usize,
    pub y: usize,
}

impl FirBoardSize {
    pub fn sqaure(n: usize) -> Self {
        FirBoardSize::rectangle(n, n)
    }

    pub fn rectangle(x: usize, y: usize) -> Self {
        assert!(x > 0 && y > 0);
        FirBoardSize { x, y }
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum SqaureState {
    #[default]
    Empty,
    Black,
    White,
}

#[derive(Copy, Clone, Debug, Default)]
enum Order {
    #[default]
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

    pub fn get_square(&self, x: usize, y: usize) -> Result<SqaureState, error::FirError> {
        // -> Result<>
        if x < self.size.x && y <= self.size.y {
            Ok(self.board[y * self.size.x + x])
        } else {
            Err(error::FirError::InvalidIndexAccessInBoard)
        }
    }

    fn to1d(&self, x: usize, y: usize) -> usize {
        self.size.x * y + x
    }

    pub fn set_square(&mut self, x: usize, y: usize, order: Order) -> Result<(), error::FirError> {
        if x < self.size.x && y <= self.size.y {
            let s = self.board[self.to1d(x, y)];
            if let SqaureState::Empty = s {
                let idx = self.to1d(x, y);
                self.board[idx] = match order {
                    Order::Black => SqaureState::Black,
                    Order::White => SqaureState::White,
                };
                Ok(())
            } else {
                Err(error::FirError::TargetSquareAlreadyUsed)
            }
        } else {
            Err(error::FirError::InvalidIndexAccessInBoard)
        }
    }
}

/// Game
#[derive(Debug)]
pub struct FirGame {
    state: FirGameState,
    order: Order,
}

#[derive(Default)]
pub enum Response {
    #[default]
    OnGoing,
    WhiteWin,
    BlackWin,
    Draw,
}

impl FirGame {
    pub fn new() -> Self {
        FirGame {
            state: FirGameState::empty_board(FirBoardSize::sqaure(8)),
            order: Order::Black,
        }
    }

    pub fn play(&mut self, x: usize, y: usize) -> Result<Response, error::FirError> {
        self.state.set_square(x, y, self.order)?;
        Ok(Response::OnGoing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
