use bitvec::prelude as bv;

const COLUMNS: usize = 64;
const ROWS: usize = 20;
pub struct GameOfLife {
    screen: bv::BitArr!(for COLUMNS*ROWS, in u8, bv::Msb0)
}

fn main() {
    let game = GameOfLife::new();
    render(&game);
}

fn render(game: &GameOfLife) {
    println!("{}", "-".repeat(COLUMNS+2));
    for idxr in 0..ROWS {
        print!("|");
        for idxc in 0..COLUMNS {
            let index = idxr*COLUMNS + idxc;
            match game.screen.get(index).as_deref() {
                Some(true) => print!("#"),
                Some(false) => print!(" "),
                None => println!("ERROR AT {}", index)
            }
        }
        println!("|");
    }
    println!("{}", "-".repeat(COLUMNS+2));
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {screen: bv::bitarr!(u8, bv::Msb0; 0; COLUMNS*ROWS)}
    }
}