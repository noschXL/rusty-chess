

pub mod piece_constants {
    pub const WHITE: u32 = 0b0;
    pub const BLACK: u32 = 0b1;

    pub const EMPTY: u32 = 0b0000;
    pub const PAWN: u32 = 0b0010;
    pub const KNIGHT: u32 = 0b0100;
    pub const BISHOP: u32 = 0b0110;
    pub const ROOK: u32 = 0b1000;
    pub const QUEEN: u32 = 0b1010;
    pub const KING: u32 = 0b1100;
}

pub mod bitmasks {
    pub const PIECE_MASK: u32 = 0b1110;
    pub const COLOR_MASK: u32 = 0b1;
}

/*

*/
pub mod movement {
    pub const ROOK: [i32; 4] = [-1, 1, -8, 8];
    pub const BISHOP: [i32; 4] = [-7, -9,  7,  9];
    
}