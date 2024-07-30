use crate::{
    circle_degrees::CircleDegrees,
    color::Color,
    models::{
        color_model::ColorModel,
        shared::{
            rgb_float_to_cmyk_float,
            rgb_float_to_hsvl,
        },
    },
    percentage_f32::PercentageF32,
};

pub struct Rgb {
    red:   u8,
    green: u8,
    blue:  u8,
}

impl ColorModel for Rgb {
    fn select_cmyk<const C: bool, const M: bool, const Y: bool, const K: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
        let (c, m, y, k) = self.select_cmyk_float::<C, M, Y, K>();

        (
            c.map(PercentageF32::to_percent_of_u8),
            m.map(PercentageF32::to_percent_of_u8),
            y.map(PercentageF32::to_percent_of_u8),
            k.map(PercentageF32::to_percent_of_u8),
        )
    }

    fn select_cmyk_float<const C: bool, const M: bool, const Y: bool, const K: bool>(
        &self,
    ) -> (
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let (r, g, b) = self.select_rgb_float::<true, true, true>();

        rgb_float_to_cmyk_float::<C, M, Y, K>(
            r.expect("Red was not returned"),
            g.expect("Green was not returned"),
            b.expect("Blue was not returned"),
        )
    }

    #[inline]
    fn select_rgb<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>) {
        (
            R.then_some(self.red),
            G.then_some(self.green),
            B.then_some(self.blue),
        )
    }

    #[inline]
    fn select_rgb_float<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        (
            R.then(|| PercentageF32::from_percent_of_u8(self.red)),
            G.then(|| PercentageF32::from_percent_of_u8(self.green)),
            B.then(|| PercentageF32::from_percent_of_u8(self.blue)),
        )
    }

    #[inline]
    fn select_hsv<const H: bool, const S: bool, const V: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let (r, g, b) = self.select_rgb_float::<true, true, true>();

        rgb_float_to_hsvl::<H, S, V, false>(
            r.expect("Red was not calculated"),
            g.expect("Green was not calculated"),
            b.expect("Blue was not calculated"),
        )
    }

    #[inline]
    fn select_hsl<const H: bool, const S: bool, const L: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let (r, g, b) = self.select_rgb_float::<true, true, true>();

        rgb_float_to_hsvl::<H, S, L, true>(
            r.expect("Red was not calculated"),
            g.expect("Green was not calculated"),
            b.expect("Blue was not calculated"),
        )
    }
}

impl<C: ColorModel> Color<C> {
    #[inline]
    pub fn to_rgb(&self) -> Color<Rgb> {
        let (r, g, b) = self.color.select_rgb::<true, true, true>();

        Color {
            color: Rgb {
                red:   r.expect("Red value was not returned"),
                green: g.expect("Green value was not returned"),
                blue:  b.expect("Blue values was not returned"),
            },
        }
    }

    #[inline]
    pub fn into_rgb(self) -> Color<Rgb> {
        self.to_rgb()
    }
}

impl Color<Rgb> {
    #[inline]
    pub const fn from_hex(from: u32) -> Self {
        Self {
            color: Rgb {
                red:   (from >> const { 2 * u8::BITS }) as u8,
                green: (from >> u8::BITS) as u8,
                blue:  from as u8,
            },
        }
    }

    #[inline]
    pub const fn new_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            color: Rgb { red, green, blue },
        }
    }
}

#[cfg(test)]
mod test {
    // rgb_tests!(
    //     Rgb {
    //         red:   161,
    //         green: 131,
    //         blue:  114,
    //     },
    //     Rgb {
    //         red:   181,
    //         green: 134,
    //         blue:  177,
    //     },
    //     Rgb {
    //         red:   138,
    //         green: 107,
    //         blue:  17,
    //     },
    //     Rgb {
    //         red:   1,
    //         green: 7,
    //         blue:  20,
    //     },
    //     Rgb {
    //         red:   222,
    //         green: 220,
    //         blue:  120,
    //     },
    //     Rgb {
    //         red:   122,
    //         green: 38,
    //         blue:  122,
    //     },
    //     Rgb {
    //         red:   177,
    //         green: 128,
    //         blue:  92,
    //     },
    //     Rgb {
    //         red:   92,
    //         green: 17,
    //         blue:  73,
    //     }
    // );
}
