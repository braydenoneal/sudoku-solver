mod board;
mod board2;
mod board_dfs;

fn main() {
    // let example_board = board::ExampleBoards::easy();
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
    // read_csv2("sudoku.csv").unwrap();
    // read_csv3("sudoku-3m.csv").unwrap();
}

use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut count = 1;
    let mut incorrect_counts = Vec::new();
    let mut total_duration = std::time::Duration::new(0, 0);

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

        use std::time::Instant;
        let now = Instant::now();

        board.solve();

        total_duration += now.elapsed();

        let eq = std::panic::catch_unwind(|| assert_eq!(board.cells(), example_board.solution));

        if eq.is_err() {
            incorrect_counts.push(count);
        }

        let average_duration = total_duration / count;

        println!("Count: {count}, Average: {average_duration:?} Incorrect: {incorrect_counts:?}");

        count += 1;
    }

    Ok(())
}

fn read_csv2<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut count = 1;
    let mut incorrect_counts: Vec<u32> = Vec::new();
    let mut total_duration = std::time::Duration::new(0, 0);

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

        let mut board = board2::Board::new(example_board.puzzle);

        use std::time::Instant;
        let now = Instant::now();

        board.solve();

        total_duration += now.elapsed();

        let average_duration = total_duration / count;

        let eq = std::panic::catch_unwind(|| assert_eq!(board.cells(), example_board.solution));

        if eq.is_err() {
            incorrect_counts.push(count);
        }

        println!("Count: {count}, Average: {average_duration:?} Incorrect: {incorrect_counts:?}");

        count += 1;
    }

    Ok(())
}

fn read_csv3<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut count = 1;
    let mut incorrect_counts: Vec<u32> = Vec::new();
    let mut total_duration = std::time::Duration::new(0, 0);

    for result in rdr.records() {
        let record = result?;

        // if &record[4] != "0.0" {
        //     continue;
        // }

        let mut example_board = board::ExampleBoards {
            puzzle: [[0; 9]; 9],
            solution: [[0; 9]; 9],
        };

        for i in 0..=1 {
            let board_string = &record[i + 1];

            board_string
                .chars()
                .map(|c| match c {
                    '.' => '0',
                    _ => c,
                })
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

        let mut board = board2::Board::new(example_board.puzzle);

        use std::time::Instant;
        let now = Instant::now();

        board.solve();

        total_duration += now.elapsed();

        let average_duration = total_duration / count;

        let eq = std::panic::catch_unwind(|| assert_eq!(board.cells(), example_board.solution));

        if eq.is_err() {
            incorrect_counts.push(count);
        }

        println!("Count: {count}, Average: {average_duration:?} Incorrect: {incorrect_counts:?}");

        count += 1;
    }

    Ok(())
}
