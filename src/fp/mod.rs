/// fixpoint numbers 16.16

use std::ops;
use std::convert::From;

pub mod exp;
pub mod sin;

pub const FP_ZERO : FP = FP { repr : 0x0_0000 };
pub const FP_ONE : FP = FP { repr : 0x1_0000 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FP {
    pub repr : i32
}

impl FP {
    pub fn raw(item : i32) -> Self {
        FP { repr: item }
    }

    pub fn frac(self) -> FP {
        FP { repr : (self.repr & 0xFFFF) }
    }

    pub fn fraq(self) -> u16 {
        (self.repr & 0xFFFF) as u16
    }

    pub fn int(self) -> i32 {
        self.repr >> 16
    }

    pub fn to_f32(self) -> f32 {
        self.repr as f32 / 65536.0
    }
}

impl From<u8> for FP {
    fn from(item : u8) -> Self {
        FP { repr: item as i32 * 65536 }
    }
}
impl From<u16> for FP {
    fn from(item : u16) -> Self {
        FP { repr: item as i32 * 65536 }
    }
}

impl From<i32> for FP {
    fn from(item : i32) -> Self {
        FP { repr: item * 65536 }
    }
}

impl From<f32> for FP {
    fn from(item : f32) -> Self {
        FP { repr: (item * 65536.0) as i32 }
    }
}

impl ops::Neg for FP {
    type Output = Self;
    fn neg(self) -> Self {
        FP { repr : - self.repr }
    }
}

impl ops::Add<FP> for FP {
    type Output = Self;
    fn add(self, rhs : FP) -> Self {
        FP { repr : self.repr + rhs.repr }
    }
}

impl ops::Sub<FP> for FP {
    type Output = Self;
    fn sub(self, rhs : FP) -> Self {
        FP { repr : self.repr - rhs.repr }
    }
}

impl ops::Mul<i32> for FP {
    type Output = Self;
    fn mul(self, rhs : i32) -> Self {
        FP { repr : self.repr * rhs }
    }
}
impl ops::Mul<FP> for FP {
    type Output = Self;
    fn mul(self, rhs : FP) -> Self {
        let result64 : i64 = self.repr as i64 * rhs.repr as i64;
        FP { repr : (result64 >> 16) as i32 }
    }
}

impl ops::Shl<usize> for FP {
    type Output = Self;
    fn shl(self, rhs : usize) -> Self {
        FP { repr: self.repr << rhs }
    }
}

impl ops::Shr<usize> for FP {
    type Output = Self;
    fn shr(self, rhs : usize) -> Self {
        FP { repr: self.repr >> rhs }
    }
}

impl ops::BitAnd<i32> for FP {
    type Output = Self;
    fn bitand(self, rhs : i32) -> Self {
        FP { repr: self.repr & rhs }
    }
}

impl ops::AddAssign<FP> for FP {
    fn add_assign(&mut self, rhs: FP) {
        self.repr += rhs.repr;
    }
}

impl ops::SubAssign<FP> for FP {
    fn sub_assign(&mut self, rhs: FP) {
        self.repr -= rhs.repr;
    }
}

impl ops::MulAssign<FP> for FP {
    fn mul_assign(&mut self, rhs: FP) {
        let result64 : i64 = self.repr as i64 * rhs.repr as i64;
        self.repr = (result64 >> 16) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fp() {
        let fp1 = FP::from(1);
        let fp2 = FP::from(2);
        let fp_pi = FP::from(3.1415279);
        assert_eq!(fp1.repr, 0x1_0000);
        assert_eq!(fp2.repr, 0x2_0000);
        assert_eq!(fp_pi.repr, 0x3_243B);
        assert!(fp1 < fp_pi);
        assert!(fp1 + fp_pi == FP::from(4.1415279));
        assert!(fp_pi > fp2);
        assert_eq!(fp1 * fp2, fp2);
        assert_eq!(fp2 * fp_pi, FP::from(6.2830558));
        assert_eq!(fp1 + fp2 * fp_pi, FP::from(7.2830558));
    }
}
