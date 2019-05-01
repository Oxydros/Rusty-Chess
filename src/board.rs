use std::collections::HashMap;

use crate::piece::{ChessPiece, load_pieces};

pub struct ChessBoard { 
    board: [Option<ChessPiece>; 64],
    images: HashMap<&'static ChessPiece, conrod_core::image::Id>,
}

impl ChessBoard {

    pub fn init(&mut self, display: &glium::Display, image_map : &mut conrod_core::image::Map<glium::texture::Texture2d>) -> (){
        self.images = load_pieces(&display, image_map);
    }

    pub fn fetch_piece_type(&self, row : usize, col : usize) -> Option<ChessPiece> {
        self.board[col + (row * 8)].clone()
    }

    pub fn fetch_piece_picture_id(&self, piece_type : &ChessPiece) -> conrod_core::image::Id {
        self.images[piece_type]
    }
}

impl Default for ChessBoard {
    fn default() -> ChessBoard {
        ChessBoard{
            board: [
                Some(ChessPiece::Rook), Some(ChessPiece::Knight), Some(ChessPiece::Bishop), Some(ChessPiece::Queen), Some(ChessPiece::King), Some(ChessPiece::Bishop), Some(ChessPiece::Knight), Some(ChessPiece::Rook),
                Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn), Some(ChessPiece::Pawn),
                Some(ChessPiece::Rook), Some(ChessPiece::Knight), Some(ChessPiece::Bishop), Some(ChessPiece::Queen), Some(ChessPiece::King), Some(ChessPiece::Bishop), Some(ChessPiece::Knight), Some(ChessPiece::Rook),
            ],
            images: HashMap::new(),
        }
    }
}