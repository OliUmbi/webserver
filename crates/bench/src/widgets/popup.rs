use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Text, Widget};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, StatefulWidget};

pub struct Popup {
    title: String,
    width: Constraint,
    height: Constraint,
}

pub struct PopupState {

}

impl Popup {
    pub fn new(title: impl Into<String>, width: Constraint, height: Constraint) -> Self {
        Self {
            title: title.into(),
            width,
            height
        }
    }
}

impl StatefulWidget for Popup {
    type State = PopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = area.centered(self.width, self.height);

        let block = Block::default().title(self.title).borders(Borders::ALL).on_dark_gray();

        let area_inner = block.inner(area);

        Clear.render(area, buf);
        block.render(area, buf);

    }
}
