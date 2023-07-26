use std::{collections::VecDeque, sync::Arc, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};

pub struct ProgressOutput {
    lines: Arc<Mutex<VecDeque<String>>>,
    sub: Arc<Mutex<Vec<ProgressBar>>>,
    wrapper: MultiProgress,
    root: ProgressBar,
}

impl ProgressOutput {
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
            lines: Arc::new(Mutex::new(VecDeque::with_capacity(max_log_lines))),
            sub: Arc::new(Mutex::new(Vec::new())),
            wrapper,
            root,
        }
    }

    pub async fn add_subtask<T>(
        &mut self,
        start_message: T,
        end_message: T,
        steps: u64,
    ) -> SubtaskHandle
    where
        T: Into<String> + Send + 'static,
    {
        self.root
            .set_length(self.root.length().unwrap_or_default() + 1);
        let child = ProgressBar::new(steps);

        let child = self.wrapper.add(child);
        child.enable_steady_tick(Duration::from_millis((1000 / 15) as u64));
        child.set_message(start_message.into());
        child.set_style(
            ProgressStyle::with_template("{wide_msg:.white} {elapsed_precise}").unwrap(),
        );

        self.sub.lock().await.push(child);

        let (tx, mut rx) = mpsc::channel::<String>(100);

        let lines = self.lines.clone();
        let sub = self.sub.clone();

        let handle = tokio::spawn(async move {
            let mut guard = sub.lock().await;
            let last = guard.last_mut().unwrap();

            while let Some(line) = rx.recv().await {
                let mut lines = lines.lock().await;

                if line.trim().is_empty() {
                    continue;
                }

                if lines.len() == lines.capacity() {
                    lines.pop_front();
                    lines.push_back(line);
                } else {
                    lines.push_back(line);
                }

                let lines: Vec<String> = lines.iter().cloned().collect();
                let message = lines.join("\n");

                last.set_message(message);
            }

            last.finish_with_message(end_message.into());
        });

        SubtaskHandle { handle, tx }
    }
}

pub struct SubtaskHandle {
    tx: mpsc::Sender<String>,
    handle: JoinHandle<()>,
}

impl SubtaskHandle {
    pub async fn send_line(&self, line: impl Into<String>) {
        self.tx.send(line.into()).await.unwrap();
    }

    pub async fn wait(self) {
        drop(self.tx);
        self.handle.await.unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::process::Stdio;

    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command,
    };

    use super::*;

    #[tokio::test]
    async fn test_scrolling_progress() {
        let mut spo = ProgressOutput::new(4, 15, "Running".into());

        let task = spo
            .add_subtask(
                "Starting minikube cluster",
                "Successfully created minikube cluster",
                5,
            )
            .await;

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
        task.wait().await;
    }
}
