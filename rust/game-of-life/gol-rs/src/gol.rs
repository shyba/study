use bitvec::prelude as bv;
use core::str::FromStr;
use core::cmp::min;

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

    pub fn get_at(self: &Self, row: usize, col: usize) -> Option<bool> {
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

    pub fn count_alive_neighbors(self: &Self, row: usize, col: usize) -> usize {
        let min_row = row.checked_sub(1).unwrap_or(0);
        let max_row = min(ROWS-1, row+1);
        let min_col = col.checked_sub(1).unwrap_or(0);
        let max_col = min(COLUMNS-1, col+1);
        let mut alive = 0;
        for idxr in min_row..=max_row {
            for idxc in min_col..=max_col {
                if idxc == col && idxr == row {
                    continue
                }
                alive = match self.get_at(idxr, idxc) {
                    Some(true) => alive + 1,
                    _ => alive
                };
            }
        }
        alive
    }
}

#[cfg(test)]
mod tests {
    use crate::gol::GameOfLife;
    use core::str::FromStr;
    #[test]
    fn it_counts_alive_neighbors() {
        let game = GameOfLife::from_str("#\n#\n#\n###").unwrap();
        assert_eq!(1, game.count_alive_neighbors(0, 0));
        assert_eq!(2, game.count_alive_neighbors(1, 0));
        assert_eq!(3, game.count_alive_neighbors(2, 0));
        assert_eq!(5, game.count_alive_neighbors(2, 1));
        assert_eq!(2, game.count_alive_neighbors(3, 0));
    }
}