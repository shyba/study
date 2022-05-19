use bitvec::prelude as bv;
use core::cmp::min;
use core::str::FromStr;

pub const COLUMNS: usize = 64;
pub const ROWS: usize = 20;
pub struct GameOfLife {
    pub screen: bv::BitArr!(for COLUMNS*ROWS, in u8, bv::Msb0),
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
                    '#' => game.screen.set(idxr * COLUMNS + idxc, true),
                    _ => (),
                }
            }
        }
        Ok(game)
    }
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {
            screen: bv::bitarr!(u8, bv::Msb0; 0; COLUMNS*ROWS),
        }
    }

    pub fn get_at(self: &Self, row: usize, col: usize) -> Option<bool> {
        let row = row * COLUMNS;
        match self.screen.get(row + col) {
            Some(x) if *x => Some(true),
            Some(_) => Some(false),
            None => None,
        }
    }

    pub fn advance(self: &mut Self) {
        let mut tmp = GameOfLife::new();
        tmp.screen = self.screen.clone();
        for idxr in 0..ROWS {
            for idxc in 0..COLUMNS {
                let index = idxr * COLUMNS + idxc;
                let new_state = tmp.next_state_at(idxr, idxc);
                self.screen.set(index, new_state);
            }
        }
    }

    pub fn count_alive_neighbors(self: &Self, row: usize, col: usize) -> usize {
        let min_row = row.checked_sub(1).unwrap_or(0);
        let max_row = min(ROWS - 1, row + 1);
        let min_col = col.checked_sub(1).unwrap_or(0);
        let max_col = min(COLUMNS - 1, col + 1);
        let mut alive = 0;
        for idxr in min_row..=max_row {
            for idxc in min_col..=max_col {
                if idxc == col && idxr == row {
                    continue;
                }
                alive = match self.get_at(idxr, idxc) {
                    Some(true) => alive + 1,
                    _ => alive,
                };
            }
        }
        alive
    }

    pub fn next_state_at(self: &Self, row: usize, col: usize) -> bool {
        let is_cell_alive = self.get_at(row, col).unwrap();
        let neighbors = self.count_alive_neighbors(row, col);
        if is_cell_alive {
            neighbors == 2 || neighbors == 3
        } else {
            neighbors == 3
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gol::GameOfLife;
    use core::str::FromStr;
    #[test]
    fn it_can_parse() {
        let game = GameOfLife::from_str(
            "### #\n\
             #\n\
             #",
        )
        .unwrap();
        assert_eq!(true, game.screen.get(0..3).unwrap().all());
        assert_eq!(false, game.screen.get(0..4).unwrap().all());
        assert_eq!(true, game.screen.get(4).unwrap());

        assert_eq!(true, game.get_at(1, 0).unwrap());
    }

    #[test]
    fn it_dies_from_underpop_simple() {
        let mut game = GameOfLife::from_str("##").unwrap();
        game.advance();
        let expected = GameOfLife::from_str("  ").unwrap();
        assert_eq!(expected.screen, game.screen);
    }

    #[test]
    fn it_revives_from_three_cells() {
        let mut game = GameOfLife::from_str(
            r#"
        #
        #
          #
        "#,
        )
        .unwrap();
        game.advance();
        let expected = GameOfLife::from_str(
            r#"

         #

        "#,
        )
        .unwrap();
        assert_eq!(expected.screen, game.screen);
    }

    #[test]
    fn it_survives() {
        let mut game = GameOfLife::from_str(
            r#"
            #
             #
             #
        "#,
        )
        .unwrap();
        game.advance();
        let expected = GameOfLife::from_str(
            r#"

            ##

        "#,
        )
        .unwrap();
        assert_eq!(expected.screen, game.screen);
    }

    #[test]
    fn it_dies_from_overpop() {
        let mut game = GameOfLife::from_str(
            r#"
            # #
            ##
             ##
        "#,
        )
        .unwrap();
        game.advance();
        let expected = GameOfLife::from_str(
            r#"
            #
            #
            ###
        "#,
        )
        .unwrap();
        assert_eq!(expected.screen, game.screen);
    }
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
