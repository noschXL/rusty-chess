use crate::engine::{constants::fen::PIECES, pieces::*};
#[derive(Debug)]
pub struct Board {
    data: [Piece; 64],
    castlerights: [bool;4], // * top left, top right, bottom left, bottom right
    whitesturn: bool,
    en_passant: i32,
    halfmove_clock: i32,
    fullmoves: i32,
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
            if step == 1 {
                let mut index = 0;
                for char in part{
                    if !char.is_numeric() {
                        let color = !char.is_lowercase();
                        let lower = char.to_lowercase().last().unwrap();
                        let piece = PIECES.chars().position(|c| c == lower).unwrap() << 1;
                        board.set_piece_data_at(index, piece as u32 | color as u32);
                        index += 1;
                    }else if char.is_numeric(){
                        index += char.to_digit(10).unwrap() as usize;
                    }

                }

            }else if step == 2 {
                board.whitesturn = part.nth(0) == Some('w')
            }else if step == 3 {
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
            }else if step == 4{
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
            }else if step == 5{
                let mut halfmove_clock = 0;
                for char in part {
                    halfmove_clock *= 10;
                    halfmove_clock += char.to_digit(10).unwrap();
                }

                board.halfmove_clock = halfmove_clock as i32;

            }else if step == 6{
                let mut fullmove_counter = 0;
                for char in part {
                    fullmove_counter*= 10;
                    fullmove_counter+= char.to_digit(10).unwrap();
                }

                board.halfmove_clock = fullmove_counter as i32;
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
            fullmoves: 0
        }
    }

    pub fn get_piece_at(&self, index: usize) -> &Piece{
        &self.data[index]
    }

    pub fn set_piece_data_at(&mut self, index: usize, data: u32){
        self.data[index].data = data
    }
}