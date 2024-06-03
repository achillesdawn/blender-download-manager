use std::{
    io::{self, Write},
    time::Instant,
};

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

    fn display(&self, mut kbs: f32) {
        if kbs > 1000.0 {
            kbs /= 1000.0;
            print!(
                "\r{:>5.1}% | {:>5.1} mb/s | {}s | estimated {} min ",
                self.percentage,
                kbs,
                self.start.elapsed().as_secs(),
                self.estimated()
            );
        } else {
            print!(
                "\r{:>5.1}% | {:>5.0} kb/s | {}s | estimated {} min ",
                self.percentage,
                kbs,
                self.start.elapsed().as_secs(),
                self.estimated()
            );
        }

        io::stdout().flush().unwrap();
    }
}
