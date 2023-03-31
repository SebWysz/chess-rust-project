enum ChessPiece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
} 
enum Colour {
    White,
    Black,
}
// a base class for chess pieces
struct Piece {
    name: String,
    piece_typ: ChessPiece,
    position: (i32, i32),
    colour: Colour,
}
fn makepawn() -> Piece {
    Piece {
        name: String::from("Pawn"),
        piece_typ: ChessPiece::Pawn,
        position: (0, 0),
        colour: Colour::White,
    }
}
fn main () {
    // // create a new piece 
    // let piece = ChessPiece {
    //     name: String::from("Pawn"),
    //     position: (0, 0),
    //     colour: String::from("White"),
    // };
    // initialise an 8x8 chess board grid of booleans
    let arr: [[bool; 8]; 8] = [
        [true, false, true, false, true, false, true, false],
        [false, true, false, true, false, true, false, true],
        [true, false, true, false, true, false, true, false],
        [false, true, false, true, false, true, false, true],
        [true, false, true, false, true, false, true, false],
        [false, true, false, true, false, true, false, true],
        [true, false, true, false, true, false, true, false],
        [false, true, false, true, false, true, false, true],
    ];
    // print the array using X for true and false use 8x8 grid
    for row in arr.iter() {
        for col in row.iter() {
            if *col {
                print!("X  "); 
            } else {
                print!("0  ");
            }
        }
        println!();
    }
}
