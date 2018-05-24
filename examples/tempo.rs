extern crate aubio;

use std::env;

use aubio::source::Source;
use aubio::tempo::Tempo;

fn main() {
    const HOP_SIZE: usize = 512;

    let argv = env::args().collect::<Vec<_>>();
    let mut source = Source::open(&argv[1], 0, HOP_SIZE).expect("Source::open");
    println!("{:?}", source);
    println!("sample_rate: {}", source.sample_rate());

    let mut tempo = Tempo::new(1024, HOP_SIZE, source.sample_rate()).expect("Tempo::new");

    while let Some(fvec) = source.read() {
        tempo.execute(&fvec);
        println!("{:?}", tempo.bpm());
    }
}
