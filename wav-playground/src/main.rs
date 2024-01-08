use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open("../data/PinkPanther30.wav").unwrap();
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.sleep_until_end();
    
}