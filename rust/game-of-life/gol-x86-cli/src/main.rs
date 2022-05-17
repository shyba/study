use gol_rs::gol::*;
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