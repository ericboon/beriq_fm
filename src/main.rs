use std::time::Duration;
use rodio::{OutputStream, source::Source};

mod sin;
mod exp;
mod wave_generator;
mod phase_generator;
mod operator;

//use phase_generator::*;
use wave_generator::*;
use operator::*;

fn main() {
    const FLOG2 : i32 = 0x8c807; // log2(110) * 64k

    let mut op1 = Operator::new();
    op1.set_freq(FLOG2);
    op1.set_wave(WaveForm::FullSine);

    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(op1.convert_samples());
    
    std::thread::sleep(Duration::from_secs(5));
}
