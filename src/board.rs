#![warn(clippy::pedantic)]
use itertools::iproduct;

type Cell = u8;
pub type CellBoard = [[Cell; 9]; 9];
type Note = [bool; 9];
type NoteBoard = [[Note; 9]; 9];

#[derive(Default)]
pub struct Board {
    cells: CellBoard,
    notes: NoteBoard,
}

impl Board {
    pub fn new(cells: CellBoard) -> Self {
        Self {
            cells,
            notes: [[[false; 9]; 9]; 9],
        }
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

    #[allow(dead_code)]
    pub fn print_notes(&self) {
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

    pub fn solve(&mut self) {
        loop {
            self.set_notes();

            let value_if_one_remaining2 = self.value_if_one_remaining2();

            // let value_if_one_remaining = self
            //     .cells
            //     .iter()
            //     .enumerate()
            //     .flat_map(|(r, v)| v.iter().enumerate().map(move |(c, v)| (r, c, v)))
            //     .filter(|(_, _, &v)| v == 0)
            //     .find_map(|(r, c, _)| self.value_if_one_remaining(r, c));

            if let Some((row, col, value)) = value_if_one_remaining2 {
                self.cells[row][col] = value;
                self.notes[row][col] = Note::default();
                continue;
            }

            let value_if_one_in_subgrid = self.value_if_one_in_subgrid();

            if let Some((row, col, value)) = value_if_one_in_subgrid {
                self.cells[row][col] = value;
                self.notes[row][col] = Note::default();
                continue;
            }

            break;
        }
    }

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

    fn value_if_one_remaining2(&self) -> Option<(usize, usize, u8)> {
        for (row, col) in iproduct!(0..9, 0..9) {
            if self.cells[row][col] != 0 {
                continue;
            }

            let remaining = self.notes[row][col].iter().enumerate().fold(
                (0_usize, 0_usize),
                |(count, acc_index), (index, &value)| {
                    if value {
                        (count + 1, index)
                    } else {
                        (count, acc_index)
                    }
                },
            );

            if let (1, index) = remaining {
                return Some((row, col, index as u8 + 1));
            }
        }

        None
    }

    fn value_if_one_remaining(&self, row: usize, col: usize) -> Option<(usize, usize, u8)> {
        let remaining = self.notes[row][col].iter().enumerate().fold(
            (0_usize, 0_usize),
            |(count, acc_index), (index, &value)| {
                if value {
                    (count + 1, index)
                } else {
                    (count, acc_index)
                }
            },
        );

        if let (1, index) = remaining {
            return Some((row, col, index as u8 + 1));
        }

        None
    }

    fn value_if_one_in_subgrid(&self) -> Option<(usize, usize, u8)> {
        for (number, subgrid_row, subgrid_col) in iproduct!(0..9, 0..3, 0..3) {
            let (count, row, col) = iproduct!(0..3, 0..3).fold(
                (0_u8, 0_usize, 0_usize),
                |(count, acc_row, acc_col), (row, col)| {
                    if self.notes[subgrid_row * 3 + row][subgrid_col * 3 + col][number] {
                        (count + 1, subgrid_row * 3 + row, subgrid_col * 3 + col)
                    } else {
                        (count, acc_row, acc_col)
                    }
                },
            );

            if count == 1 {
                return Some((row, col, number as u8 + 1));
            }
        }

        None
    }

    // fn value_if_one_in_row(&self) -> Option<(usize, usize, u8)> {}
}

#[allow(dead_code)]
pub struct ExampleBoards {
    pub puzzle: CellBoard,
    pub solution: CellBoard,
}

#[allow(dead_code)]
impl ExampleBoards {
    pub fn new(puzzle: CellBoard, solution: CellBoard) -> Self {
        Self { puzzle, solution }
    }

    pub fn number_of_given(&self) -> usize {
        self.puzzle
            .iter()
            .flat_map(|cells| cells.iter())
            .filter(|&&number| number != 0)
            .count()
    }

    pub fn print_number_of_given(&self) {
        let given = self.number_of_given();
        println!("Number of given cells: {given}");
    }

    pub fn easy() -> Self {
        Self {
            puzzle: [
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
            solution: [
                [2, 9, 1, 4, 6, 7, 5, 3, 8],
                [7, 4, 3, 8, 9, 5, 1, 6, 2],
                [6, 5, 8, 2, 3, 1, 4, 7, 9],
                [9, 6, 2, 1, 7, 8, 3, 4, 5],
                [8, 1, 5, 9, 4, 3, 7, 2, 6],
                [4, 3, 7, 6, 5, 2, 8, 9, 1],
                [5, 8, 6, 7, 2, 4, 9, 1, 3],
                [1, 7, 9, 3, 8, 6, 2, 5, 4],
                [3, 2, 4, 5, 1, 9, 6, 8, 7],
            ],
        }
    }

    pub fn medium() -> Self {
        Self {
            puzzle: [
                [6, 0, 0, 0, 0, 3, 0, 0, 0],
                [0, 0, 2, 0, 4, 0, 1, 7, 0],
                [0, 0, 0, 0, 2, 9, 8, 0, 3],
                [4, 6, 0, 0, 0, 0, 9, 3, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 7],
                [0, 7, 3, 0, 0, 4, 0, 0, 0],
                [0, 3, 8, 4, 0, 2, 5, 1, 9],
                [9, 0, 0, 8, 0, 1, 0, 0, 0],
                [1, 2, 6, 7, 0, 5, 3, 0, 4],
            ],
            solution: [
                [6, 8, 4, 1, 7, 3, 2, 9, 5],
                [3, 9, 2, 5, 4, 8, 1, 7, 6],
                [5, 1, 7, 6, 2, 9, 8, 4, 3],
                [4, 6, 1, 2, 5, 7, 9, 3, 8],
                [8, 5, 9, 3, 1, 6, 4, 2, 7],
                [2, 7, 3, 9, 8, 4, 6, 5, 1],
                [7, 3, 8, 4, 6, 2, 5, 1, 9],
                [9, 4, 5, 8, 3, 1, 7, 6, 2],
                [1, 2, 6, 7, 9, 5, 3, 8, 4],
            ],
        }
    }

    pub fn hard() -> Self {
        Self {
            puzzle: [
                [1, 0, 0, 0, 0, 3, 2, 0, 0],
                [3, 0, 0, 0, 0, 0, 0, 5, 1],
                [0, 0, 0, 0, 1, 0, 7, 0, 0],
                [0, 1, 0, 6, 0, 2, 0, 9, 4],
                [0, 9, 0, 8, 0, 0, 0, 0, 7],
                [0, 7, 2, 0, 0, 0, 1, 0, 3],
                [0, 3, 0, 0, 0, 4, 9, 0, 0],
                [0, 8, 0, 0, 0, 0, 4, 0, 0],
                [4, 0, 9, 0, 5, 0, 0, 8, 2],
            ],
            solution: [
                [1, 5, 8, 7, 6, 3, 2, 4, 9],
                [3, 4, 7, 9, 2, 8, 6, 5, 1],
                [9, 2, 6, 4, 1, 5, 7, 3, 8],
                [5, 1, 3, 6, 7, 2, 8, 9, 4],
                [6, 9, 4, 8, 3, 1, 5, 2, 7],
                [8, 7, 2, 5, 4, 9, 1, 6, 3],
                [7, 3, 5, 2, 8, 4, 9, 1, 6],
                [2, 8, 1, 3, 9, 6, 4, 7, 5],
                [4, 6, 9, 1, 5, 7, 3, 8, 2],
            ],
        }
    }

    pub fn kaggle_fail_1() -> Self {
        Self {
            puzzle: [
                [0, 1, 0, 0, 0, 0, 0, 3, 4],
                [0, 0, 0, 0, 7, 9, 5, 0, 0],
                [0, 2, 9, 3, 0, 0, 6, 0, 0],
                [0, 3, 0, 6, 0, 7, 4, 2, 0],
                [0, 8, 6, 1, 0, 0, 0, 9, 0],
                [0, 0, 0, 0, 5, 0, 0, 0, 0],
                [9, 4, 0, 2, 0, 5, 8, 7, 0],
                [8, 0, 0, 0, 6, 0, 2, 0, 0],
                [6, 7, 0, 0, 8, 4, 0, 0, 3],
            ],
            solution: [
                [7, 1, 8, 5, 2, 6, 9, 3, 4],
                [3, 6, 4, 8, 7, 9, 5, 1, 2],
                [5, 2, 9, 3, 4, 1, 6, 8, 7],
                [1, 3, 5, 6, 9, 7, 4, 2, 8],
                [4, 8, 6, 1, 3, 2, 7, 9, 5],
                [2, 9, 7, 4, 5, 8, 3, 6, 1],
                [9, 4, 3, 2, 1, 5, 8, 7, 6],
                [8, 5, 1, 7, 6, 3, 2, 4, 9],
                [6, 7, 2, 9, 8, 4, 1, 5, 3],
            ],
        }
    }

    pub fn kaggle_fail_2() -> Self {
        Self {
            puzzle: [
                [5, 1, 0, 0, 7, 0, 0, 0, 0],
                [0, 3, 9, 0, 0, 4, 5, 0, 7],
                [0, 6, 0, 8, 0, 0, 0, 2, 1],
                [0, 7, 3, 6, 5, 9, 0, 8, 0],
                [4, 0, 0, 2, 3, 0, 9, 0, 0],
                [0, 0, 6, 0, 8, 0, 7, 0, 0],
                [0, 0, 0, 9, 1, 2, 6, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 3],
                [0, 0, 5, 0, 0, 0, 4, 9, 0],
            ],
            solution: [
                [5, 1, 2, 3, 7, 6, 8, 4, 9],
                [8, 3, 9, 1, 2, 4, 5, 6, 7],
                [7, 6, 4, 8, 9, 5, 3, 2, 1],
                [2, 7, 3, 6, 5, 9, 1, 8, 4],
                [4, 8, 1, 2, 3, 7, 9, 5, 6],
                [9, 5, 6, 4, 8, 1, 7, 3, 2],
                [3, 4, 8, 9, 1, 2, 6, 7, 5],
                [6, 9, 7, 5, 4, 8, 2, 1, 3],
                [1, 2, 5, 7, 6, 3, 4, 9, 8],
            ],
        }
    }
}
