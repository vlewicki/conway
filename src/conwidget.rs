use ratatui::{style::Stylize, text::Span, widgets::StatefulWidget};

use crate::{conway::GameOfLife, gamestate::GameState};

fn alive_cell(state: &GameState) -> Span<'_> {
    let span = Span::from("  ");
    if state.paused {
        span.on_gray()
    } else {
        span.on_white()
    }
}

impl StatefulWidget for &GameOfLife {
    type State = GameState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        let grid = self.get_grid();
        let (h, w): (usize, usize) = grid.size();
        let (left, top) = (area.left(), area.top());
        let h_max = u16::try_from(h).unwrap_or(u16::MAX);
        let w_max = u16::try_from(w).unwrap_or(u16::MAX);
        // stderr().write(format!("{:?}\n", (h_max, w_max, area.as_size())).as_bytes()).unwrap();
        for r in 0..h_max.min(area.height) {
            for c in 0..w_max.min(area.width / 2) {
                let span = match grid.get(r as usize, c as usize) {
                    crate::conway::Cell::Alive => alive_cell(state),
                    crate::conway::Cell::Dead => Span::from("  ").on_black(),
                };
                let _ = buf.set_span(left + 2 * c, top + r, &span, 2);
            }
        }
    }
}
