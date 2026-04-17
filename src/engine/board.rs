use std::{net::UdpSocket, usize};

use crate::engine::{constants::fen::PIECES, pieces::*};
#[derive(Debug)]
pub struct Board {
    data: [Piece; 64],
    castlerights: [bool;4], // * top left, top right, bottom left, bottom right
    whitesturn: bool,
    en_passant: i32,
    halfmove_clock: i32,
    fullmoves: i32,
    moves: Vec<Move>,
    captures: Vec<Piece>,
}


impl Board {
    /*
    * 1: position
    * 2: player to move
    * 3: castling rights
    * 4: that french move
    * 5: halfmove-clock, if 100 its a tie. reset at pawn push / capture
    * 6: fullmove counter
    */
    pub fn new_fen(fen: &String) -> Board{
        let parts = fen.split_whitespace();

        let mut board = Board::new_empty();

        for (step, part) in parts.enumerate() {
            let mut part = part.chars();
            if step == 0 {
                let mut index = 0;
                for char in part{
                    if char.is_alphabetic() {
                        let color = char.is_lowercase();
                        let lower = char.to_lowercase().last().unwrap();

                        let piece = (PIECES.chars().position(|c| c == lower).unwrap()+1) << 1;
                        board.set_piece_data_at(index, piece as u32 | color as u32);
                        index += 1;
                    }else if char.is_numeric(){
                        index += char.to_digit(10).unwrap() as usize;
                    }
                }

            }else if step == 1 {
                board.whitesturn = part.nth(0) == Some('w')
            }else if step == 2 {
                let mut rights: [bool; 4] = [false,false,false,false];
                for char in part {
                    match char {
                        'q'=>rights[0] = true,
                        'k'=>rights[1] = true,
                        'Q'=>rights[2] = true,
                        'K'=>rights[3] = true,
                        _=>break
                    }
                }
                board.castlerights = rights;
            }else if step == 3{
                let mut en_passant = 0;
                for char in part{
                    if char == '-'{
                        en_passant = -1;
                        break;
                    }

                    match char {
                        'a'=>en_passant += 0 * 8,
                        'b'=>en_passant += 0 * 8,
                        'c'=>en_passant += 0 * 8,
                        'd'=>en_passant += 0 * 8,
                        'e'=>en_passant += 0 * 8,
                        'f'=>en_passant += 0 * 8,
                        'g'=>en_passant += 0 * 8,
                        'h'=>en_passant += 0 * 8,
                        _=> en_passant += char.to_digit(10).unwrap() as i32
                    }
                }

                board.en_passant = en_passant;
            }else if step == 4{
                let mut halfmove_clock = 0;
                for char in part {
                    halfmove_clock *= 10;
                    halfmove_clock += char.to_digit(10).unwrap();
                }

                board.halfmove_clock = halfmove_clock as i32;

            }else if step == 5{
                let mut fullmove_counter = 0;
                for char in part {
                    fullmove_counter*= 10;
                    fullmove_counter+= char.to_digit(10).unwrap();
                }

                board.fullmoves = fullmove_counter as i32;
            }
        }

        board
    }

    pub fn new_empty() -> Board{
        Board {
            data: std::array::from_fn(|_| Piece::new_empty()),
            castlerights: [true,true,true,true],
            whitesturn: true,
            en_passant: -1,
            halfmove_clock: 0,
            fullmoves: 0,
            moves: Vec::new(),
            captures: Vec::new(),
        }
    }

    pub fn get_piece_at(&self, index: usize) -> &Piece{
        &self.data[index]
    }

    pub fn set_piece_data_at(&mut self, index: usize, data: u32){
        self.data[index].data = data
    }

    pub fn make_move(&mut self, mov: Move){
        if mov.capture {
            self.captures.push(self.data[mov.to as usize].clone());
        }
        
        self.data[mov.to as usize] = self.data[mov.from as usize].clone();
        
        self.data[mov.from as usize] = Piece::new_empty();
        
        self.moves.push(mov);
    }
    
    pub fn unmake_move(&mut self){
        let mut p = Piece::new_empty();
        let mov = self.moves.pop().unwrap_or_else(Move::new_invalid);
        if mov.is_invalid() {
            return
        }
        if mov.capture {
            p = self.captures.pop().unwrap();
        }
        
        self.data[mov.from as usize] = self.data[mov.to as usize].clone();
        
        self.data[mov.to as usize] = p;
    }
}


#[derive(Debug)]
pub struct Move {
    from: i32,
    to: i32,
    capture: bool,
    promotion: i32,
    en_passant: bool,
}

impl Move {
    pub fn new_invalid() -> Move{
        Move{
            from: 1,
            to: 1,
            capture: false,
            promotion: 0,
            en_passant: false,
        }
    }
    pub fn new(from: i32, to: i32, capture: bool, promotion: i32, en_passant: bool) -> Move{
        Move{from, to, capture, promotion, en_passant}
    }
    pub fn is_invalid(&self) -> bool{
        self.from == self.to
    }
}
