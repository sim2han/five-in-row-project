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

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum SqaureState {
    #[default]
    Empty,
    Black,
    White,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Order {
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

    pub fn play(&mut self, x: u32, y: u32, order: Order) -> Result<Response, error::FirError> {
        if order != self.order {
            return Ok(Response::OnGoing);
        }
        self.state.set_square(x as usize, y as usize, self.order)?;
        self.order = match self.order {
            Order::Black => Order::White,
            Order::White => Order::Black,
        };
        Ok(Response::OnGoing)
    }

    pub fn board_state(&self) -> String {
        let mut str = String::new();
        let size = self.state.get_size();
        for i in 0..size.x {
            str.push('\n');
            for j in 0..size.y {
                let can = match self.state.get_square(i, j).unwrap() {
                    SqaureState::Empty => '*',
                    SqaureState::Black => 'X',
                    SqaureState::White => '0',
                };
                str.push(can);
            }
        }
        str
    }

    pub fn is_end(&self) -> (bool, Order) {
        for i in 0..8 {
            for j in 0..3 {
                let mut wstate = true;
                let mut bstate = true;
                for k in 0..5 {
                    if self.state.get_square(i, j + k).unwrap() != SqaureState::White {
                        wstate = false;
                    }
                    if self.state.get_square(i, j + k).unwrap() != SqaureState::Black {
                        bstate = false;
                    }
                }
                if wstate {
                    return (true, Order::White);
                } else if bstate {
                    return (true, Order::Black);
                }
            }
        }

        for i in 0..3 {
            for j in 0..8 {
                let mut wstate = true;
                let mut bstate = true;
                for k in 0..5 {
                    if self.state.get_square(i + k, j).unwrap() != SqaureState::White {
                        wstate = false;
                    }
                    if self.state.get_square(i + k, j).unwrap() != SqaureState::Black {
                        bstate = false;
                    }
                }
                if wstate {
                    return (true, Order::White);
                } else if bstate {
                    return (true, Order::Black);
                }
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut wstate = true;
                let mut bstate = true;
                for k in 0..5 {
                    if self.state.get_square(i + k, j + k).unwrap() != SqaureState::White {
                        wstate = false;
                    }
                    if self.state.get_square(i + k, j + k).unwrap() != SqaureState::Black {
                        bstate = false;
                    }
                }
                if wstate {
                    return (true, Order::White);
                } else if bstate {
                    return (true, Order::Black);
                }
            }
        }

        for i in 5..8 {
            for j in 0..3 {
                let mut wstate = true;
                let mut bstate = true;
                for k in 0..5 {
                    if self.state.get_square(i - k, j + k).unwrap() != SqaureState::White {
                        wstate = false;
                    }
                    if self.state.get_square(i - k, j + k).unwrap() != SqaureState::Black {
                        bstate = false;
                    }
                }
                if wstate {
                    return (true, Order::White);
                } else if bstate {
                    return (true, Order::Black);
                }
            }
        }

        (false, Order::White)
    }
}
