use std::{
    io::{self, stdout, Stdout},
    str::FromStr,
    time::Duration,
};

use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use futures::StreamExt;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Terminal,
};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub struct TuiApp {
    done: bool,

    text: String,
}

impl TuiApp {
    pub fn new() -> Self {
        TuiApp {
            done: false,
            text: String::from_str("hello tui").unwrap(),
        }
    }

    pub async fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        let period = Duration::from_secs_f32(1.0 / 60.0);
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
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Max(1)])
            .split(area);

        let title = Title::from(self.text.as_str()).alignment(Alignment::Center);

        let commands = Title::from(Line::from(vec![format!(".:: Frame ::.",).bold()]))
            .alignment(Alignment::Center)
            .position(Position::Bottom);

        let block = Block::bordered()
            .title(title)
            .title(commands)
            .border_set(border::ROUNDED)
            .cyan();

        // let frame = self.frames.get(self.current_frame).unwrap();
        let p = Paragraph::new("hello test")
            .wrap(ratatui::widgets::Wrap { trim: false })
            .centered()
            .block(block);

        p.render(layout[0], buf);

        let p = Paragraph::new("help text")
            .centered()
            .style(Style::new().bg(Color::Cyan));

        p.render(layout[1], buf);
    }
}
