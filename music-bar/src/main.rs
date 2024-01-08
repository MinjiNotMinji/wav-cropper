use std::thread;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

fn main() {
    let mut elapsed = 0;
    let total_size = 100_000;
    let unit = total_size / 1_000;

    println!("\n   {} - {}", "Hype-boy", "NewJeans\n");

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.white} |{wide_bar:.red/pink}| {elapsed_precise}/{duration_precise}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    while elapsed < total_size {
        let new = min(elapsed + unit, total_size);
        elapsed = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(10));
    }
}
