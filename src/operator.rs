//! operator
//!
//! models an FM operator

struct Operator {
    phase_gen : PhaseGenerator;
    wave_gen : WaveGenerator;

    wave : Waveform;
    phase : Phase;
    
}