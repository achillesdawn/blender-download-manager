use std::{
    io::{self},
    rc::Rc,
    sync::{Arc, RwLock},
    time::Duration,
};

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};

use tokio::sync::mpsc::Receiver;

use futures::StreamExt;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    widgets::Widget,
};

use crate::config::Config;

use super::{utils::Tui, Message, TxMessage};

mod widgets;

use widgets::{
    files::FileListWidget,
    help::HelpWidget,
    remote::{extract_and_clean, get_file, get_links, RemoteWidget},
};

mod state;
use state::{ActiveWidget, State, StateRef};

pub struct TuiApp {
    done: bool,

    state: StateRef,

    events_tx: TxMessage,
    events: Receiver<Message>,

    file_widget: FileListWidget,
    help_widget: HelpWidget,
    remote_widget: RemoteWidget,
}

impl TuiApp {
    pub fn new(config: Config) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<Message>(1);

        let state = Rc::new(RwLock::new(State {
            config,
            active_widget: ActiveWidget::FileListWidget,
        }));

        let help_widget = HelpWidget::new();
        let file_widget = FileListWidget::new(state.clone());
        let remote_widget = RemoteWidget::new(state.clone());

        TuiApp {
            done: false,

            events_tx: Arc::new(tx),
            events: rx,

            file_widget,
            help_widget,
            remote_widget,

            state,
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
            Message::Links(links) => {
                self.remote_widget.set_available(links);
            }
            Message::Error(err) => {
                self.remote_widget.set_message(err);
            }
            Message::VersionUpdate(s) => {
                self.remote_widget.set_message(s);
            }
            Message::VersionResult(path) => {
                self.remote_widget.set_message("downloaded...extracting...");

                let config = self.state.read().unwrap().config.clone();
                let handle = tokio::task::spawn_blocking(move || {
                    extract_and_clean(path, &config);
                });

                let tx = self.events_tx.clone();
                tokio::spawn(async move {
                    handle.await.unwrap();
                    tx.send(Message::ExtractResult).await.unwrap();
                });
            }

            Message::ExtractResult => {
                self.file_widget.refresh_local();
                self.remote_widget.set_message("ready");
            }
        }
    }

    fn handle_events(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {}
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up => match self.state.read().unwrap().active_widget {
                        ActiveWidget::FileListWidget => {
                            self.file_widget.decrement_active_selection();
                        }
                        ActiveWidget::RemoteWidget => {
                            self.remote_widget.decrement_active_selection();
                        }
                    },
                    KeyCode::Down => match self.state.read().unwrap().active_widget {
                        ActiveWidget::FileListWidget => {
                            self.file_widget.increment_active_selection();
                        }
                        ActiveWidget::RemoteWidget => {
                            self.remote_widget.increment_active_selection();
                        }
                    },
                    KeyCode::Left => {
                        let mut state = self.state.write().unwrap();

                        state.active_widget = ActiveWidget::FileListWidget;
                    }
                    KeyCode::Right => {
                        let mut state = self.state.write().unwrap();

                        state.active_widget = ActiveWidget::RemoteWidget;
                    }
                    KeyCode::Char(' ') => {}
                    KeyCode::Char('q') => {
                        self.done = true;
                    }
                    KeyCode::Enter => {
                        let active_widget = &self.state.read().unwrap().active_widget;
                        match active_widget {
                            ActiveWidget::FileListWidget => {
                                self.remote_widget
                                    .set_message("checking available versions...");

                                let config = self.state.read().unwrap().config.clone();
                                let tx = self.events_tx.clone();

                                tokio::spawn(async move {
                                    let versions = get_links(config).await;
                                    match versions {
                                        Ok(versions) => {
                                            tx.send(Message::Links(versions))
                                                .await
                                                .unwrap();
                                        }
                                        Err(err) => {
                                            tx.send(Message::Error(err.to_string())).await.unwrap();
                                        }
                                    }
                                });
                            }
                            ActiveWidget::RemoteWidget => {
                                if self.remote_widget.select_mode {
                                    let version = self.remote_widget.download_selected();
                                    let config = self.state.read().unwrap().config.clone();
                                    let tx = self.events_tx.clone();

                                    tokio::spawn(async move {
                                        let (mut file, path) = get_file(&version, config);
                                        crate::getter::download_with_tx(
                                            &version.link,
                                            &mut file,
                                            path,
                                            tx,
                                        )
                                        .await;
                                    });
                                } else {
                                    self.remote_widget
                                        .set_message("checking available versions...");

                                    let config = self.state.read().unwrap().config.clone();
                                    let tx = self.events_tx.clone();

                                    tokio::spawn(async move {
                                        let versions = get_links(config).await;
                                        match versions {
                                            Ok(versions) => {
                                                tx.send(Message::Links(versions))
                                                    .await
                                                    .unwrap();
                                            }
                                            Err(err) => {
                                                tx.send(Message::Error(err.to_string()))
                                                    .await
                                                    .unwrap();
                                            }
                                        }
                                    });
                                }
                            }
                        }
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
            .constraints([Constraint::Percentage(40), Constraint::default()])
            .split(main_layout[0]);

        self.file_widget.render(split_layout[0], buf);
        self.remote_widget.render(split_layout[1], buf);
        self.help_widget.render(main_layout[1], buf);
    }
}
