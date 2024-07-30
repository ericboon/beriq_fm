/// FM synth
use super::fp::*;

pub const SAMPLE_FREQ : u32 = 48000; // -> log2(SAMPLE_FREQ) = 15.55075
const LOG2_SF : FP = FP { repr : 0xF_8CFE }; // 15.55075 as FP

pub mod phase_generator;
pub mod wave_generator;
pub mod operator;
pub mod voice; 