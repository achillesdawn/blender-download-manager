use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Span;

use crate::BlenderVersion;

pub fn version_span(version: &BlenderVersion) -> Span<'_> {
    match &version.version {
        x if x.contains("4.2") => {
            Span::styled(format!("{x:^10}"), Style::default().bg(Color::Green))
        }
        x if x.contains("4.3") => {
            Span::styled(format!("{x:^10}"), Style::default().bg(Color::Magenta))
        }
        x => Span::styled(format!("{x:^10}"), Style::default().bg(Color::Gray)),
    }
}

pub fn release_span(version: &BlenderVersion) -> Span<'_> {
    match version.release.as_str() {
        x if x == "stable" => Span::styled(format!("{x:^11}"), Style::default().fg(Color::Green)),
        x if x == "beta" => Span::styled(format!("{x:^11}"), Style::default().fg(Color::Magenta)),

        x if x == "candidate" => Span::styled(format!("{x:^11}"), Style::default().fg(Color::Red)),
        x if x == "alpha" => Span::styled(format!("{x:^11}"), Style::default().fg(Color::Gray)),
        _ => Span::styled(String::new(), Style::default().fg(Color::Red)),
    }
}
