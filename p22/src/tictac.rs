use std::fmt;

// Define Player enum
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    X,
    Y,
}

// Define Error enum
#[derive(Debug, PartialEq)]
pub enum TicTacError {
    InvalidMove,
    OutOfBounds,
}

// Define TicTacField struct
pub struct TicTacField {
    board: [[Option<Player>; 3]; 3],
}

impl TicTacField {
    // Function to create a new empty field
    pub fn new() -> Self {
        TicTacField {
            board: [[None; 3]; 3],
        }
    }

    // Function to make a move
    pub fn make_move(&mut self, x: usize, y: usize, player: Player) -> Result<(), TicTacError> {
        if x >= 3 || y >= 3 {
            return Err(TicTacError::OutOfBounds);
        }
        if self.board[x][y].is_some() {
            return Err(TicTacError::InvalidMove);
        }
        self.board[x][y] = Some(player);
        Ok(())
    }

    // Function to analyze the field
    pub fn analyze(&self) -> Result<GameState, TicTacError> {
        // Check rows
        for row in 0..3 {
            if self.board[row][0] == self.board[row][1]
                && self.board[row][1] == self.board[row][2]
            {
                if let Some(player) = self.board[row][0] {
                    return Ok(GameState::Win(player));
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] == self.board[1][col]
                && self.board[1][col] == self.board[2][col]
            {
                if let Some(player) = self.board[0][col] {
                    return Ok(GameState::Win(player));
                }
            }
        }

        // Check diagonals
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            if let Some(player) = self.board[0][0] {
                return Ok(GameState::Win(player));
            }
        }
        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            if let Some(player) = self.board[0][2] {
                return Ok(GameState::Win(player));
            }
        }

        // Check for draw
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col].is_none() {
                    return Ok(GameState::GameOn);
                }
            }
        }

        // If no winner and no draw, the game is on
        Ok(GameState::GameOn)
    }
}

// Define GameState enum
#[derive(Debug, PartialEq)]
pub enum GameState {
    Win(Player),
    GameOn,
}

// Implement Display trait for GameState enum
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::Win(player) => write!(f, "Player {:?} wins!", player),
            GameState::GameOn => write!(f, "Game On!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_move() {
        let mut field = TicTacField::new();
        assert_eq!(field.make_move(0, 0, Player::X), Ok(()));
        assert_eq!(field.make_move(0, 0, Player::Y), Err(TicTacError::InvalidMove));
        assert_eq!(field.make_move(3, 0, Player::X), Err(TicTacError::OutOfBounds));
    }

    #[test]
    fn test_analyze() {
        let mut field = TicTacField::new();
        assert_eq!(field.analyze(), Ok(GameState::GameOn));
        field.make_move(0, 0, Player::X).unwrap();
        field.make_move(1, 0, Player::X).unwrap();
        field.make_move(2, 0, Player::X).unwrap();
        assert_eq!(field.analyze(), Ok(GameState::Win(Player::X)));
    }
}
