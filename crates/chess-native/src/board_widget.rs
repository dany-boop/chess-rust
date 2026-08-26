use chess_core::{
    board::{
        empty_board, place_black_pawns, place_black_pieces, place_white_pawns, place_white_pieces,
        Board,
    },
    types::Piece,
};
use eframe::egui::{self, vec2, Rect, Sense, SystemTheme::Light};

pub struct BoardStruct {
    board: Board,
}

impl Default for BoardStruct {
    fn default() -> Self {
        let mut board = empty_board();
        place_white_pieces(&mut board);
        place_white_pawns(&mut board);
        place_black_pawns(&mut board);
        place_black_pieces(&mut board);
        BoardStruct { board }
    }
}
impl eframe::App for BoardStruct {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Chess Fuck Yourself");
                ui.add_space(10.0);

                let square_size = 50.0;
                let grid_spacing = 2.0;
                let light = egui::Color32::YELLOW;
                let dark = egui::Color32::BROWN;
                let step = square_size + grid_spacing;

                for row in 0..8 {
                    for col in 0..8 {
                        let square_color = if (row + col) % 2 == 0 { light } else { dark };
                        let square_rect = Rect::from_min_size(
                            ui.min_rect().min + vec2(col as f32 * step, row as f32 * step),
                            vec2(square_size, square_size),
                        );

                        let response = ui.allocate_rect(square_rect, Sense::click());
                        ui.painter()
                            .rect_filled(square_rect, 0.0,  square_color);

                        if let Some(piece_on_square) = self.board.squares[row][col] {
                            let letter = match piece_on_square.piece {
                                Piece::Pawn => "P",
                                Piece::Knight => "N",
                                Piece::Bishop => "B",
                                Piece::Rook => "R",
                                Piece::Queen => "Q",
                                Piece::King => "K",
                            };

                            let text_color = match piece_on_square.color {
                                chess_core::types::Color::White => egui::Color32::WHITE,
                                chess_core::types::Color::Black => egui::Color32::BLACK,
                            };

                            ui.painter().text(
                                square_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                letter,
                                egui::FontId::default(),
                                text_color,
                            );
                        }
                    }
                }
                // let mut move_to execute= None
            });
        });
    }
}
