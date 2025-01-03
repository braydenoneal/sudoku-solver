type Cell = u8;
type CellBoard = [[Cell; 9]; 9];
type Note = [bool; 9];
type NoteBoard = [[Note; 9]; 9];

fn main() {
    let mut board: CellBoard = [
        [0, 9, 0, 4, 6, 7, 5, 0, 8],
        [7, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 8, 0, 0, 0, 4, 0, 9],
        [9, 6, 2, 1, 0, 0, 0, 4, 0],
        [8, 1, 0, 0, 0, 3, 0, 2, 0],
        [0, 3, 7, 6, 5, 0, 8, 0, 1],
        [5, 8, 0, 7, 0, 4, 9, 1, 3],
        [1, 0, 0, 3, 0, 0, 0, 0, 0],
        [0, 2, 4, 0, 0, 9, 6, 0, 0],
    ];

    let mut notes: NoteBoard = [[[false; 9]; 9]; 9];
    
    
}
