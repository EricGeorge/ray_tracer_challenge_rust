use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40}] {pos}/{len} ({percent}%)")
            .unwrap(),
    );
    pb
}
