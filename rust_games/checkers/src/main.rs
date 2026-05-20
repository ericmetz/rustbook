#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Copy, Default)]
pub struct BitMap(pub u64);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position{
    bb_sides: [BitMap; 2],
    bb_piece: [[BitMap; 6]; 2],
}

pub struct Sides;

impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Pieces;

impl Pieces {
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}




fn main() {
    println!("Hello World!");
}

