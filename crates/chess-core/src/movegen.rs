use crate::board::{Board, Move};
use crate::types::{Color, Piece};

pub fn move_piece(board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Option<(Move)> {
    // let piece = board.squares[from.0][from.1];
    let piece:Piece = match board.squares[from.0][from.1] {
        Some(square_data) => square_data.piece,
        None => return None, // No piece to move
    };

    let (from_row, from_col) = from;
    let (to_row, to_col) = to;

    // let piece_name = match &board.squares[from_row][from_col] {
    //     Some(square_data) => match square_data.piece {
    //         crate::types::Piece::Pawn => "Pawn".to_string(),
    //         crate::types::Piece::Rook => "Rook".to_string(),
    //         crate::types::Piece::Knight => "Knight".to_string(),
    //         crate::types::Piece::Bishop => "Bishop".to_string(),
    //         crate::types::Piece::Queen => "Queen".to_string(),
    //         crate::types::Piece::King => "King".to_string(),
    //     },
    //     None => return None,
    // };
    
    let completed_move = Move {
        from,
        to,
        piece,
    };

    // board.history.push(completed_move);

    let piece = board.squares[from_row][from_col].take();
    board.squares[to_row][to_col] = piece;

    // board.squares[to.0][to.1] = piece;
    // board.squares[from.0][from.1] = None;
    Some(completed_move)
}

pub fn pawn_moves(board: &Board, c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
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

            // // En passant capture
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
            if row == 3 && col > 0 {
                if let Some(piece) = board.squares[row][col - 1] {
                    if piece.color == Color::White {
                        moves.push((row - 1, col - 1));
                    }
                }
            }
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

pub fn pawn_attacks(c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
    let mut attacks = Vec::new();
    let (row, col) = c_pos;
    match color {
        Color::White => {
            if row < 7 && col > 0 {
                attacks.push((row + 1, col - 1));
            }
            if row < 7 && col < 7 {
                attacks.push((row + 1, col + 1));
            }
        }
        Color::Black => {
            if row > 0 && col > 0 {
                attacks.push((row - 1, col - 1));
            }
            if row > 0 && col < 7 {
                attacks.push((row - 1, col + 1));
            }
        }
    }
    attacks
}

pub fn knight_moves(board: &Board, c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
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

pub fn king_moves(board: &Board, c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let (row, col) = c_pos;
    let directions = [
        (1, 0),
        (1, 1),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];

    for (dr, dc) in directions {
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

pub fn bishop_moves(board: &Board, c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let (row, col) = c_pos;
    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for (dr, dc) in directions.iter() {
        let mut new_row = row as i32 + dr;
        let mut new_col = col as i32 + dc;

        while new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
            let new_pos = (new_row as usize, new_col as usize);
            if let Some(piece) = board.squares[new_pos.0][new_pos.1] {
                if piece.color != color {
                    moves.push(new_pos);
                }
                break;
            } else {
                moves.push(new_pos);
            }
            new_row += dr;
            new_col += dc;
        }
    }
    moves
}

pub fn rook_moves(board: &Board, c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let (row, col) = c_pos;
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (dr, dc) in directions.iter() {
        let mut new_row = row as i32 + dr;
        let mut new_col = col as i32 + dc;

        while new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
            let new_pos = (new_row as usize, new_col as usize);
            if let Some(piece) = board.squares[new_pos.0][new_pos.1] {
                if piece.color != color {
                    moves.push(new_pos);
                }
                break;
            } else {
                moves.push(new_pos);
            }
            new_row += dr;
            new_col += dc;
        }
    }
    moves
}

pub fn queen_moves(board: &Board, c_pos: (usize, usize), color: Color) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    moves.extend(bishop_moves(board, c_pos, color));
    moves.extend(rook_moves(board, c_pos, color));
    moves
}

pub fn is_square_attacked(board: &Board, square: (usize, usize), color: Color) -> bool {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board.squares[row][col] {
                if piece.color == color {
                    let moves = match piece.piece {
                        // Piece::Pawn => pawn_moves(board, (row, col), color),
                        Piece::Pawn => pawn_attacks((row, col), piece.color),
                        Piece::Knight => knight_moves(board, (row, col), piece.color),
                        Piece::Bishop => bishop_moves(board, (row, col), piece.color),
                        Piece::Rook => rook_moves(board, (row, col), piece.color),
                        Piece::Queen => queen_moves(board, (row, col), piece.color),
                        Piece::King => king_moves(board, (row, col), piece.color),
                    };
                    if moves.contains(&square) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn find_king(board: &Board, color: Color) -> Option<(usize, usize)> {
    for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board.squares[r][c] {
                if p.piece == Piece::King && p.color == color {
                    return Some((r, c));
                }
            }
        }
    }
    None
}

pub fn legal_moves_for(board: &Board, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let Some(piece_on_square) = board.squares[pos.0][pos.1] else {
        return Vec::new();
    };

    let moving_color = piece_on_square.color;
    let enemy_color = match moving_color {
        Color::Black => Color::White,
        Color::White => Color::Black,
    };

    let pseudo_moves = match piece_on_square.piece {
        Piece::Pawn => pawn_moves(board, pos, moving_color),
        Piece::Knight => knight_moves(board, pos, moving_color),
        Piece::Bishop => bishop_moves(board, pos, moving_color),
        Piece::Rook => rook_moves(board, pos, moving_color),
        Piece::Queen => queen_moves(board, pos, moving_color),
        Piece::King => king_moves(board, pos, moving_color),
    };

    pseudo_moves
        .into_iter()
        .filter(|&target_pos| {
            let mut temp_board = board.clone();

            let piece = temp_board.squares[pos.0][pos.1].take();
            temp_board.squares[target_pos.0][target_pos.1] = piece;

            let king_pos = if piece_on_square.piece == Piece::King {
                Some(target_pos)
            } else {
                find_king(&temp_board, moving_color)
            };
            // !is_square_attacked(&temp_board, king_pos, enemy_color)
            match king_pos {
                Some(k) => !is_square_attacked(&temp_board, k, enemy_color),
                None => true,
            }
        })
        .collect::<Vec<_>>()
}
