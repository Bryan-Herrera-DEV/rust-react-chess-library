#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PieceType {
    Empty = 0, // Casilla vacía
    King,      // Rey
    Queen,     // Reina
    Rook,      // Torre
    Bishop,    // Alfil
    Knight,    // Caballo
    Pawn,      // Peón
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    /**
     * Crea una nueva pieza con el tipo y color especificados
     */
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }

    /**
     * to_byte convierte la pieza a un byte, donde los primeros 3 bits son el tipo de pieza y los últimos 3 son el color
     */
    pub fn to_byte(&self) -> u8 {
        (self.piece_type as u8) | ((self.color as u8) << 3)
    }

    /**
     * from_byte convierte un byte a una pieza
     */
    pub fn from_byte(byte: u8) -> Piece {
        let piece_type = match byte & 0b0000_0111 {
            // Esta linea obtiene los 3 primeros bits del byte
            1 => PieceType::King,
            2 => PieceType::Queen,
            3 => PieceType::Rook,
            4 => PieceType::Bishop,
            5 => PieceType::Knight,
            6 => PieceType::Pawn,
            _ => return none,
        };

        let color = if byte & 0b0000_1000 == 0 { // Esta linea obtiene el cuarto bit del byte
            Color::White
        } else {
            Color::Black
        };

        Some(Piece::new(piece_type, color))
    }
}
