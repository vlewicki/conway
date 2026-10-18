use std::io::Result;

use ratatui::Frame;

pub trait App {
    fn running(&self) -> bool;
    fn draw(&self, frame: &mut Frame);
    fn tick(&mut self) -> Result<()>;

    fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal| {
            while self.running() {
                terminal.draw(|frame| self.draw(frame))?;
                self.tick()?;
            }
            Ok(())
        })
    }
}
