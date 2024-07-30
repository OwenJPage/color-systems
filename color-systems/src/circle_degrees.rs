use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};

#[macro_export]
macro_rules! circle_degrees_wrapped {
    ($value:expr) => {
        $crate::circle_degrees::CircleDegrees::new_wrapped($value)
    };
}

#[macro_export]
macro_rules! circle_degrees {
    ($value:expr) => {{
        const OUTPUT: $crate::circle_degrees::CircleDegrees =
            $crate::circle_degrees::CircleDegrees::new_exact_or_panic($value);
        OUTPUT
    }};
}

#[derive(Clone, Copy, Debug)]
pub struct CircleDegrees(i16);

impl CircleDegrees {
    #[inline]
    pub const fn new_exact(with: i16) -> Option<Self> {
        match with {
            0..=359 => Some(Self(with)),
            _ => None,
        }
    }

    pub const fn new_exact_or_panic(with: i16) -> Self {
        assert!(0 <= with && with < 360, "Value is not in range 0..360");
        Self(with)
    }

    #[inline]
    pub const fn new_wrapped(with: i16) -> Self {
        Self(with % 360)
    }

    #[inline]
    pub const fn value(&self) -> i16 {
        self.0
    }
}

impl Add for CircleDegrees {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new_wrapped(self.0 + rhs.0)
    }
}

impl AddAssign for CircleDegrees {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for CircleDegrees {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_wrapped(self.0 - rhs.0)
    }
}

impl SubAssign for CircleDegrees {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<CircleDegrees> for i16 {
    #[inline]
    fn from(value: CircleDegrees) -> Self {
        value.value()
    }
}
