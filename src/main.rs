#![allow(dead_code)]

mod fp;
mod synth;

use std::time::Duration;
use fp::*;
use rodio::{OutputStream, source::Source};

fn main() {
    let note : fp::FP = fp::FP::raw(0x7_c807); // log2(110)

    let mut voice = synth::voice::Voice::new();
    
    voice.operators[0].wave_gen.waveform = synth::wave_generator::WaveForm::FullSine;
    voice.operators[0].total_level = 0;

    voice.operators[1].wave_gen.waveform = synth::wave_generator::WaveForm::FullSine;
    voice.operators[1].total_level = 4;
    voice.operators[2].phase_gen.tune = FP::from(-8);

    voice.operators[2].wave_gen.waveform = synth::wave_generator::WaveForm::Square;
    voice.operators[2].total_level = 64;
    voice.operators[2].phase_gen.tune = FP::from(0.5833);

    voice.operators[3].wave_gen.waveform = synth::wave_generator::WaveForm::FullSine;
    voice.operators[3].total_level = 255;

    voice.set_freq(note);
    voice.algorithm = 1;
    voice.operators[0].phase_gen.tune = FP_ONE;

/*
    for i in 0..1024 {
        println!("{i}; {}", voice.get_sample());
    }
*/
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(voice.convert_samples());
    
    std::thread::sleep(Duration::from_secs(3));
}
