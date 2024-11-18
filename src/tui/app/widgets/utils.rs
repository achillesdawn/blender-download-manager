use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Span;

use crate::BlenderVersion;

pub fn release_span(version: &BlenderVersion) -> Span<'_>  {
    match version.release.as_str() {
        x if x == "stable" => {
            Span::styled(format!("{x:^11}"), Style::default().fg(Color::Green))
        }
        x if x == "beta" => {
            Span::styled(format!("{x:^11}"), Style::default().fg(Color::Magenta))
        }
    
        x if x == "candidate" => {
            Span::styled(format!("{x:^11}"), Style::default().fg(Color::Red))
        }
        x if x == "alpha" => {
            Span::styled(format!("{x:^11}"), Style::default().fg(Color::Gray))
        }
        _ => Span::styled(String::new(), Style::default().fg(Color::Red)),
    }
}
