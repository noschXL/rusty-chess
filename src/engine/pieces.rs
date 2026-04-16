use crate::engine::constants;
pub struct Piece {
    data: u32,
}

impl Piece {
    fn new(data: u32) -> Piece{
        Piece {data}
    }

    fn get_type(self) -> u32{
        self.data & constants::bitmasks::PIECE_MASK
    }

    fn get_color(self) -> u32{
        self.data & constants::bitmasks::COLOR_MASK
    }

    fn get_moves(self)-> Vec<u32> {
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