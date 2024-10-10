use std::{
    io::{self},
    str::FromStr,
    time::Duration,
};

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};

use futures::StreamExt;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Padding, Paragraph, Widget,
    },
};

use crate::Config;

mod file_list_widget;
mod help_widget;

use file_list_widget::FileListWidget;
use help_widget::HelpWidget;

use super::utils::Tui;

pub struct TuiApp {
    done: bool,

    config: Config,

    file_widget: FileListWidget,
    help_widget: HelpWidget,
    text: String,
}

impl TuiApp {
    pub fn new(config: Config, downloaded: Vec<String>) -> Self {
        let file_widget = file_list_widget::FileListWidget::new(downloaded);
        let help_widget = help_widget::HelpWidget::new();
        TuiApp {
            done: false,
            text: String::from_str("loading...").unwrap(),
            config,

            file_widget,
            help_widget,
        }
    }

    pub async fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        let period = Duration::from_secs_f32(1.0 / 30.0);
        let mut interval = tokio::time::interval(period);

        let mut events = EventStream::new();

        while !self.done {
            tokio::select! {
                _ = interval.tick() => {
                    terminal.draw(|frame| {
                        self.render_frame(frame);
                    })?;
                },
                Some(Ok(event)) = events.next() => {
                    self.handle_events(event).unwrap();
                }
            }
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {}
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Left => {}
                    KeyCode::Right => {}
                    KeyCode::Char(' ') => {}
                    KeyCode::Char('q') => {
                        self.done = true;
                    }
                    _ => {}
                };
            }
            _ => {}
        }
        Ok(())
    }
}

impl Widget for &TuiApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Max(1)])
            .split(area);

        let split_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50)])
            .split(main_layout[0]);

        self.file_widget.render(split_layout[0], buf);
        self.help_widget.render(main_layout[1], buf);
    }
}
