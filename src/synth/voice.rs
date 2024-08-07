use std::time::Duration;
use std::iter::zip;
use rodio::source::Source;

use crate::fp::*;

use super::operator::*;

pub struct Voice {
    pub operators : [ Operator; 4 ],
    pub algorithm : usize,
    output : FP,
    adder : FP,
}

impl Voice {
    pub fn new() -> Voice {
        Voice {
            operators : [
                Operator::new(),
                Operator::new(),
                Operator::new(),
                Operator::new()
            ],
            algorithm : 0,
            output : FP_ZERO,
            adder : FP_ZERO,
        }
    }

    pub fn op(&mut self, idx : usize) -> &mut Operator {
        &mut self.operators[idx]
    }

    pub fn get_sample(&mut self) -> f32 {
        self.output = FP_ZERO;
        self.adder = FP_ZERO;
        let algo = &ALGORITHMS[self.algorithm];
        for (op, i) in zip(&mut self.operators, 0..4) {
            op.mod_input = 
                match algo[i].mod_source {
                    Register::Null => FP_ZERO,
                    Register::Output => self.output,
                    Register::Adder => self.adder
                };
            match algo[i].out_sink {
                Register::Output => self.output = op.get_sample(),
                Register::Adder  => self.adder = self.adder + op.get_sample(),
                Register::Null => ()
            };
        }

        // Output sink of last op is final output
        return 
            match algo[3].out_sink {
                Register::Null => 0.0,
                Register::Output => self.output.to_f32(),
                Register::Adder  => self.adder.to_f32(),
            };
    }

    pub fn set_freq(&mut self, flog2 : FP) {
        for op in &mut self.operators {
            op.phase_gen.flog2 = flog2;
        }
    }

    pub fn note_on(&mut self, flog2 : FP) {
        for op in &mut self.operators {
            op.phase_gen.flog2 = flog2;
            op.env_gen.open();
        }
    }

    pub fn note_off(&mut self) {
        for op in &mut self.operators {
            op.env_gen.close();
        }
    }
}

impl Iterator for Voice {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for Voice {
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

enum Register {
    Null,
    Output,
    Adder
}

struct Route {
    mod_source : Register,
    out_sink : Register
}

type Algorithm = [Route ; 4];

const ALGORITHMS : [Algorithm; 3] = 
[
    // [1]-[2]-[3]-[4]->
    [ 
        Route { mod_source : Register::Null,   out_sink : Register::Output }, 
        Route { mod_source : Register::Output, out_sink : Register::Output }, 
        Route { mod_source : Register::Output, out_sink : Register::Output }, 
        Route { mod_source : Register::Output, out_sink : Register::Output }
    ], 
    //         .-[3]-.
    //         |     |
    // [1]-[2]-+-[4]-+->
    [
        Route { mod_source : Register::Null,   out_sink : Register::Output }, 
        Route { mod_source : Register::Output, out_sink : Register::Output }, 
        Route { mod_source : Register::Output, out_sink : Register::Adder  }, 
        Route { mod_source : Register::Output, out_sink : Register::Adder  }
    ],
    // [1]-.
    //     |
    // [2]-+
    //     |
    // [3]-+
    //     |
    // [4]-+->
    [
        Route { mod_source : Register::Null, out_sink : Register::Adder }, 
        Route { mod_source : Register::Null, out_sink : Register::Adder }, 
        Route { mod_source : Register::Null, out_sink : Register::Adder }, 
        Route { mod_source : Register::Null, out_sink : Register::Adder }
    ],
];