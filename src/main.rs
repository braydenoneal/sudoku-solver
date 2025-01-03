type Cell = u8;
type CellBoard = [[Cell; 9]; 9];
type Note = [bool; 9];
type NoteBoard = [[Note; 9]; 9];

struct Board {
    cells: CellBoard,
    notes: NoteBoard,
}

impl Board {
    fn check_number(&self, number: usize, row: usize, col: usize) -> bool {
        for cell in self.cells[row] {
            if cell == number as u8 {
                return false;
            }
        }

        for row in self.cells {
            if row[col] == number as u8 {
                return false;
            }
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

    fn set_note(&mut self, row: usize, col: usize) {
        if self.cells[row][col] != 0 {
            return;
        }

        for number in 1..=9 {
            self.notes[row][col][number - 1] = self.check_number(number, row, col);
        }
    }

    fn set_notes(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                self.set_note(row, col);
            }
        }
    }

    fn value_if_one_remaining(&self, row: usize, col: usize) -> Option<u8> {
        let remaining: Vec<usize> = self.notes[row][col]
            .iter()
            .enumerate()
            .filter(|(_, &value)| value)
            .map(|(index, _)| index + 1)
            .collect();

        match remaining.len() {
            1 => Some(remaining[0] as u8),
            _ => None,
        }
    }

    pub fn solve(&mut self) {
        loop {
            self.set_notes();

            let mut solved = true;

            'grid: for row in 0..9 {
                for col in 0..9 {
                    if self.cells[row][col] == 0 {
                        solved = false;

                        let value_if_one_remaining = self.value_if_one_remaining(row, col);

                        match value_if_one_remaining {
                            Some(value) => {
                                self.cells[row][col] = value;

                                break 'grid;
                            }
                            None => {}
                        }
                    }
                }
            }

            if solved {
                break;
            }
        }
    }

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

    pub fn _print_notes(&self) {
        for row in self.notes {
            print!("|");

            for note in row {
                for (number, &value) in note.iter().enumerate() {
                    if value {
                        print!("{n}", n = number + 1);
                    } else {
                        print!(" ");
                    }
                }

                print!("|");
            }

            println!();
        }
    }
}

fn main() {
    let mut board = Board {
        cells: [
            [0, 9, 0, 4, 6, 7, 5, 0, 8],
            [7, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 8, 0, 0, 0, 4, 0, 9],
            [9, 6, 2, 1, 0, 0, 0, 4, 0],
            [8, 1, 0, 0, 0, 3, 0, 2, 0],
            [0, 3, 7, 6, 5, 0, 8, 0, 1],
            [5, 8, 0, 7, 0, 4, 9, 1, 3],
            [1, 0, 0, 3, 0, 0, 0, 0, 0],
            [0, 2, 4, 0, 0, 9, 6, 0, 0],
        ],
        notes: [[[false; 9]; 9]; 9],
    };

    board.print_cells();
    board.solve();
    println!();
    board.print_cells();

    let solution: CellBoard = [
        [2, 9, 1, 4, 6, 7, 5, 3, 8],
        [7, 4, 3, 8, 9, 5, 1, 6, 2],
        [6, 5, 8, 2, 3, 1, 4, 7, 9],
        [9, 6, 2, 1, 7, 8, 3, 4, 5],
        [8, 1, 5, 9, 4, 3, 7, 2, 6],
        [4, 3, 7, 6, 5, 2, 8, 9, 1],
        [5, 8, 6, 7, 2, 4, 9, 1, 3],
        [1, 7, 9, 3, 8, 6, 2, 5, 4],
        [3, 2, 4, 5, 1, 9, 6, 8, 7],
    ];

    assert_eq!(board.cells, solution);
}
