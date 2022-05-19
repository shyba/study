use std::cmp::min;
use std::io;
use gol_rs::gol::*;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::Terminal;
use tui::widgets::Widget;

fn main() {
    let mut game = GameOfLife::from_str(
        r#"#


                #
               ###
                #"#).unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(100);

    for _ in 0..100 {
        terminal.draw(|f| {
            let size = f.size();
            let widget = App::new(&game);
            f.render_widget(widget, size);
        }).unwrap();
        sleep(tick_rate);
        game.advance();
    }
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}

struct App<'a> {
    game: &'a GameOfLife,
}

impl<'a> App<'a> {
    fn new(game: &'a GameOfLife) -> App {
        App {
            game,
        }
    }

    fn all_black(self: &Self, buf: &mut Buffer) {
        for cell in buf.content.iter_mut() {
            cell.set_fg(Color::Black).set_bg(Color::Black);
        }
    }

    fn game_to_buf(self, area: Rect, buf: &mut Buffer) {
        //self.all_black(buf);
        let cols = min(COLUMNS, area.width.checked_sub(COLUMNS as u16).unwrap_or(0u16).into());
        let rows = min(ROWS, area.height.checked_sub(ROWS as u16).unwrap_or(0u16).into());
        for idxr in 0..rows {
            for idxc in 0..cols {
                let value = self.game.get_at(idxr, idxc).unwrap();
                let color = match &value {
                    true => Color::Red,
                    _ => Color::Black,
                };
                buf.get_mut(idxc as u16, (10+idxr) as u16).set_bg(color);
            }
        }
    }
}

impl<'a> Widget for App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.game_to_buf(area, buf);
    }
}

#[allow(dead_code)]
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