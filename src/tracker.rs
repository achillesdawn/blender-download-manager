use std::{
    io::{self, Write},
    time::Instant,
};

use colored::{ColoredString, Colorize};

pub struct ProgressTracker {
    content_length: usize,

    pub total_read: usize,
    incremental_read: usize,

    percentage: f32,
    start: Instant,
    timer: Instant,
}

impl ProgressTracker {
    pub fn new(content_length: usize) -> Self {
        ProgressTracker {
            content_length,
            total_read: 0,
            incremental_read: 0,
            percentage: 0.0,
            start: Instant::now(),
            timer: Instant::now(),
        }
    }

    pub fn update(&mut self, read: usize) {
        self.total_read += read;
        self.percentage = self.total_read as f32 / self.content_length as f32 * 100.0;

        self.incremental_read += read;

        let incremental_time = self.timer.elapsed().as_secs_f32();

        if incremental_time > 1.0 {
            let kbs = self.incremental_read as f32 / incremental_time / 1000.0;

            self.display(kbs);

            self.timer = Instant::now();
            self.incremental_read = 0;
        }
    }

    fn estimated(&self) -> u64 {
        let rate = self.total_read as u64 / self.start.elapsed().as_secs();
        let remaining = self.content_length - self.total_read;
        let estimated = remaining as u64 / rate / 60;

        estimated
    }

    fn kbs_to_human_readable(kbs: f32) -> String {
        let kbs_string = format!("{:>5.1}", kbs).blue();
        if kbs > 1000.0 {
            format!("{} mb/s", kbs_string)
        } else {
            format!("{} kb/s", kbs_string)
        }
    }

    fn elapsed_to_human_readable(&self) -> String {
        let secs = self.start.elapsed().as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else {
            let min = secs / 60;

            format!("{}min{}s", min, secs.rem_euclid(60))
        }
    }

    fn progress_bar(&self) -> ColoredString {
        let bar = match self.percentage {
            0.0..=10.0 => "··········",
            10.0..=20.0 => "⣿·········",
            20.0..=30.0 => "⣿⣿········",
            30.0..=40.0 => "⣿⣿⣿·······",
            40.0..=50.0 => "⣿⣿⣿⣿······",
            50.0..=60.0 => "⣿⣿⣿⣿⣿·····",
            60.0..=70.0 => "⣿⣿⣿⣿⣿⣿····",
            70.0..=80.0 => "⣿⣿⣿⣿⣿⣿⣿···",
            80.0..=90.0 => "⣿⣿⣿⣿⣿⣿⣿⣿··",
            90.0..=100.0 => "⣿⣿⣿⣿⣿⣿⣿⣿⣿·",
            _ => "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
        };

        format!("{:>5.1}%  [{}]", self.percentage, bar.green()).bold()
    }

    fn display(&self, kbs: f32) {
        print!(
            "\r{} | {} | {} | estimated {} min ",
            self.progress_bar(),
            ProgressTracker::kbs_to_human_readable(kbs),
            self.elapsed_to_human_readable(),
            self.estimated().to_string().green()
        );

        io::stdout().flush().unwrap();
    }
}
