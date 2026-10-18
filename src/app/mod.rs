use std::io::{Error, Result};

use ratatui::{Frame, widgets::Widget};
use thiserror::Error;

pub trait App{
    fn running(&self) -> bool;
    fn draw(&self, frame: &mut Frame);
    fn tick(&mut self) -> Result<()>;
    
    fn run(&mut self) -> Result<()>{
        ratatui::run(
            |terminal| {
                while self.running(){
                    terminal.draw(|frame| self.draw(frame))?;
                    self.tick()?;
                }
                Ok(())
            }
        )
    }
}