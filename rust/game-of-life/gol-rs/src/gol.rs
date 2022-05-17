use bitvec::prelude as bv;
use core::str::FromStr;

pub const COLUMNS: usize = 64;
pub const ROWS: usize = 20;
pub struct GameOfLife {
    pub screen: bv::BitArr!(for COLUMNS*ROWS, in u8, bv::Msb0)
}

#[derive(Debug)]
pub struct ParseGameError;

impl FromStr for GameOfLife {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = GameOfLife::new();
        for idxr in 0..ROWS {
            for idxc in 0..COLUMNS {
                let index = idxc*COLUMNS + idxr;
                match s.chars().nth(index as usize) {
                    Some(c) if c == '#' => game.screen.set(index, true),
                    Some(_) => game.screen.set(index, false),
                    None => ()
                }
            }
        }
        Ok(game)
    }

}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {screen: bv::bitarr!(u8, bv::Msb0; 0; COLUMNS*ROWS)}
    }
}