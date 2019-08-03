extern crate aubio;

use std::env;

use aubio::source::Source;
use aubio::sink::Sink;

fn main() {
    const HOP_SIZE: usize = 512;
    const SAMPLE_RATE: usize = 44100;
    let argv = env::args().collect::<Vec<_>>();

    let mut source = Source::open(&argv[1], SAMPLE_RATE, HOP_SIZE).unwrap();
    let mut sink = Sink::open(&argv[2], SAMPLE_RATE).unwrap();

    while let Some(buf) = source.read() {
        sink.write(&buf);
    }
}