#[derive(Debug, Clone, Copy)]
enum Piece{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy)]
enum Color{
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
struct PieceOnSquare {
    piece: Piece,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
struct Board {
    squares: [[Option<PieceOnSquare>; 8];8],
}


fn empty_board()-> Board{
    Board{
        squares: [[None; 8]; 8],
    }
}

fn main() {
    let board= empty_board();
    println!("{:#?}", board);
}