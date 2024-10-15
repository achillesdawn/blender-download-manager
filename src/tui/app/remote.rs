use crate::{config::Config, BlenderVersion};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect, Stylize},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{block::Title, Block, Padding, Paragraph, Widget},
};

use super::StateRef;

pub struct RemoteWidget {
    state: StateRef,

    checked: bool,

    available: Vec<BlenderVersion>,
    len: usize,

    selected: usize,

    message: String,
}

impl RemoteWidget {
    pub fn new(state: StateRef) -> Self {
        RemoteWidget {
            state,

            checked: false,

            len: 0,
            available: Vec::new(),

            selected: 0,
            message: "press enter to check available versions".into(),
        }
    }
}

pub async fn get_links(config: Config) -> Result<Vec<BlenderVersion>, String> {
    match crate::getter::get_links(&config).await {
        Ok(versions) => Ok(versions),
        Err(err) => Err(err.to_string()),
    }
}

impl RemoteWidget {
    pub fn increment_active_selection(&mut self) {
        self.selected += 1;

        if self.selected >= self.len {
            self.selected = 0;
        }
    }

    pub fn decrement_active_selection(&mut self) {
        if self.selected == 0 {
            self.selected = self.len - 1;
        } else {
            self.selected = self.selected.saturating_sub(1);
        }
    }

    pub fn set_available(&mut self, links: Vec<BlenderVersion>) {
        self.len = links.len();
        self.checked = true;
        self.available = links;
    }

    pub fn set_message(&mut self, message: impl ToString) {
        self.message = message.to_string();
    }
}

impl Widget for &RemoteWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("remote").alignment(Alignment::Center);

        let mut block = Block::bordered()
            .title(title)
            .border_set(border::ROUNDED)
            .padding(Padding::uniform(1));

        match self.state.read().unwrap().active_widget {
            super::ActiveWidget::FileListWidget => {
                block = block.cyan();
            }
            super::ActiveWidget::RemoteWidget => {
                block = block.magenta();
            }
        }

        if self.available.len() == 0 {
            let text = Text::from(self.message.clone());
            let p = Paragraph::new(text)
                .wrap(ratatui::widgets::Wrap { trim: false })
                .left_aligned()
                .block(block);

            p.render(area, buf);
            return;
        }

        let lines: Vec<Line> = self
            .available
            .iter()
            .enumerate()
            .map(|(idx, version)| {
                let version_span = match &version.version {
                    x if x.contains("4.2") => {
                        Span::styled(format!("{x:^10}"), Style::default().bg(Color::Green))
                    }
                    x if x.contains("4.3") => {
                        Span::styled(format!("{x:^10}"), Style::default().bg(Color::Magenta))
                    }
                    x => Span::styled(format!("{x:^10}"), Style::default().bg(Color::Gray)),
                };

                let release_span = match version.release.as_str() {
                    x if x == "stable" => {
                        Span::styled(format!("{x:^10}"), Style::default().fg(Color::Green))
                    }
                    x if x == "beta" => {
                        Span::styled(format!("{x:^10}"), Style::default().fg(Color::Magenta))
                    }
                    x if x == "alpha" => {
                        Span::styled(format!("{x:^10}"), Style::default().fg(Color::Gray))
                    }
                    _ => Span::styled(String::new(), Style::default().fg(Color::Red)),
                };

                let mut line = Line::from(vec![version_span, release_span]);

                if idx == self.selected {
                    line = line
                        .into_iter()
                        .map(|s| s.patch_style(Style::default().bg(Color::LightCyan)))
                        .collect();
                }

                line
            })
            .collect();

        let text = Text::from(lines);

        let p = Paragraph::new(text)
            .wrap(ratatui::widgets::Wrap { trim: false })
            .left_aligned()
            .block(block);

        p.render(area, buf);
    }
}
