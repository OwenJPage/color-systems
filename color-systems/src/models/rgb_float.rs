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

pub struct RgbFloat {
    red:   PercentageF32,
    green: PercentageF32,
    blue:  PercentageF32,
}

impl ColorModel for RgbFloat {
    #[inline]
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

    #[inline]
    fn select_cmyk_float<const C: bool, const M: bool, const Y: bool, const K: bool>(
        &self,
    ) -> (
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        rgb_float_to_cmyk_float::<C, M, Y, K>(self.red, self.green, self.blue)
    }

    fn select_rgb<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>) {
        (
            R.then(|| self.red.to_percent_of_u8()),
            G.then(|| self.green.to_percent_of_u8()),
            B.then(|| self.blue.to_percent_of_u8()),
        )
    }

    fn select_rgb_float<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        (
            R.then_some(self.red),
            G.then_some(self.green),
            B.then_some(self.blue),
        )
    }

    fn select_hsv<const H: bool, const S: bool, const V: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        rgb_float_to_hsvl::<H, S, V, false>(self.red, self.green, self.blue)
    }

    fn select_hsl<const H: bool, const S: bool, const L: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        rgb_float_to_hsvl::<H, S, L, true>(self.red, self.green, self.blue)
    }
}

impl<C: ColorModel> Color<C> {
    pub fn to_rgb_float(&self) -> Color<RgbFloat> {
        let (r, g, b) = self.color.select_rgb_float::<true, true, true>();

        Color {
            color: RgbFloat {
                red:   r.expect("Red value not returned"),
                green: g.expect("Green value not returned"),
                blue:  b.expect("Blue value not returned"),
            },
        }
    }

    pub fn into_rgb_float(self) -> Color<RgbFloat> {
        self.to_rgb_float()
    }
}

impl Color<RgbFloat> {
    pub const fn new_rgb_float(
        red: PercentageF32,
        green: PercentageF32,
        blue: PercentageF32,
    ) -> Self {
        Self {
            color: RgbFloat { red, green, blue },
        }
    }
}

#[cfg(test)]
mod test {}
