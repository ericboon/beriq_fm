pub const SAMPLE_FREQ : u32 = 48000; // -> log2(SAMPLE_FREQ) = 15.55075
const LOG2_SF : i32 = 0xF8CFE; // 15.55075 as 16.16fp

use crate::exp::*;

pub struct PhaseGenerator {
    phase: i32,
    flog2: i32,
}

impl PhaseGenerator {
    pub fn new() -> PhaseGenerator {
        PhaseGenerator {
            phase: 0,
            flog2: 0,
        }
    }

    pub fn set_freq(&mut self, freq_log2: i32) {
        self.flog2 = freq_log2;
    }

    pub fn update(&mut self, m: i32) -> i32 {
        // freq_log2 is the log2 of the freq in 16.16 fixed point,
        // 16 is octave, .16 is note within octave 
        // (Basically this is the 1V/Oct input)
        //
        // d(wt) = freq / sample_freq
        // = exp2[ log2(freq) - log2(sample_freq) ]

        let phase_inc = exp_q16(self.flog2 - LOG2_SF);
        self.phase = (self.phase + phase_inc) & 0x0_FFFF;

        // mod is the modulation signal
        // (from) the modulating operator

        return (self.phase + m) & 0x0_FFFF;
    }
}