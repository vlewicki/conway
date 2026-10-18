mod conway;
mod conwidget;
mod constore;
mod gamestate;
mod app;


use std::{io::{Result, stdout}, thread, time::Duration};

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEvent}, execute};
use ratatui::Frame;

use crate::{app::App, conway::{GameOfLife, Grid}, gamestate::GameState};


struct ConwayApp{
    running: bool,
    game: GameOfLife,
    paused: bool
}


impl App for ConwayApp{
    fn running(&self) -> bool {
        self.running
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_stateful_widget(&self.game, frame.area(), &mut GameState{paused: self.paused});
    }

    fn tick(&mut self) -> std::io::Result<()> {
        if !self.paused{
            self.game.step()
        }
        self.handle_input()?;
        thread::sleep(Duration::from_millis(16));
        Ok(())
    }
}

impl ConwayApp {
    pub fn run(&mut self) -> Result<()>{
        execute!(stdout(), EnableMouseCapture).unwrap();
        let result = App::run(self);
        execute!(stdout(), DisableMouseCapture).unwrap();
        result
    }
    fn handle_input(&mut self) -> Result<()> {
        while event::poll(Duration::from_millis(0))? {
            let Self { running, game, paused } = self;
            match event::read()? {
                Event::Mouse(MouseEvent{kind, column, row, modifiers: _}) => {
                    if let event::MouseEventKind::Down(event::MouseButton::Left) = kind {
                        let (h, w) = game.get_grid().size();
                        let (x, y) = (((column / 2) as usize), (row as usize));
                        if y < h && x < w {
                            game.toggle(y, x);
                        }
                    }
                }
                Event::Key(KeyEvent{code, modifiers: _, kind: _, state: _}) => {
                    match code {
                        KeyCode::Char('q') => *running = false,
                        KeyCode::Char('p') => *paused = !*paused,
                        KeyCode::Char('f') => game.step(),
                        KeyCode::Char('s') => game.get_grid().save("./data/backup".into()),
                        KeyCode::Char('l') => *game = GameOfLife::from_grid(Grid::load("./backup".into()).unwrap()),
                        KeyCode::Char('C') => *game = GameOfLife::from_grid(Grid::new(50, 50)),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() -> Result<()>{
    ConwayApp{game: GameOfLife::from_grid(Grid::new(100, 100)),
        running: true,
        paused: true
    }.run()
}
