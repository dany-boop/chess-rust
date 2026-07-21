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

fn place_white_pawns(board: &mut Board){
    for i in 0..8{
        board.squares[1][i] = Some(PieceOnSquare{
            piece: Piece::Pawn,
            color: Color::White,
        });
    }
}

fn place_black_pawns(board: &mut Board){
    for i in 0..8{
        board.squares[6][i] = Some(PieceOnSquare{
            piece: Piece::Pawn,
            color: Color::Black,
        });
    }
}

fn place_white_pieces(board: &mut Board){
    let pieces = [
        Piece::Rook,
        Piece::Knight,
        Piece::Bishop,
        Piece::Queen,
        Piece::King,
        Piece::Bishop,
        Piece::Knight,
        Piece::Rook
    ];
    for i in 0..8{
        board.squares[0][i] = Some(PieceOnSquare{
            piece: pieces[i],
            color:Color::White,
        })
    }

}



fn place_black_pieces(board: &mut Board){
    let pieces = [
        Piece::Rook,
        Piece::Knight,
        Piece::Bishop,
        Piece::Queen,
        Piece::King,
        Piece::Bishop,
        Piece::Knight,
        Piece::Rook,
    ];
    for i in 0..8{
        board.squares[0][i] = Some(PieceOnSquare{
            piece: pieces[i],
            color: Color::Black,
        });
    }
}

fn main() {
    let mut board= empty_board();
    place_white_pieces(&mut board);
    place_white_pawns(&mut board);

    place_black_pawns(&mut board);
    place_black_pieces(&mut board);
    println!("{:#?}", board);
}