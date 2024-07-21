//! operator
//!
//! models an FM operator
use std::time::Duration;
use rodio::source::Source;

use crate::phase_generator::*;
use crate::wave_generator::*;

pub struct Operator {
    phase_gen : PhaseGenerator,
    wave_gen  : WaveGenerator,
    feedback : i32,
    feedback_level : i32
}

impl Operator {
    pub fn new() -> Operator {
        Operator {
            phase_gen : PhaseGenerator::new(),
            wave_gen  : WaveGenerator::new(),
            feedback : 0,
            feedback_level : 16
        }
    }

    pub fn set_freq(&mut self, flog2 : i32) {
        self.phase_gen.set_freq(flog2);
    }

    pub fn set_wave(&mut self, wave: WaveForm) {
        self.wave_gen.set_wave(wave);
    }

    pub fn get_sample(&mut self) -> f32 {
        let phase = self.phase_gen.update(self.feedback);
        let output = self.wave_gen.generate(phase);
        self.feedback = output / self.feedback_level;
        return output as f32 / 65536.0;
    }
}

impl Iterator for Operator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for Operator {
    fn channels(&self) -> u16 {
        return 1;
    }

     fn sample_rate(&self) -> u32 {
        return SAMPLE_FREQ; // from phase_generator
     }

     fn current_frame_len(&self) -> Option<usize> {
        return None;
     }

     fn total_duration(&self) -> Option<Duration> {
        return None;
     }
}