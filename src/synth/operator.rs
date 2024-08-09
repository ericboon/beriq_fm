//! operator
//!
//! models an FM operator
use std::time::Duration;
use rodio::source::Source;

use crate::fp::*;

use super::phase_generator::*;
use super::wave_generator::*;
use super::env_generator::*;

#[derive(Debug, Copy, Clone)]
pub struct Operator {
    pub phase_gen : PhaseGenerator,
    pub wave_gen  : WaveGenerator,
    pub env_gen : EnvGenerator,

    pub total_level : u8,
    pub feedback_level : u8,

    pub mod_input : FP,
    feedback : FP,
}

impl Operator {
    pub fn new() -> Operator {
        Operator {
            phase_gen : PhaseGenerator::new(),
            wave_gen  : WaveGenerator::new(),
            env_gen   : EnvGenerator::new(),

            total_level : 255,
            feedback_level : 0,

            mod_input : FP_ZERO,
            feedback : FP_ZERO,
        }
    }

    pub fn get_sample(&mut self) -> FP {
        let phase       = self.phase_gen.update(self.mod_input + self.feedback);
        let wave_sample = self.wave_gen.generate(phase);
        let env_level   = self.env_gen.get_sample();

        let output = 
            if env_level == FP_ZERO || self.total_level == 0 {
                FP_ZERO
            } else {
                wave_sample * env_level * FP::from(self.total_level) >> 8
            };

        self.feedback = (output * FP::from(self.feedback_level)) >> 8;

        return output;
    }
}

impl Iterator for Operator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample().to_f32());
    }
}

impl Source for Operator {
    fn channels(&self) -> u16 {
        return 1;
    }

     fn sample_rate(&self) -> u32 {
        return super::SAMPLE_FREQ;
     }

     fn current_frame_len(&self) -> Option<usize> {
        return None;
     }

     fn total_duration(&self) -> Option<Duration> {
        return None;
     }
}