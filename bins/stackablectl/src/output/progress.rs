use std::{borrow::Cow, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct ProgressOutput {
    wrapper: MultiProgress,
    bars: Vec<ProgressBar>,
}

impl ProgressOutput {
    pub fn new() -> Self {
        Self {
            wrapper: MultiProgress::new(),
            bars: Vec::new(),
        }
    }

    pub fn add(&mut self, message: impl Into<Cow<'static, str>>, initial_len: Option<u64>) {
        // Add one more step to the parent progress bar
        if let Some(last) = self.bars.last_mut() {
            last.update(|s| s.set_len(s.len().unwrap_or_default() + 1));
        }

        let pb = ProgressBar::new(initial_len.unwrap_or(1));
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:} [{elapsed_precise}] {msg:.blue} [{wide_bar}] [{pos}/{len}]",
            )
            .unwrap()
            .progress_chars("=>-"),
        );
        pb.set_message(message);

        let pb = self.wrapper.add(pb);
        self.bars.push(pb);
    }

    pub fn tick(&mut self) {
        if let Some(last) = self.bars.last_mut() {
            let len = last.length().unwrap_or_default();
            let pos = last.position();

            if pos != len - 1 {
                last.inc(1);
                return;
            }

            self.done();
        }
    }

    pub fn done(&mut self) {
        match self.bars.pop() {
            Some(pb) => {
                if self.bars.is_empty() {
                    pb.finish_with_message("Done!")
                } else {
                    pb.finish_and_clear();
                    self.wrapper.remove(&pb);
                    self.tick();
                }
            }
            None => (),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_progress() {
        let mut po = ProgressOutput::new();

        po.add("Total progress", Some(1));
        po.add("Sub task", Some(3));

        std::thread::sleep(Duration::from_secs(5));
        po.tick();
        std::thread::sleep(Duration::from_secs(1));
        po.tick();
        std::thread::sleep(Duration::from_secs(1));
        po.tick();

        std::thread::sleep(Duration::from_secs(1));
        po.tick();
    }
}
