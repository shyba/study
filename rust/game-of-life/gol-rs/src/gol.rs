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
        for (idxr, line) in s.lines().enumerate() {
            for (idxc, c) in line.chars().enumerate() {
                match c {
                    '#' => game.screen.set(idxr*ROWS + idxc, true),
                    _ => ()
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

    pub fn get_at(self, row: usize, col: usize) -> Option<bool> {
        let row = row * ROWS;
        match self.screen.get(row + col) {
            Some(x) if *x => Some(true),
            Some(_) => Some(false),
            None => None
        }
    }

    pub fn advance(self: &mut Self) {
        let tmp = GameOfLife::new();
        self.screen = tmp.screen;
    }
}