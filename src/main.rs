mod conway;
mod conwidget;
mod constore;
mod gamestate;

use std::{io::{Error, stdout}, time::Duration};

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEvent}, execute};
use ratatui::{DefaultTerminal, Frame};

use crate::{conway::{Cell::{self}, GameOfLife, Grid}, gamestate::GameState};

fn get_grid(grid: Vec<Vec<u8>>) -> Grid<Cell> {
    let rows = grid.len();
    let cols = grid[0].len();
    let grid: Vec<Cell> = grid
        .into_iter()
        .flatten()
        .map(|value| if value == 0 { Cell::Dead } else { Cell::Alive })
        .collect();
    assert_eq!(grid.len(), rows * cols);
    Grid::from_slice(grid.as_slice(), rows, cols)
}

fn render(widget: &GameOfLife, frame: &mut Frame, state: &mut GameState){
    frame.render_stateful_widget(widget, frame.area(), state);
}

fn main_loop(terminal: &mut DefaultTerminal) -> Result<(), Error>{
    execute!(stdout(), EnableMouseCapture).expect("mouse not captured");
    // let grid1 = get_grid(vec![
    //     vec![0, 1, 1, 0, 0, 0],
    //     vec![1, 0, 0, 1, 0, 0],
    //     vec![0, 1, 0, 1, 0, 0],
    //     vec![0, 0, 1, 0, 0, 0],
    //     vec![0, 0, 0, 0, 0, 0],
    //     vec![0, 0, 0, 0, 0, 0],
    //     vec![0, 0, 1, 1, 1, 0],
    //     vec![0, 1, 1, 1, 0, 0],
    //     vec![0, 0, 0, 0, 0, 0],
    // ]);
    // 
    // let mut game = GameOfLife::from_grid(grid1.clone());
    let mut game = GameOfLife::from_grid(Grid::new(50, 50));
    let mut running = true;
    let mut pause = true;
    while running {
        let _ = terminal.draw(
            |frame| render(&game, frame, &mut GameState { paused: pause })
        ).unwrap();
        if !pause{
            game.step();
        }
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Mouse(MouseEvent{kind, column, row, modifiers}) => {
                    match kind {
                        event::MouseEventKind::Down(event::MouseButton::Left) => {
                            let (h, w) = game.get_grid().size();
                            let (x, y) = (((column / 2) as usize), (row as usize));
                            if y < h && x < w {
                                game.toggle(y, x);
                            }
                        }
                        _ => {}
                    }
                }
                Event::Key(KeyEvent{code, modifiers, kind, state}) => {
                    match code {
                        KeyCode::Char('q') => running = false,
                        KeyCode::Char('p') => pause = !pause,
                        KeyCode::Char('f') => game.step(),
                        KeyCode::Char('s') => game.get_grid().save("./backup".into()),
                        KeyCode::Char('l') => game = GameOfLife::from_grid(Grid::load("./backup".into()).unwrap()),
                        KeyCode::Char('C') => game = GameOfLife::from_grid(Grid::new(50, 50)),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    execute!(stdout(), DisableMouseCapture).expect("mouse not discaptured");
    Ok(())
}

fn main() -> Result<(), Error>{
    ratatui::run(
        |terminal| main_loop(terminal)
    )
}
