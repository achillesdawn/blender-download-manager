use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    style::Stylize,
    symbols::border,
    widgets::{block::Title, Block, Padding, Paragraph, Widget},
};

pub struct FileListWidget {
    files: Vec<String>,
}

impl FileListWidget {
    pub fn new(files: Vec<String>) -> Self {
        FileListWidget { files: files }
    }
}

impl Widget for &FileListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("local").alignment(Alignment::Center);

        let block = Block::bordered()
            .title(title)
            .border_set(border::ROUNDED)
            .padding(Padding::uniform(1))
            .cyan();

        let files = self.files.join("\n");

        let p = Paragraph::new(files)
            .wrap(ratatui::widgets::Wrap { trim: false })
            .left_aligned()
            .block(block);

        p.render(area, buf);
    }
}
