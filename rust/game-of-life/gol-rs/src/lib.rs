#![no_std]
pub mod gol;

#[cfg(test)]
mod tests {
    use crate::gol::GameOfLife;
    use core::str::FromStr;
    #[test]
    fn it_can_parse() {
        let game = GameOfLife::from_str(
            "### #\n\
             #\n\
             #").unwrap();
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
        let mut game = GameOfLife::from_str(r#"
        #
        #
          #
        "#).unwrap();
        game.advance();
        let expected = GameOfLife::from_str(r#"

         #

        "#).unwrap();
        assert_eq!(expected.screen, game.screen);
    }

    #[test]
    fn it_survives() {
        let mut game = GameOfLife::from_str(r#"
            #
             #
             #
        "#
        ).unwrap();
        game.advance();
        let expected = GameOfLife::from_str(r#"

            ##

        "#
        ).unwrap();
        assert_eq!(expected.screen, game.screen);
    }

    #[test]
    fn it_dies_from_overpop() {
        let mut game = GameOfLife::from_str(r#"
            # #
            ##
             ##
        "#
        ).unwrap();
        game.advance();
        let expected = GameOfLife::from_str(r#"
            #
            #
            ###
        "#
        ).unwrap();
        assert_eq!(expected.screen, game.screen);
    }
}
