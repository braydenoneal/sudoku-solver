mod board;

fn main() {
    let example_board = board::ExampleBoards::easy();
    example_board.print_number_of_given();

    let mut board = board::Board::new(example_board.puzzle);

    board.print_cells();
    board.solve();
    println!();
    board.print_cells();

    assert_eq!(board.get_cells(), example_board.solution);
}
