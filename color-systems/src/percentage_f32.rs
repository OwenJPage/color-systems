use std::ops::{
    Add,
    Div,
    Mul,
    Sub,
};

#[macro_export]
macro_rules! p32 {
    ($value:expr) => {
        $crate::percentage_f32::PercentageF32::new_or_panic($value)
    };
}

#[macro_export]
macro_rules! try_percentage_f32 {
    ($value:expr) => {
        $crate::percentage_f32::PercentageF32::try_new($value)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct PercentageF32(f32);

impl PercentageF32 {
    pub const MAX: Self = Self(1.);
    pub const MIN: Self = Self(0.);

    #[inline]
    pub fn try_new(with: f32) -> Option<Self> {
        if (0f32..=1f32).contains(&with) {
            Some(Self(with))
        } else {
            None
        }
    }

    #[inline]
    pub fn new_or_panic(with: f32) -> Self {
        Self::try_new(with).unwrap_or_else(|| {
            panic!("Attempted to create new PercentageF32 using invalid value ({with})")
        })
    }

    #[inline]
    pub fn from_percent_of_u8(from: u8) -> Self {
        Self(from as f32 / u8::MAX as f32)
    }

    #[inline]
    pub const fn value(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn as_percent_of_u8(&self) -> u8 {
        f32::round(self.0 * u8::MAX as f32) as u8
    }

    #[inline]
    pub fn to_percent_of_u8(self) -> u8 {
        f32::round(self.0 * u8::MAX as f32) as u8
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self(f32::max(self.0, other.0))
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self(f32::min(self.0, other.0))
    }
}

impl Add for PercentageF32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let value = self.0 + rhs.0;

        Self::try_new(value).unwrap_or_else(|| {
            panic!("Add operation resulted in a value outside of valid range ({value})")
        })
    }
}

impl Div for PercentageF32 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let value = self.0 / rhs.0;

        Self::try_new(value).unwrap_or_else(|| {
            panic!("Div operation resulted in a value outside of valid range ({value})")
        })
    }
}

impl Mul for PercentageF32 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let value = self.0 * rhs.0;

        Self::try_new(value).unwrap_or_else(|| {
            panic!("Mul operation resulted in a value outside of valid range ({value})")
        })
    }
}

impl Sub for PercentageF32 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let value = self.0 - rhs.0;

        Self::try_new(value).unwrap_or_else(|| {
            panic!("Sub operation resulted in a value outside of valid range ({value})")
        })
    }
}

impl TryFrom<f32> for PercentageF32 {
    type Error = f32;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_new(value).ok_or(value)
    }
}

impl From<PercentageF32> for f32 {
    #[inline]
    fn from(value: PercentageF32) -> Self {
        value.value()
    }
}
