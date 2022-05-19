use gol_rs::gol::*;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut game = GameOfLife::from_str(
        r#"#


                #
               ###
                #"#).unwrap();
    loop {
        render(&game);
        game.advance();
        sleep(Duration::from_secs_f32(0.1));
    }
}

fn render(game: &GameOfLife) {
    dbg!(game.screen.count_ones());
    dbg!(game.screen.count_zeros());
    println!("{}", "-".repeat(COLUMNS+2));
    for idxr in 0..ROWS {
        print!("|");
        for idxc in 0..COLUMNS {
            let index = idxr*COLUMNS + idxc;
            let value = game.screen.get(index);
            match value.as_deref() {
                Some(true) => print!("#"),
                Some(false) => print!("-"),
                None => println!("ERROR AT {}", index)
            }
        }
        println!("|");
    }
    println!("{}", "-".repeat(COLUMNS+2));
}