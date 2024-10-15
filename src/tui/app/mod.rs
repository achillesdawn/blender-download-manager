use std::{
    io::{self},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use remote::{get_links, RemoteWidget};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::BlenderVersion;
use futures::StreamExt;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    widgets::Widget,
};

use crate::config::Config;

mod file_list;
mod help;
mod remote;

use file_list::FileListWidget;
use help::HelpWidget;

use super::utils::Tui;

enum Message {
    GetLinksResult(Vec<BlenderVersion>),
    Error(String),
}

pub struct TuiApp {
    done: bool,

    config: Config,

    events_tx: Arc<Sender<Message>>,
    events: Receiver<Message>,

    file_widget: FileListWidget,
    help_widget: HelpWidget,
    remote_widget: RemoteWidget,

    text: String,
}

impl TuiApp {
    pub fn new(config: Config, downloaded: Vec<BlenderVersion>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<Message>(1);

        let file_widget = file_list::FileListWidget::new(downloaded);
        let help_widget = help::HelpWidget::new();
        let remote_widget = remote::RemoteWidget::new(config.clone());

        TuiApp {
            done: false,
            text: String::from_str("loading...").unwrap(),
            config,

            events_tx: Arc::new(tx),
            events: rx,

            file_widget,
            help_widget,
            remote_widget,
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
                    self.handle_events(event).await.unwrap();
                },
                Some(message) = self.events.recv() => {
                    self.handle_messages(message);
                }

            }
        }

        Ok(())
    }

    fn handle_messages(&mut self, message: Message) {
        match message {
            Message::GetLinksResult(links) => {
                self.remote_widget.set_available(links);
            }
            Message::Error(err) => {
                self.remote_widget.set_message(err);
            }
        }
    }

    async fn handle_events(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {}
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up => {
                        self.file_widget.increment_active_selection();
                    }
                    KeyCode::Down => {
                        self.file_widget.decrement_active_selection();
                    }
                    KeyCode::Left => {}
                    KeyCode::Right => {}
                    KeyCode::Char(' ') => {}
                    KeyCode::Char('q') => {
                        self.done = true;
                    }
                    KeyCode::Enter => {
                        self.remote_widget
                            .set_message("checking available versions...");

                        let config = self.config.clone();
                        let tx = self.events_tx.clone();

                        tokio::spawn(async move {
                            let versions = get_links(config).await;
                            match versions {
                                Ok(versions) => {
                                    tx.send(Message::GetLinksResult(versions)).await.unwrap();
                                }
                                Err(err) => {
                                    tx.send(Message::Error(err.to_string())).await.unwrap();
                                }
                            }
                        });
                    }
                    _ => {}
                };
            }
            _ => {}
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
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
            .constraints([Constraint::Percentage(30), Constraint::default()])
            .split(main_layout[0]);

        self.file_widget.render(split_layout[0], buf);
        self.remote_widget.render(split_layout[1], buf);
        self.help_widget.render(main_layout[1], buf);
    }
}
