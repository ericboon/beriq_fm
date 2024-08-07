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
    voice.operators[0].phase_gen.tune = FP_ONE;

    voice.operators[1].wave_gen.waveform = synth::wave_generator::WaveForm::Square;
    voice.operators[1].total_level = 32;
    voice.operators[1].phase_gen.tune = FP::from(-1);

    voice.operators[1].env_gen.attack_rate = FP::from(0.2);
    voice.operators[1].env_gen.decay_rate = FP::from(0.01);
    voice.operators[1].env_gen.sustain_level = FP::from(0.59);
    voice.operators[1].env_gen.release_rate = FP::from(0.005);
    voice.operators[1].env_gen.is_sustained = true;

    voice.operators[2].wave_gen.waveform = synth::wave_generator::WaveForm::FullSine;
    voice.operators[2].total_level = 63;
    voice.operators[2].phase_gen.tune = FP::from(0.5833);

    voice.operators[2].env_gen.attack_rate = FP::from(0.999);
    voice.operators[2].env_gen.decay_rate = FP::from(0.001);
    voice.operators[2].env_gen.sustain_level = FP::from(0.3);
    voice.operators[2].env_gen.release_rate = FP::from(0.005);
    voice.operators[2].env_gen.is_sustained = false;

    voice.operators[3].wave_gen.waveform = synth::wave_generator::WaveForm::FullSine;
    voice.operators[3].total_level = 255;
    voice.operators[3].phase_gen.tune = FP::from(1);

    voice.operators[3].env_gen.attack_rate = FP::from(0.999);
    voice.operators[3].env_gen.decay_rate = FP::from(0.002);
    voice.operators[3].env_gen.sustain_level = FP::from(0.5);
    voice.operators[3].env_gen.release_rate = FP::from(0.005);
    voice.operators[3].env_gen.is_sustained = false;
    voice.operators[3].feedback_level = 100;

    voice.algorithm = 1;

/*
    for i in 0..1024 {
        println!("{i}; {}", voice.get_sample());
    }
*/

voice.note_on(note);

    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(voice.convert_samples());
    
    //voice.note_on(note);
    //voice.note_off();
    std::thread::sleep(Duration::from_secs(3));

}
