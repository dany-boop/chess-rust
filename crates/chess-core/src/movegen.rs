use crate::board::Board;
use crate::types::{Color, Piece};

pub fn move_piece(board: &mut Board, from: (usize, usize), to: (usize, usize)) {
    let piece = board.squares[from.0][from.1];

    board.squares[to.0][to.1] = piece;
    board.squares[from.0][from.1] = None;
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

            // En passant capture
            if row == 4 && col > 0 {
                if let Some(piece) = board.squares[row][col - 1] {
                    if piece.color == Color::Black {
                        moves.push((row + 1, col - 1));
                    }
                }
            }
            if row == 4 && col < 7 {
                if let Some(piece) = board.squares[row][col + 1] {
                    if piece.color == Color::Black {
                        moves.push((row + 1, col + 1));
                    }
                }
            }
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
            if row == 3 && col < 7 {
                if let Some(piece) = board.squares[row][col + 1] {
                    if piece.color == Color::White {
                        moves.push((row - 1, col + 1));
                    }
                }
            }
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

pub fn king_moves(board: &Board, c_pos:(usize, usize), color: Color) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let (row, col) = c_pos;
    let directions = [
        (1, 0),  (1, 1),  (1, -1),
        (0, 1),  (0, -1),
        (-1, 0), (-1, 1), (-1, -1),
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
                        Piece::Pawn => pawn_attacks((row, col), color),
                        Piece::Knight => knight_moves(board, (row, col), color),
                        Piece::Bishop => bishop_moves(board, (row, col), color),
                        Piece::Rook => rook_moves(board, (row, col), color),
                        Piece::Queen => queen_moves(board, (row, col), color),
                        Piece::King => king_moves(board, (row, col), color),
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

