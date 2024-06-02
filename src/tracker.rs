use std::{
    io::{self, Write},
    time::Instant,
};

pub struct ProgressTracker {
    content_length: usize,
    pub total_read: usize,
    last_read: usize,
    incremental: usize,
    percentage: f32,
    start: Instant,
    pub elapsed: Instant,
}

impl ProgressTracker {
    pub fn new(content_length: usize) -> Self {
        ProgressTracker {
            content_length,
            total_read: 0,
            last_read: 0,
            incremental: 0,
            percentage: 0.0,
            start: Instant::now(),
            elapsed: Instant::now(),
        }
    }

    pub fn update(&mut self, total_read: usize) {
        self.total_read += total_read;
        self.incremental = total_read - self.last_read;
        self.last_read = total_read;
        self.percentage = total_read as f32 / self.content_length as f32 * 100.0;

        let incremental_time = self.elapsed.elapsed().as_secs_f32();
        self.elapsed = Instant::now();
        let mut kbs = self.incremental as f32 / incremental_time / 1000.0;

        if kbs > 1000.0 {
            kbs /= 1000.0;
            print!(
                "\r{:>5.1}% | {:>5.1} mb/s | {}s ",
                self.percentage,
                kbs,
                self.start.elapsed().as_secs()
            );
        } else {
            print!(
                "\r{:>5.1}% | {:>5.0} kb/s | {}s ",
                self.percentage,
                kbs,
                self.start.elapsed().as_secs()
            );
        }

        io::stdout().flush().unwrap();
    }
}
