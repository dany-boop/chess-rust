use chess_core::{
    board::{
        Board, Move, empty_board, place_black_pawns, place_black_pieces, place_white_pawns, place_white_pieces,
    }, movegen::{legal_moves_for, move_piece},
};
use eframe::egui::{self};

use crate::board_widget;

pub struct ChessApp {
    board: Board,
    selected: Option<(usize, usize)>,
    turn_color: chess_core::types::Color,
    history: Vec<Move>,
}

impl Default for ChessApp {
    fn default() -> Self {
        let mut board = empty_board();
        place_white_pieces(&mut board);
        place_white_pawns(&mut board);
        place_black_pawns(&mut board);
        place_black_pieces(&mut board);
        ChessApp {
            board,
            selected: None,
            turn_color: chess_core::types::Color::White,
            history: Vec::new(),
        }
    }
}

impl eframe::App for ChessApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Chess Fuxk Urself");

            let legal_moves = match self.selected {
                Some(from) => legal_moves_for(&self.board, from),
                None => vec![],
            };

            let turn_color_text = match self.turn_color {
                chess_core::types::Color::White => "White's Turn",
                chess_core::types::Color::Black => "Black's Turn",
            };
            ui.label(turn_color_text);

            if let Some(clicked) =
                board_widget::draw_board(ui, &self.board, self.selected, &legal_moves)
            {
                match self.selected {
                    None => {
                        if self.board.squares[clicked.0][clicked.1].is_some() {
                            self.selected = Some(clicked);
                        } else {
                            self.selected = None;
                        }
                    }

                    Some(from) => {
                        if  legal_moves.contains(&clicked) {
                            if let Some(moved_piece) = move_piece(&mut self.board, from, clicked) {
                                self.history.push(moved_piece);
                                self.turn_color = self.turn_color.opposite();
                                self.selected = None;
                            }
                        } else if self.board.squares[clicked.0][clicked.1].is_some()  {
                            self.selected = Some(clicked);
                        } else {
                            self.selected = None;
                        }
                    }
                }
            }
        });
    }
}
