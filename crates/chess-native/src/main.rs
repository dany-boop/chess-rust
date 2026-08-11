use chess_core::board::*;
use chess_core::movegen::*;
use chess_core::types::*;

// fn move_piece(board: &mut Board, from: (usize, usize), to: (usize, usize)){
//     let piece = board.squares[from.0][from.1];
//     board.squares[to.0][to.1] = piece;
//     board.squares[from.0][from.1] = None;
// }

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
