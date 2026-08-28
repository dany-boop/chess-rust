use chess_core::{
    board::{
        empty_board, place_black_pawns, place_black_pieces, place_white_pawns, place_white_pieces,
        Board,
    },
    types::Piece::{self, Rook},
};
use eframe::egui::{self, vec2, Rect, Sense};

pub struct BoardStruct {
    board: Board,
    selected: Option<(usize, usize)>,
}

impl Default for BoardStruct {
    fn default() -> Self {
        let mut board = empty_board();
        place_white_pieces(&mut board);
        place_white_pawns(&mut board);
        place_black_pawns(&mut board);
        place_black_pieces(&mut board);
        let selected: Option<(usize, usize)> = None;
        BoardStruct { board, selected }
    }
}

pub fn draw_board(
    ui: &mut egui::Ui,
    board: &Board,
    selected: Option<(usize, usize)>,
) -> Option<(usize, usize)> {
    let mut clicked_square = None;
    let square_size: f32 = 50.0;
    let light = egui::Color32::YELLOW;
    let dark = egui::Color32::BROWN;
    let highlight = egui::Color32::from_rgb(100, 200, 100);
    for row in 0..8 {
        for col in 0..8 {
            let is_selected = selected == Some((row, col));
            let square_color = if is_selected {
                highlight
            } else if (row + col) % 2 == 0 {
                light
            } else {
                dark
            };
            let square_rect = Rect::from_min_size(
                ui.min_rect().min + vec2(col as f32 * square_size, row as f32 * square_size),
                vec2(square_size, square_size),
            );
            let response = ui.allocate_rect(square_rect, Sense::click());
            if response.clicked() {
                clicked_square = Some((row, col))
            }
            ui.painter().rect_filled(square_rect, 0.0, square_color);
            if let Some(piece_on_square) = board.squares[row][col] {
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
    clicked_square
}
