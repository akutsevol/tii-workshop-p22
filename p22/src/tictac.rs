mod tictac {
    use std::fmt;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Player {
        X,
        Y,
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Cell {
        Empty,
        Occupied(Player),
    }

    pub struct TicTacField {
        cells: [[Cell; 3]; 3],
    }

    impl TicTacField {
        pub fn new() -> TicTacField {
            TicTacField {
                cells: [[Cell::Empty; 3]; 3],
            }
        }

        pub fn analyze(&self) -> Result<AnalyzeResult, Error> {
            // Implement the logic to analyze the field and return the appropriate result
            // For example:
            // ...
            // return Ok(AnalyzeResult::GameOn);
            unimplemented!()
        }

        pub fn make_move(&mut self, x: usize, y: usize, player: Player) -> Result<(), Error> {
            if x >= 3 || y >= 3 {
                return Err(Error::OutOfBounds);
            }
            if let Cell::Empty = self.cells[x][y] {
                self.cells[x][y] = Cell::Occupied(player);
                Ok(())
            } else {
                Err(Error::CellOccupied)
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum AnalyzeResult {
        WinX,
        WinY,
        WinBoth,
        GameOn,
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        OutOfBounds,
        CellOccupied,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::OutOfBounds => write!(f, "Out of bounds"),
                Error::CellOccupied => write!(f, "Cell already occupied"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tictac::*;

    #[test]
    fn test_make_move_success() {
        let mut field = TicTacField::new();
        assert_eq!(field.make_move(0, 0, Player::X), Ok(()));
    }

    #[test]
    fn test_make_move_out_of_bounds() {
        let mut field = TicTacField::new();
        assert_eq!(field.make_move(3, 0, Player::X), Err(Error::OutOfBounds));
    }

    #[test]
    fn test_make_move_cell_occupied() {
        let mut field = TicTacField::new();
        field.make_move(0, 0, Player::X).unwrap();
        assert_eq!(field.make_move(0, 0, Player::Y), Err(Error::CellOccupied));
    }
}