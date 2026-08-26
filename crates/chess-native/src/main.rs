use chess_core::board::*;
use chess_core::movegen::*;
use chess_native::board_widget::BoardStruct;
use eframe::egui;


// fn move_piece(board: &mut Board, from: (usize, usize), to: (usize, usize)){
//     let piece = board.squares[from.0][from.1];
//     board.squares[to.0][to.1] = piece;
//     board.squares[from.0][from.1] = None;
// }

fn main()-> eframe::Result<()> {

    let mut options = eframe::NativeOptions::default(); 
    options.viewport = egui::ViewportBuilder::default()
        .with_title("Chess Game")
        .with_inner_size([800.0, 600.0]);
        
        eframe::run_native(
            "Chess Game",
            options,
            Box::new(|cc| {
                cc.egui_ctx.set_visuals(egui::Visuals::dark());

                Ok(Box::new(BoardStruct::default()))
            }),
        )
    // let mut board = empty_board();
    // place_white_pieces(&mut board);
    // place_white_pawns(&mut board);

    // place_black_pawns(&mut board);
    // place_black_pieces(&mut board);
    // let pawn_moves = pawn_moves(&board, (1, 4), Color::White);
    // println!("Pawn moves: {:?}", pawn_moves);

    // let knight_moves = knight_moves(&board, (0, 1), Color::White);
    // println!("Knight moves: {:?}", knight_moves);

    // move_piece(&mut board, (1, 4), (4, 4));
    // move_piece(&mut board, (1, 3), (4, 3));
    // println!("{:#?}", board);
    // // --- test is_square_attacked ---
    

    // // Clear a path so black's rook at (7,0) can see down the board.
    // // Remove the black pawn in front of it at (6,0).
    // board.squares[6][0] = None;

    // let attacked = is_square_attacked(&board, (5, 0), Color::Black);
    // println!("Is (5,0) attacked by Black? {}", attacked); // expect true — rook sees straight down the a-file

    // let not_attacked = is_square_attacked(&board, (5, 4), Color::Black);
    // println!("Is (5,4) attacked by Black? {}", not_attacked); // expect false — nothing lines up there

}


