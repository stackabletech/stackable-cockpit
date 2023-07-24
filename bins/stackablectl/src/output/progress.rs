use std::{
    borrow::Cow,
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::{sync::mpsc, task::JoinHandle};

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
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:} [{elapsed_precise}] {msg:.blue} [{wide_bar}] [{pos}/{len}]",
            )
            .unwrap()
            .progress_chars("=>-"),
        );
        pb.enable_steady_tick(Duration::from_millis(100));

        let pb = self.wrapper.add(pb);
        pb.set_message(message);
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

    pub fn tick_with_message(&mut self, message: impl Into<Cow<'static, str>>) {
        self.tick();
        self.update_message(message);
    }

    pub fn update_message(&mut self, message: impl Into<Cow<'static, str>>) {
        if let Some(last) = self.bars.last_mut() {
            last.set_message(message)
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

pub struct ScrollingProgressOutput {
    sub: Arc<Mutex<Vec<ProgressBar>>>,
    lines: VecDeque<String>,
    wrapper: MultiProgress,
    root: ProgressBar,
}

impl ScrollingProgressOutput {
    pub fn new(max_log_lines: usize, fps: usize, initial_message: String) -> Self {
        let wrapper = MultiProgress::new();
        let dur = Duration::from_millis((1000 / fps) as u64);

        let root = ProgressBar::new_spinner();

        let root = wrapper.add(root);
        root.enable_steady_tick(dur);
        root.set_message(initial_message);
        root.set_style(
            ProgressStyle::with_template("{spinner} {msg} {elapsed} [{pos}/{len}]").unwrap(),
        );

        Self {
            lines: VecDeque::with_capacity(max_log_lines),
            sub: Arc::new(Mutex::new(Vec::new())),
            wrapper,
            root,
        }
    }

    pub fn add_subtask(
        &mut self,
        start_message: String,
        steps: u64,
        end_message: Option<String>,
    ) -> SubtaskHandle {
        let child = ProgressBar::new(steps);

        let child = self.wrapper.add(child);
        child.enable_steady_tick(Duration::from_millis((1000 / 15) as u64));
        child.set_message(start_message);
        child.set_style(
            ProgressStyle::with_template("{wide_msg:.white} {elapsed_precise}").unwrap(),
        );

        self.sub.lock().unwrap().push(child);

        let (tx, mut rx) = mpsc::channel::<String>(100);

        let sub = self.sub.clone();

        let handle = tokio::spawn(async move {
            let mut first_message_received = false;

            while let Some(line) = rx.recv().await {
                first_message_received = true;

                let mut guard = sub.lock().unwrap();
                let last = guard.last_mut().unwrap();

                if first_message_received {}

                // last.set_message(line.clone());
                last.println(line);
                // println!("{line}");
            }
        });

        SubtaskHandle {
            tx: tx.clone(),
            done: handle,
        }
    }
}

pub struct SubtaskHandle {
    tx: mpsc::Sender<String>,
    done: JoinHandle<()>,
}

impl SubtaskHandle {
    pub async fn send_line(&self, line: String) {
        self.tx.send(line).await.unwrap();
    }

    pub async fn done(self) {
        self.done.await.unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::{process::Stdio, time::Duration};

    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command,
    };

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

    #[tokio::test]
    async fn test_scrolling_progress() {
        let mut spo = ScrollingProgressOutput::new(4, 15, "Running".into());

        let task = spo.add_subtask("Start".into(), 5, None);

        let mut minikube_cmd = Command::new("minikube")
            .arg("start")
            .args(["--driver", "docker"])
            .args(["--nodes", 2.to_string().as_str()])
            .args(["-p", "test"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = minikube_cmd.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            task.send_line(line).await;
        }

        minikube_cmd.wait().await.unwrap();
        task.done().await;
    }
}
