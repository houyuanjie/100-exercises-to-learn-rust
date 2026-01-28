use std::ops::Add;

// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value.eq(other)
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl<T: Into<SaturatingU16>> Add<T> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.into().value),
        }
    }
}

impl Add<&Self> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        self.add(*rhs)
    }
}

impl Into<SaturatingU16> for &u16 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16 { value: *self }
    }
}

impl Into<SaturatingU16> for u8 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16 { value: self.into() }
    }
}

impl Into<SaturatingU16> for &u8 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16 {
            value: (*self).into(),
        }
    }
}
