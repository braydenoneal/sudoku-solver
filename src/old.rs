struct Board {
    board: [[[bool; 10]; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[[true; 10]; 9]; 9],
        }
    }

    fn mark_all_unavailable(&mut self, column: usize, row: usize) {
        self.board[column][row] = [false; 10];
    }

    fn mark_unavailable(&mut self, column: usize, row: usize, index: usize) {
        self.board[column][row][index] = false;
    }

    fn mark_column_unavailable(&mut self, column: usize, index: usize) {
        for i in 0..9 {
            self.mark_unavailable(column, i, index);
        }
    }

    fn mark_row_unavailable(&mut self, row: usize, index: usize) {
        for i in 0..9 {
            self.mark_unavailable(i, row, index);
        }
    }

    fn mark_cell_unavailable(&mut self, column: usize, row: usize, index: usize) {
        let column = column / 3;
        let row = row / 3;

        for c in 0..3 {
            for r in 0..3 {
                self.mark_unavailable(column * 3 + c, row * 3 + r, index);
            }
        }
    }

    fn get_random_available_index(&self, column: usize, row: usize) -> Option<usize> {
        use rand::seq::SliceRandom;

        let available_indices = self.board[column][row]
            .iter()
            .enumerate()
            .filter(|&(_, &available)| available)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        let index_option = available_indices.choose(&mut rand::thread_rng());

        match index_option {
            None => None,
            Some(&index) => Some(index),
        }
    }

    fn get_random_cell(&self) -> (usize, usize) {
        use rand::Rng;

        (
            rand::thread_rng().gen_range(0..9),
            rand::thread_rng().gen_range(0..9),
        )
    }

    fn set_random_cell(&mut self) {
        let (column, row) = self.get_random_cell();

        let index = self.get_random_available_index(column, row);

        match index {
            None => {
                return;
            }
            _ => {}
        }

        self.mark_all_unavailable(column, row);
        self.mark_column_unavailable(column, index.unwrap());
        self.mark_row_unavailable(row, index.unwrap());
        self.mark_cell_unavailable(column, row, index.unwrap());
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (column_index, column) in self.board.iter().enumerate() {
            if column_index % 3 == 0 {
                writeln!(f)?;
            }

            write!(f, "|")?;

            for (row_index, row) in column.iter().enumerate() {
                if row_index % 3 == 0 {
                    write!(f, " ")?;
                }

                for (index, &number) in row.iter().enumerate() {
                    if number {
                        write!(f, "{index}")?;
                    } else {
                        write!(f, " ")?;
                    }
                }

                write!(f, "|")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let mut board = Board::new();
    board.set_random_cell();
    // for _ in 0..1000 {
    //     board.set_random_cell();
    // }
    print!("{board}");
}
