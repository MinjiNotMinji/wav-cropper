use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use std::path::Path;

fn main() {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file_path = "../data/PinkPanther30.wav";
    let file = File::open(file_path).unwrap();
    let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();

    let source = Decoder::new(file).unwrap();
    let total_duration = source.total_duration().unwrap();
    sink.append(source);

    thread::spawn(move || {
        println!("\n   {}\n", file_name);

        let total_duration_ms = total_duration.as_millis() as u64;
        let step = 10;
        let mut elapsed = 0;

        let pb = ProgressBar::new(total_duration_ms);

        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.white} |{wide_bar:.red/pink}| {elapsed_precise} | {duration_precise}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        pb.wrap_read(File::open(file_path).unwrap());

        while elapsed < total_duration_ms {
            elapsed = elapsed + step;
            pb.set_position(elapsed);
            thread::sleep(Duration::from_millis(step));
        }
    });

    // Wait for the music playback to finish
    sink.sleep_until_end();

}
    