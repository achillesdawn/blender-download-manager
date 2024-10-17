use crate::BlenderVersion;
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect, Stylize},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{block::Title, Block, Padding, Paragraph, Widget},
};

use super::StateRef;

mod utils;

pub struct FileListWidget {
    state: StateRef,

    files: Vec<BlenderVersion>,
    selected: usize,
    len: usize,
}

impl FileListWidget {
    pub fn new(state: StateRef) -> Self {
        let mut file_list_widget = FileListWidget {
            state,

            len: 0,
            files: Vec::new(),
            selected: 0,
        };

        file_list_widget.refresh_local();
        file_list_widget
    }

    fn refresh_local(&mut self) {
        let config = self.state.read().unwrap().config.clone();

        let file_list = utils::check_downloaded(&config).unwrap();
        let files = utils::parse_downloaded(file_list);

        self.len = files.len();
        self.files = files;
    }
}

impl FileListWidget {
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
}

impl Widget for &FileListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("local").alignment(Alignment::Center);

        let mut block = Block::bordered()
            .title(title)
            .border_set(border::ROUNDED)
            .padding(Padding::uniform(1));

        match self.state.read().unwrap().active_widget {
            super::ActiveWidget::FileListWidget => {
                block = block.magenta();
            }
            super::ActiveWidget::RemoteWidget => {
                block = block.cyan();
            }
        }

        let lines: Vec<Line> = self
            .files
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

                let branch_span = Span::raw(&version.branch);

                let mut line = Line::from(vec![version_span, release_span, branch_span]);

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
