use crate::engine::constants;

#[derive(Debug)]
pub struct Piece {
    pub data: u32,
}

impl Piece {

    pub fn new_empty() -> Piece{
        Piece{data: 0}
    }
    pub fn new_full(data: u32) -> Piece{
        Piece{data}
    }

    pub fn get_type(&self) -> u32{
        self.data & constants::bitmasks::PIECE_MASK
    }

    pub fn get_color(&self) -> u32{
        self.data & constants::bitmasks::COLOR_MASK
    }

    pub fn is_empty(&self) -> bool{
        self.data == 0
    }

    pub fn get_moves(&self)-> Vec<u32> {
        let mut moves: Vec<u32>;
        match self.get_type() {
            constants::piece_constants::ROOK => {
                todo!("Rookmoves not yet verifiable");
                moves
            },
            _=>{
                panic!("invalid piece type, returning nothing");
                moves
            },
        }
    }
}

impl Clone for Piece {
    fn clone(&self)->Piece {
        Piece::new_full(self.data)
    }
}
