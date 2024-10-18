use std::{fs::File, path::PathBuf, str::FromStr};

use crate::{config::Config, BlenderVersion};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect, Stylize},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Widget},
};

use super::StateRef;

pub struct RemoteWidget {
    state: StateRef,

    checked: bool,
    pub select_mode: bool,

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
            select_mode: false,

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

pub fn get_file(version: &BlenderVersion, config: Config) -> File{
    let filename = version.link.split("daily/").nth(1).unwrap();

    let mut path = PathBuf::from_str(&config.path).unwrap();
    path.push(filename);

    // if downloaded.contains(&path.with_extension("").with_extension("")) {
    //     println!("{} Already at Latest version", version.version);
    // }

    if path.exists() {
        println!("{} Already downloaded", version.version);
    }

    let file = std::fs::File::create(&path).unwrap();
    file

    // let download_result = crate::getter::download(&version.link, &mut file).await;

    // if download_result.is_err() {
    //     println!("Download Error: {}", download_result.err().unwrap());
    // } else {
    //     drop(file);

    //     crate::extract_and_clean(path, &config);
    // }
}

impl RemoteWidget {
    pub fn increment_active_selection(&mut self) {
        self.selected += 1;

        if self.selected >= self.len {
            self.selected = 0;
        }
    }

    pub fn decrement_active_selection(&mut self) {
        if self.len == 0 {
            return;
        }

        if self.selected == 0 {
            self.selected = self.len - 1;
        } else {
            self.selected = self.selected.saturating_sub(1);
        }
    }

    pub fn set_available(&mut self, links: Vec<BlenderVersion>) {
        self.len = links.len();
        self.select_mode = true;
        self.checked = true;
        self.available = links;
        self.set_message("ready");
    }

    pub fn set_message(&mut self, message: impl ToString) {
        self.message = message.to_string();
    }

    pub fn download_selected(&mut self) -> BlenderVersion {
        let selected = self.available.get(self.selected).unwrap().clone();
        self.set_message(format!("downloading {}", selected.link));
        selected
    }
}

impl Widget for &RemoteWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Max(4)])
            .split(area);

        let block = Block::bordered()
            .title(" status ")
            .border_set(border::ROUNDED)
            .padding(Padding::horizontal(1));

        let text = Text::from(self.message.clone());

        let p = Paragraph::new(text)
            .wrap(ratatui::widgets::Wrap { trim: false })
            .left_aligned()
            .block(block);

        p.render(layout[1], buf);

        let mut block = Block::bordered()
            .title(" remote ")
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

        p.render(layout[0], buf);
    }
}
