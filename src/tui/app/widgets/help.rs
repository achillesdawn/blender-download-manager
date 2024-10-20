use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

pub struct HelpWidget {
    message: String,
}

impl HelpWidget {
    pub fn new() -> Self {
        HelpWidget {
            message: "help".to_owned(),
        }
    }
}

impl Widget for &HelpWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let p = Paragraph::new(self.message.clone())
            .centered()
            .style(Style::new().bg(Color::Cyan));

        p.render(area, buf);
    }
}
