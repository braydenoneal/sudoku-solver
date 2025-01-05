use crate::board::CellBoard;

#[derive(Default, Copy, Clone)]
struct Cell {
    value: u8,
    note: [bool; 9],
}

pub struct Board {
    cells: [Cell; 81],
}

impl Board {
    pub fn new(cells: CellBoard) -> Self {
        let mut new_cells: [Cell; 81] = [Cell {
            value: 0,
            note: [false; 9],
        }; 81];

        let mut index = 0;

        for row in cells {
            for cell in row {
                new_cells[index].value = cell as u8;
                index += 1;
            }
        }

        Self { cells: new_cells }
    }

    pub fn cells(&self) -> CellBoard {
        let mut cells = CellBoard::default();

        for index in 0..self.cells.len() {
            cells[index / 9][index % 9] = self.cells[index].value;
        }

        cells
    }

    fn check_number(&self, number: usize, index: usize) -> bool {
        for col_index in 0..9 {
            if self.cells[index / 9 * 9 + col_index].value == number as u8 {
                return false;
            }
        }

        for row_index in 0..9 {
            if self.cells[row_index * 9 + index % 9].value == number as u8 {
                return false;
            }
        }

        let row = index / 9 / 3;
        let col = index % 9 / 3;

        for subgrid_row in 0..3 {
            for subgrid_col in 0..3 {
                if self.cells[(row * 3 + subgrid_row) * 9 + (col * 3 + subgrid_col)].value
                    == number as u8
                {
                    return false;
                }
            }
        }

        true
    }

    fn set_notes(&mut self) {
        for index in 0..self.cells.len() {
            if self.cells[index].value != 0 {
                continue;
            }

            for number in 0..9 {
                self.cells[index].note[number] = self.check_number(number + 1, index);
            }
        }
    }

    fn value_if_one_remaining(&self) -> Option<(usize, u8)> {
        for index in 0..self.cells.len() {
            if self.cells[index].value != 0 {
                continue;
            }

            let mut count = 0;
            let mut return_number = 0;

            for number in 0..9 {
                if self.cells[index].note[number] {
                    count += 1;
                    return_number = number;
                }
            }

            if count == 1 {
                return Some((index, return_number as u8 + 1));
            }
        }

        None
    }

    fn value_if_one_in_row(&self) -> Option<(usize, u8)> {
        for number in 0..9 {
            for row in 0..9 {
                let mut count = 0;
                let mut return_index = 0;

                for col in 0..9 {
                    let index = row * 9 + col;

                    if self.cells[index].note[number] {
                        count += 1;
                        return_index = index;
                    }
                }

                if count == 1 {
                    return Some((return_index, number as u8 + 1));
                }
            }
        }

        None
    }

    fn value_if_one_in_col(&self) -> Option<(usize, u8)> {
        for number in 0..9 {
            for col in 0..9 {
                let mut count = 0;
                let mut return_index = 0;

                for row in 0..9 {
                    let index = row * 9 + col;

                    if self.cells[index].note[number] {
                        count += 1;
                        return_index = index;
                    }
                }

                if count == 1 {
                    return Some((return_index, number as u8 + 1));
                }
            }
        }

        None
    }

    fn value_if_one_in_subgrid(&self) -> Option<(usize, u8)> {
        for number in 0..9 {
            for subgrid_row in 0..3 {
                for subgrid_col in 0..3 {
                    let mut count = 0;
                    let mut return_index = 0;

                    for row in 0..3 {
                        for col in 0..3 {
                            let index = (subgrid_row * 3 + row) * 9 + (subgrid_col * 3 + col);

                            if self.cells[index].note[number] {
                                count += 1;
                                return_index = index;
                            }
                        }
                    }

                    if count == 1 {
                        return Some((return_index, number as u8 + 1));
                    }
                }
            }
        }

        None
    }

    pub fn solve(&mut self) {
        loop {
            self.set_notes();

            let remaining = self.value_if_one_remaining();

            if let Some((index, number)) = remaining {
                self.cells[index].value = number;
                self.cells[index].note = [false; 9];

                continue;
            }

            let remaining = self.value_if_one_in_row();

            if let Some((index, number)) = remaining {
                self.cells[index].value = number;
                self.cells[index].note = [false; 9];

                continue;
            }

            let remaining = self.value_if_one_in_col();

            if let Some((index, number)) = remaining {
                self.cells[index].value = number;
                self.cells[index].note = [false; 9];

                continue;
            }

            let remaining = self.value_if_one_in_subgrid();

            if let Some((index, number)) = remaining {
                self.cells[index].value = number;
                self.cells[index].note = [false; 9];

                continue;
            }

            break;
        }
    }
}
