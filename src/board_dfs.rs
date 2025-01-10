#![warn(clippy::pedantic)]

type Cell = u8;
pub type CellBoard = [[Cell; 9]; 9];

#[derive(Default)]
pub struct Board {
    cells: CellBoard,
}

impl Board {
    fn check_number(&self, number: usize, row: usize, col: usize) -> bool {
        if self.cells[row].contains(&(number as Cell)) {
            return false;
        }

        if self
            .cells
            .iter()
            .map(|row| row[col])
            .any(|cell| cell == number as Cell)
        {
            return false;
        }

        let col = col / 3;
        let row = row / 3;

        for subgrid_row in 0..3 {
            for subgrid_col in 0..3 {
                if self.cells[row * 3 + subgrid_row][col * 3 + subgrid_col] == number as u8 {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.cells[row][col] != 0 {
                    continue;
                }

                for number in 1..=9 {
                    if !self.check_number(number, row, col) {
                        continue;
                    }

                    self.cells[row][col] = number as Cell;

                    if self.solve() {
                        return true;
                    }

                    self.cells[row][col] = 0;
                }

                return false;
            }
        }

        true
    }

    pub fn new(cells: CellBoard) -> Self {
        Self { cells }
    }

    pub fn cells(&self) -> CellBoard {
        self.cells
    }

    #[allow(dead_code)]
    pub fn print_cells(&self) {
        for row in self.cells {
            for cell in row {
                match cell {
                    0 => {
                        print!("_ ")
                    }
                    _ => {
                        print!("{cell} ")
                    }
                }
            }

            println!();
        }
    }
}
