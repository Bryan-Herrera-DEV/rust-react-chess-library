use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ChessBoard {
    board: Vec<u8>,
}

#[wasm_bindgen]
impl ChessBoard {
    pub fn new() -> ChessBoard {
        let board = vec![0; 8 * 8];
        ChessBoard { board }
    }

    pub fn get_board(&self) -> Vec<u8> {
        self.board.clone()
    }
}