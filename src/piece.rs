use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ChessPiece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

static PIECE_LIST : [(ChessPiece, &str); 6] = [
    (ChessPiece::Pawn, "pieces/pawn.png"),
    (ChessPiece::Rook, "pieces/rook.png"),
    (ChessPiece::Knight, "pieces/knight.png"),
    (ChessPiece::Bishop, "pieces/bishop.png"),
    (ChessPiece::Queen, "pieces/queen.png"),
    (ChessPiece::King, "pieces/king.png"),
];

pub fn load_pieces(display: &glium::Display, image_map : &mut conrod_core::image::Map<glium::texture::Texture2d>) -> HashMap<&'static ChessPiece, conrod_core::image::Id> {
    let mut piece_images = HashMap::new();

    for (piece_id, piece_path) in PIECE_LIST.iter() {
        let texture = load_piece_image(&display, piece_path);
        let image_id = image_map.insert(texture);
        piece_images.insert(piece_id, image_id);
    }
    piece_images
}

fn load_piece_image(display: &glium::Display, piece_path : &str) -> glium::texture::Texture2d {
    let assets = find_folder::Search::ParentsThenKids(5, 3).for_folder("assets/").unwrap();
    let path = assets.join(piece_path);
    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}