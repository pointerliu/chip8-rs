use std::error::{self, Error};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use derive_builder::Builder;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[derive(Builder)]
struct Chip8App {
    map: Vec<bool>,
    #[builder(default = false)]
    exit: bool,
}

impl Chip8App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        while !self.exit {
            terminal.draw(|frame| {
                self.map.iter().enumerate().for_each(|(index, value)| {
                    if *value {
                        let x = (index % SCREEN_WIDTH) as u16;
                        let y = (index / SCREEN_WIDTH) as u16;
                        let area = Rect::new(x, y, 2, 1);
                        let block = Block::default().style(Style::new().on_white());
                        frame.render_widget(block, area);
                    }
                });
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), Box<dyn Error>> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_events(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();
        let mut terminal = ratatui::init();
        let arr: Vec<bool> = (0..SCREEN_WIDTH * SCREEN_HEIGHT)
            .map(|x| rng.gen_bool(0.5))
            .collect();
        assert!(arr.len() == SCREEN_WIDTH * SCREEN_HEIGHT);
        let app_result = Chip8AppBuilder::default()
            .map(arr)
            .build()
            .unwrap()
            .run(&mut terminal);
        ratatui::restore();
    }
}
