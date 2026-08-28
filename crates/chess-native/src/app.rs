use chess_core::{
    board::{
        Board, empty_board, place_black_pawns, place_black_pieces, place_white_pawns, place_white_pieces,
    }, movegen::{legal_moves_for, move_piece},
};
use eframe::egui::{self};

use crate::board_widget;

pub struct ChessApp {
    board: Board,
    selected: Option<(usize, usize)>,
}

impl Default for ChessApp {
    fn default() -> Self {
        let mut board = empty_board();
        place_white_pieces(&mut board);
        place_white_pawns(&mut board);
        place_black_pawns(&mut board);
        place_black_pieces(&mut board);
        let selected: Option<(usize, usize)> = None;
        ChessApp {
            board,
            selected: None,
        }
    }
}

impl eframe::App for ChessApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Chess Fuxk Urself");
            if let Some(clicked) = board_widget::draw_board(ui, &self.board, self.selected) {
                match self.selected {
                    None => {
                        if self.board.squares[clicked.0][clicked.1].is_some() {
                            self.selected = Some(clicked);
                        }
                    }
                    Some(from) => {
                        let legal = legal_moves_for(&self.board, from);
                        if legal.contains(&clicked){
                            move_piece(&mut self.board, from, clicked);
                        }
                        self.selected = None;
                    }
                }
            }
        });
    }
}
