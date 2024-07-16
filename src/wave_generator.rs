//! wave_generator
//! 
//! generate several sine-based waveforms.

use crate::math::sin::sin_from_table;
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, EnumIter)]
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

const Q16_ONE : i32 = 0x0001_0000;
const Q16_MINUSONE : i32  = - Q16_ONE;

const Q16_QUART  : i32 = 0x3FFF;
const Q16_EIGHTH : i32 = Q16_QUART >> 1;

const Q_SHIFT : i32 = 14;
const Q_MASK  : i32 = 0x03;

pub fn generate(wave_gen : WaveForm, phase : i32) -> i32 {
	let q : u16 = ((phase >> Q_SHIFT) & Q_MASK) as u16;
	let w = phase & Q16_QUART;

	match wave_gen {
		WaveForm::FullSine => fullsine(q, w),
		WaveForm::HalfSine =>	halfsine(q, w),
		WaveForm::DblHalfSine => dblhalfsine(q, w),
		WaveForm::DblQuartSine => dblquartsine(q, w),
		WaveForm::FastSine => fastsine(q, w),
		WaveForm::FastHalfSine => fasthalfsine(q, w),
		WaveForm::Sawish => sawish(q, w),
		WaveForm::Square => square(q, w)
	}
}

// -------

fn fullsine(q : u16, w : i32) -> i32 {
	match q {
		0 => sin_from_table(w as i32),
		1 => sin_from_table(Q16_QUART - w),
		2 => - sin_from_table(w as i32),
		3 => - sin_from_table(Q16_QUART - w),
		_ => 0
	}
}

fn halfsine(q : u16, w : i32) -> i32 {
	match q {
		0 => sin_from_table(w),
		1 => sin_from_table(Q16_QUART - w),
		_ => 0
	}		
}

fn dblhalfsine(q : u16, w : i32) -> i32 {
	match q {
		0 | 2=> sin_from_table(w),
		1 | 3 => sin_from_table(Q16_QUART - w),
		_ => 0
	}		
}

fn dblquartsine(q : u16, w : i32) -> i32 {
	match q {
		0 => sin_from_table(w),
		2 => -sin_from_table(w),
	_ => 0
	}		
}

fn fastsine(q : u16, w : i32) -> i32 {
	match q {
		0 => {
			if (w & (Q16_EIGHTH + 1)) == 0 {
				sin_from_table(w * 2)
			} else {
				sin_from_table(Q16_QUART - (w & Q16_EIGHTH) * 2)
			}
		},
		1 => {
			if (w & (Q16_EIGHTH + 1)) == 0 {
				-sin_from_table(w * 2)
			} else {
				-sin_from_table(Q16_QUART - (w & Q16_EIGHTH) * 2)
			}
			},
	_ => 0
	}		
}

fn fasthalfsine(q : u16, w : i32) -> i32 {
	match q {
		0 | 1 => {
			if (w & (Q16_EIGHTH + 1)) == 0 {
				sin_from_table(w * 2)
			} else {
				sin_from_table(Q16_QUART - (w & Q16_EIGHTH) * 2)
			}
		},
		_ => 0
}		
}

fn sawish(q : u16, w : i32) -> i32 {
	match q {
		0 =>  sin_from_table((w / 2) & Q16_QUART),
		1 =>  sin_from_table((Q16_EIGHTH + w / 2) & Q16_QUART),
		2 => -sin_from_table(Q16_QUART - ((w / 2) & Q16_QUART)),
		3 => -sin_from_table(Q16_QUART - ((Q16_EIGHTH + w / 2) & Q16_QUART)),
		_ => 0
	}		
}

fn square(q : u16, _w : i32) -> i32 {
	match q {
		0 | 1 => Q16_ONE,
		2 | 3 => Q16_MINUSONE,
		_ => 0
	}		
}
