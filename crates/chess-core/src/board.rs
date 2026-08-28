use crate::types::{Color, Piece, PieceOnSquare};

#[derive(Debug,Clone)]
pub struct Move{
    pub from:(usize,usize),
    pub to:(usize,usize),
    pub piece_type:String
}

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: [[Option<PieceOnSquare>; 8]; 8],
    pub history: Vec<Move>
}

pub fn empty_board() -> Board {
    Board {
        squares: [[None; 8]; 8],
        history: Vec::new()
    }
}

pub fn place_white_pawns(board: &mut Board) {
    for i in 0..8 {
        board.squares[1][i] = Some(PieceOnSquare {
            piece: Piece::Pawn,
            color: Color::White,
        });
    }
}

pub fn place_black_pawns(board: &mut Board) {
    for i in 0..8 {
        board.squares[6][i] = Some(PieceOnSquare {
            piece: Piece::Pawn,
            color: Color::Black,
        });
    }
}

pub fn place_white_pieces(board: &mut Board) {
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
    for i in 0..8 {
        board.squares[0][i] = Some(PieceOnSquare {
            piece: pieces[i],
            color: Color::White,
        });
    }
}

pub fn place_black_pieces(board: &mut Board) {
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
    for i in 0..8 {
        board.squares[7][i] = Some(PieceOnSquare {
            piece: pieces[i],
            color: Color::Black,
        });
    }
}
