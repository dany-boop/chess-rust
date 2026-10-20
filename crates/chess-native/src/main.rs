use chess_native::app::ChessApp;
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

                Ok(Box::new(ChessApp::default()))
            }),
        )
}


