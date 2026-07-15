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



fn place_white_pawns(board:  Board){
    for i in 0..8{
        board.squares[1][i] = Some(PieceOnSquare{
            piece: Piece::Pawn,
            color: Color::White,
        });
    }
}

fn empty_board()-> Board{
    Board{
        squares: [[None; 8]; 8],
    }
}
fn main() {
    let board= empty_board();
    place_white_pawns(&mut board);
    println!("{:#?}", board);
}