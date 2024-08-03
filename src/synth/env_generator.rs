use std::f32::MIN;

use crate::fp::*;

const CLOCK_DIVIDER : u8 = 1; // update env every 4 samples (12kHz)

const INDEX_OFFSET : FP = FP { repr : 12625 }; // log2(8/7) + delta
const OFFSET_UP : FP    = FP { repr : 74898 }; // 8/7
const OFFSET_DN : FP    = FP { repr : 9362 };  // 1/7
const MIN_INDEX : FP    = FP { repr : 65536 * -3 };

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnvState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release
}

#[derive(Debug, Copy, Clone)]
pub struct EnvGenerator {
    pub attack_rate : FP,
    pub decay_rate : FP,
    pub sustain_level : FP,
    pub release_rate : FP,
    pub is_sustained : bool,

    clock : u8,
    index : FP,
    level : FP,
    state : EnvState
}

impl EnvGenerator {
    pub fn new() -> EnvGenerator {
        EnvGenerator {
            attack_rate : FP_ZERO,
            decay_rate : FP_ZERO,
            sustain_level : FP_ONE,
            release_rate : FP_ZERO,
            is_sustained : true,

            clock : CLOCK_DIVIDER,
            index : INDEX_OFFSET,
            level : FP_ZERO,
            state : EnvState::Idle
        }
    }

    pub fn open(&mut self) {
        self.state = EnvState::Attack;
        self.index = INDEX_OFFSET;
    }

    pub fn close(&mut self) {
        if self.state == EnvState:: Attack {
            self.find_release_index();
        }
        self.state = EnvState::Release;
    }

    pub fn get_sample(&mut self) -> FP {
        self.clock -= 1;
        if self.clock == 0 {
            self.clock = CLOCK_DIVIDER;
            match self.state {
                EnvState::Attack  => { self.attack(); }
                EnvState::Decay   => { self.decay(); }
                EnvState::Release => { self.release(); }
                _ => ()
            };
        }
        self.level
    }

    fn attack(&mut self) {
        // index counts down from XFACTOR to XFACTOR-3 in FP
        self.index = self.index - self.attack_rate;
        self.level = OFFSET_UP - FP::exp(self.index);

        if self.level >= FP_ONE || self.index <= MIN_INDEX {
            self.level = FP_ONE;
            self.state = EnvState::Decay;
            self.index = INDEX_OFFSET;
        }
    }

    fn decay(&mut self) {
        // index counts down from INDEX_OFFSET to INDEX_OFFSET-3 in FP
        self.index = self.index - self.decay_rate;
        self.level = FP::exp(self.index) - OFFSET_DN;

        if self.level <= self.sustain_level || self.index <= MIN_INDEX {
            // do not change level and index
            if self.is_sustained {
                self.state = EnvState::Sustain;
            } else {
                self.state = EnvState::Release;
            }
        }
    }

    fn release(&mut self) {
        // index counts down from INDEX_OFFSET to INDEX_OFFSET-3 in FP
        self.index = self.index - self.release_rate;
        self.level = FP::exp(self.index) - OFFSET_DN;

        if self.level <= FP_ZERO || self.index <= MIN_INDEX {
            self.level = FP_ZERO;
            self.state = EnvState::Idle;
            self.index = INDEX_OFFSET;
        }
    }

    fn find_release_index(&mut self) {
        // find index for release matching current level
        let mut idx = (MIN_INDEX + INDEX_OFFSET) >> 1;
        let mut adjust = (idx - MIN_INDEX) >> 1;
        for _ in 0..8 {
            let guess = FP::exp(idx) - OFFSET_DN;
            if guess > self.level {
                idx -= adjust;
            } else if guess < self.level {
                idx += adjust;
            }
            adjust = adjust >> 1;
        }
        self.index = idx;
    }

    pub fn state_to_str(self) -> String {
        match self.state {
            EnvState::Attack  => { String::from("Att") },
            EnvState::Decay   => { String::from("Dec") },
            EnvState::Idle    => { String::from("Idl") },
            EnvState::Release => { String::from("Rel") },
            EnvState::Sustain => { String::from("Sus") },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let mut env : EnvGenerator = EnvGenerator::new();
        env.attack_rate = FP::from(0.015);
        env.decay_rate = FP::from(0.015);
        env.sustain_level = FP::from(0.33);
        env.release_rate = FP::from(0.005);

        for tick in 0..1024 {
            if tick == 2 { env.open() }
            if tick == 350 { env.close() }
            let i = env.index.to_f32().to_string().replace(".", ",");
            let s = env.get_sample().to_f32().to_string().replace(".", ",");
            let e = env.state_to_str();
            println!("{tick};{i};{s};{e}");
        }

        // make test fail, otherwise there is no output from println!()
        assert!(env.state != EnvState::Idle);
    }
}
 