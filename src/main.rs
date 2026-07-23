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

#[derive(Debug, Clone, Copy)]
struct Board {
    squares: [[Option<PieceOnSquare>; 8]; 8],
}

fn empty_board() -> Board {
    Board {
        squares: [[None; 8]; 8],
    }
}

fn place_white_pawns(board: &mut Board) {
    for i in 0..8 {
        board.squares[1][i] = Some(PieceOnSquare {
            piece: Piece::Pawn,
            color: Color::White,
        });
    }
}

fn place_black_pawns(board: &mut Board) {
    for i in 0..8 {
        board.squares[6][i] = Some(PieceOnSquare {
            piece: Piece::Pawn,
            color: Color::Black,
        });
    }
}

fn place_white_pieces(board: &mut Board) {
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

fn place_black_pieces(board: &mut Board) {
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

// fn move_piece(board: &mut Board, from: (usize, usize), to: (usize, usize)){
//     let piece = board.squares[from.0][from.1];
//     board.squares[to.0][to.1] = piece;
//     board.squares[from.0][from.1] = None;
// }
fn move_piece(board: &mut Board, from: (usize, usize), to: (usize, usize)) {
    let piece = board.squares[from.0][from.1];

    board.squares[to.0][to.1] = piece;
    board.squares[from.0][from.1] = None;
}

fn pawn_moves(board: &Board, c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let (row, col) = c_pos;
    match color {
        Color::White => {
            if row < 7 && board.squares[row + 1][col].is_none() {
                moves.push((row + 1, col));
                if row == 1 && board.squares[row + 2][col].is_none() {
                    moves.push((row + 2, col));
                }
            }

            // Capture diagonally
            if row < 7 && col > 0 {
                if let Some(piece) = board.squares[row + 1][col - 1] {
                    if piece.color == Color::Black {
                        moves.push((row + 1, col - 1));
                    }
                }
            }
            if row < 7 && col < 7 {
                if let Some(piece) = board.squares[row + 1][col + 1] {
                    if piece.color == Color::Black {
                        moves.push((row + 1, col + 1));
                    }
                }
            }

            // En passant capture
            // if row == 4 && col > 0 {
            //     if let Some(piece) = board.squares[row][col - 1] {
            //         if piece.color == Color::Black {
            //             moves.push((row + 1, col - 1));
            //         }
            //     }
            // }
            // if row == 4 && col < 7 {
            //     if let Some(piece) = board.squares[row][col + 1] {
            //         if piece.color == Color::Black {
            //             moves.push((row + 1, col + 1));
            //         }
            //     }
            // }
        }
        Color::Black => {
            if row > 0 && board.squares[row - 1][col].is_none() {
                moves.push((row - 1, col));
                if row == 6 && board.squares[row - 2][col].is_none() {
                    moves.push((row - 2, col));
                }
            }
            if row > 0 && col > 0 {
                if let Some(piece) = board.squares[row - 1][col - 1] {
                    if piece.color == Color::White {
                        moves.push((row - 1, col - 1));
                    }
                }
            }
            if row > 0 && col < 7 {
                if let Some(piece) = board.squares[row - 1][col + 1] {
                    if piece.color == Color::White {
                        moves.push((row - 1, col + 1));
                    }
                }
            }
            // if row == 3 && col > 0 {
            //     if let Some(piece) = board.squares[row][col - 1] {
            //         if piece.color == Color::White {
            //             moves.push((row - 1, col - 1));
            //         }
            //     }
            // }
            // if row == 3 && col < 7 {
            //     if let Some(piece) = board.squares[row][col + 1] {
            //         if piece.color == Color::White {
            //             moves.push((row - 1, col + 1));
            //         }
            //     }
            // }
        }
    }
    moves
}

fn knight_moves(board: &Board, c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let (row, col) = c_pos;
    let offsets: [(i32, i32); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    for (dr, dc) in offsets {
        let new_row = (row as i32) + dr;
        let new_col = (col as i32) + dc;

        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
            let new_pos = (new_row as usize, new_col as usize);
            if let Some(piece) = board.squares[new_pos.0][new_pos.1] {
                if piece.color != color {
                    moves.push(new_pos);
                }
            } else {
                moves.push(new_pos);
            }
        }
    }
    moves
}

fn main() {
    let mut board = empty_board();
    place_white_pieces(&mut board);
    place_white_pawns(&mut board);

    place_black_pawns(&mut board);
    place_black_pieces(&mut board);
    let pawn_moves = pawn_moves(&board, (1, 4), Color::White);
    println!("Pawn moves: {:?}", pawn_moves);

    let knight_moves = knight_moves(&board, (0, 1), Color::White);
    println!("Knight moves: {:?}", knight_moves);

    move_piece(&mut board, (1, 4), (4, 4));
    println!("{:#?}", board);
}
