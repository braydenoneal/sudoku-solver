#![warn(clippy::pedantic)]

type Cell = u8;
pub type CellBoard = [[Cell; 9]; 9];

#[derive(Default)]
pub struct Board {
    cells: CellBoard,
}

impl Board {
    fn is_valid(&self, row: usize, col: usize) -> bool {
        for number in 0..9 {
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

    pub fn solve(&mut self) {
        loop {
            break;
        }
    }
}
