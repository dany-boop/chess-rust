#[derive(Debug, Clone, Copy)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
struct PieceOnSquare {
    piece: Piece,
    color: Color,
}