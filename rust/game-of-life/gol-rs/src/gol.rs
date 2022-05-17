use bitvec::prelude as bv;

pub const COLUMNS: usize = 64;
pub const ROWS: usize = 20;
pub struct GameOfLife {
    pub screen: bv::BitArr!(for COLUMNS*ROWS, in u8, bv::Msb0)
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {screen: bv::bitarr!(u8, bv::Msb0; 0; COLUMNS*ROWS)}
    }
}