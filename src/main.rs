mod board;

fn main() {
    // let example_board = board::ExampleBoards::hard();
    // example_board.print_number_of_given();
    // println!();
    //
    // let mut board = board::Board::new(example_board.puzzle);
    //
    // board.print_cells();
    // board.solve();
    // println!();
    // board.print_cells();
    //
    // assert_eq!(board.cells(), example_board.solution);

    read_csv("sudoku.csv").unwrap();
}

use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut count = 0;
    let mut incorrect_count = 0;

    for result in rdr.records() {
        let record = result?;

        let mut example_board = board::ExampleBoards {
            puzzle: [[0; 9]; 9],
            solution: [[0; 9]; 9],
        };

        for i in 0..=1 {
            let board_string = &record[i];

            board_string
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .enumerate()
                .for_each(|(index, value)| match i {
                    0 => {
                        example_board.puzzle[index / 9][index % 9] = value;
                    }
                    _ => {
                        example_board.solution[index / 9][index % 9] = value;
                    }
                });
        }

        let mut board = board::Board::new(example_board.puzzle);
        board.solve();
        let eq = std::panic::catch_unwind(|| assert_eq!(board.cells(), example_board.solution));
        if eq.is_err() {
            incorrect_count += 1;
        }

        count += 1;
        println!("Count: {count}, Incorrect: {incorrect_count}");
    }

    Ok(())
}
