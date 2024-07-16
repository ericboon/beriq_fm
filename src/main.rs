mod wave_generator;
mod phase_generator;
mod math;

use phase_generator::*;
use wave_generator::*;

fn main() {
    const FLOG2 : i32 = 0x6c807; // log2(110) * 64k
    const GLOG2 : i32 = 0x4c807;
    let mut carrier = PhaseGenerator::new();
    let mut modulator = PhaseGenerator::new();
    let mut feedback = 0;
    for a in 0..1024 {
        let ph_m = modulator.update(GLOG2, 0);
        let m_out = generate(WaveForm::FullSine, ph_m);
        let phase = carrier.update(FLOG2, feedback + m_out / 8);
        let out = generate(WaveForm::FullSine, phase);
        println!("{a};{ph_m};{m_out};{phase};{out}" );
        feedback = out / 6;
    }
}
