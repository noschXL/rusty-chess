use std::ops::Deref;

use macroquad::prelude::warn;

use crate::engine::pieces::*;
#[derive(Debug)]
pub struct Board {
    data: [Piece; 64],
    rooksmoved: [bool;4],
    whitesturn: bool,
}


impl Board {
    pub fn new_fen(fen: &String) -> Board{
        warn!("fen encoding is not yet provided");
        Board::new_empty()
    }

    pub fn new_empty() -> Board{
        Board { data: std::array::from_fn(|_| Piece::new_empty()), rooksmoved: [false,false,false,false], whitesturn: true }
    }

    pub fn get_piece_at(&self, index: usize) -> &Piece{
        &self.data[index]
    }

    pub fn set_piece_data_at(&mut self, index: usize, data: u32){
        self.data[index].data = data
    }
}