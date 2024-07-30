//! wave_generator
//! 
//! generate several sine-based waveforms.

use crate::fp::*;

pub enum WaveForm {
	FullSine,
	HalfSine,
	DblHalfSine,
	DblQuartSine,
	FastSine,
	FastHalfSine,
	Sawish,
	Square
}

pub struct WaveGenerator {
	pub waveform : WaveForm
}

const FP_MINUSONE : FP = FP { repr: -65536 }; // -1

const FP_QUART  : FP = FP { repr: 0x0_3FFF };
const FP_EIGHTH : FP = FP { repr: 0x0_1FFF };

const Q_SHIFT : i32 = 14;
const Q_MASK  : u16 = 0x03;

impl WaveGenerator {
	pub fn new() -> WaveGenerator {
		WaveGenerator {
			waveform: WaveForm::FullSine
		}
	}

	pub fn generate(&self, phase : FP) -> FP {
		let ph : u16 = phase.fraq();
		let q : u16 = (ph >> Q_SHIFT) & Q_MASK;
		let w : FP = FP::raw((ph & FP_QUART.fraq()) as i32);

		match self.waveform {
			WaveForm::FullSine => Self::fullsine(q, w),
			WaveForm::HalfSine =>	Self::halfsine(q, w),
			WaveForm::DblHalfSine => Self::dblhalfsine(q, w),
			WaveForm::DblQuartSine => Self::dblquartsine(q, w),
			WaveForm::FastSine => Self::fastsine(q, w),
			WaveForm::FastHalfSine => Self::fasthalfsine(q, w),
			WaveForm::Sawish => Self::sawish(q, w),
			WaveForm::Square => Self::square(q, w)
		}
	}

	// -------

	fn fullsine(q : u16, w : FP) -> FP {
		match q {
			0 => FP::sinw(w),
			1 => FP::sinw(FP_QUART - w),
			2 => - FP::sinw(w),
			3 => - FP::sinw(FP_QUART - w),
			_ => FP_ZERO
		}
	}

	fn halfsine(q : u16, w : FP) -> FP {
		match q {
			0 => FP::sinw(w),
			1 => FP::sinw(FP_QUART - w),
			_ => FP_ZERO
		}		
	}

	fn dblhalfsine(q : u16, w : FP) -> FP {
		match q {
			0 | 2=> FP::sinw(w),
			1 | 3 => FP::sinw(FP_QUART - w),
			_ => FP_ZERO
		}		
	}

	fn dblquartsine(q : u16, w : FP) -> FP {
		match q {
			0 => FP::sinw(w),
			2 => -FP::sinw(w),
			_ => FP_ZERO
		}		
	}

	fn fastsine(q : u16, w : FP) -> FP {
		match q {
			0 => {
				if w < FP_EIGHTH {
					FP::sinw(w * 2)
				} else {
					FP::sinw((FP_QUART - w) * 2)
				}
			},
			1 => {
				if w < FP_EIGHTH {
					-FP::sinw(w * 2)
				} else {
					-FP::sinw((FP_QUART - w) * 2)
				}
			},
			_ => FP_ZERO
		}		
	}

	fn fasthalfsine(q : u16, w : FP) -> FP {
		match q {
			0 | 1 => {
				if w < FP_EIGHTH {
					FP::sinw(w * 2)
				} else {
					FP::sinw((FP_QUART - w) * 2)
				}
			},
			_ => FP_ZERO
	}		
	}

	fn sawish(q : u16, w : FP) -> FP {
		let w2 = w >> 1;
		match q {
			0 =>  FP::sinw(w2),
			1 =>  FP::sinw(w2 + FP_EIGHTH),
			2 => -FP::sinw(FP_QUART - w2),
			3 => -FP::sinw(FP_EIGHTH - w2),
			_ => FP_ZERO
		}		
	}

	fn square(q : u16, _w : FP) -> FP {
		match q {
			0 | 1 => FP_ONE,
			2 | 3 => FP_MINUSONE,
			_ => FP_ZERO
		}		
	}
}