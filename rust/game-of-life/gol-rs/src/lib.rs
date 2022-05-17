#![no_std]
pub mod gol;

#[cfg(test)]
mod tests {
    use crate::gol::GameOfLife;
    use core::str::FromStr;
    #[test]
    fn it_can_parse() {
        let game = GameOfLife::from_str("### #").unwrap();
        assert_eq!(true, game.screen.get(0..3).unwrap().all());
        assert_eq!(false, game.screen.get(0..4).unwrap().all());
        assert_eq!(true, game.screen.get(4).unwrap());
    }
}
