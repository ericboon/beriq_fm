use crate::fp::*;
use crate::synth::LOG2_SF;

#[derive(Debug, Copy, Clone)]
pub struct PhaseGenerator {
    pub phase: FP,
    pub flog2: FP,
    pub tune: FP, // this is the log2 of "mult"
}

impl PhaseGenerator {
    pub fn new() -> PhaseGenerator {
        PhaseGenerator {
            phase: FP_ZERO,
            flog2: FP_ZERO,
            tune: FP_ZERO,
        }
    }

    pub fn update(&mut self, m: FP) -> FP {
        // flog2 is the log2 of the freq in FP,
        // int is octave, frac is note within octave 
        // (Basically this is the 1V/Oct input)
        //
        // d(wt) = freq / sample_freq
        // = exp2[ log2(freq) - log2(sample_freq) ]

        let phase_inc = FP::exp(self.flog2 + self.tune - LOG2_SF);
        self.phase = (self.phase + phase_inc).frac();

        // m is the modulation signal from the modulating operator
        (self.phase + m).frac()
    }
}