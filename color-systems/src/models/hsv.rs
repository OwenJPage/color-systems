use crate::{
    circle_degrees::CircleDegrees,
    color::Color,
    models::{
        color_model::ColorModel,
        shared::{
            get_rgb_using_float,
            rgb_float_to_cmyk_float,
        },
    },
    percentage_f32::PercentageF32,
};

pub struct Hsv {
    hue:        CircleDegrees,
    saturation: PercentageF32,
    value:      PercentageF32,
}

impl ColorModel for Hsv {
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
        get_rgb_using_float::<R, G, B, Self>(self)
    }

    fn select_rgb_float<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let h_raw = self.hue.value();
        let s_raw = self.saturation.value();
        let v_raw = self.value.value();

        let f = |n| {
            let k = (n + (h_raw as f32 / 60.)) % 6.;

            PercentageF32::new_or_panic(
                v_raw - v_raw * s_raw * f32::max(0., f32::min(k, f32::min(4. - k, 1.))),
            )
        };

        (R.then(|| f(5.)), G.then(|| f(3.)), B.then(|| f(1.)))
    }

    #[inline]
    fn select_hsv<const H: bool, const S: bool, const V: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        (
            H.then_some(self.hue),
            S.then_some(self.saturation),
            V.then_some(self.value),
        )
    }

    fn select_hsl<const H: bool, const S: bool, const L: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let s_raw = self.saturation.value();
        let v_raw = self.value.value();

        let l = const { L || S }.then(|| v_raw * (1. - (s_raw / 2.)));

        let s = S.then(|| {
            if v_raw == 0. || v_raw == 1. {
                0.
            } else {
                let l_val = l.expect("Luminosity was not calculated");

                (v_raw - l_val) / f32::min(l_val, 1. - l_val)
            }
        });

        (
            H.then_some(self.hue),
            s.map(PercentageF32::new_or_panic),
            l.map(PercentageF32::new_or_panic),
        )
    }
}

impl<C: ColorModel> Color<C> {
    pub fn as_hsv(&self) -> Color<Hsv> {
        let (h, s, v) = self.color.select_hsv::<true, true, true>();

        Color {
            color: Hsv {
                hue:        h.expect("Hue was not returned"),
                saturation: s.expect("Saturation was not returned"),
                value:      v.expect("Value was not returned"),
            },
        }
    }

    pub fn into_hsv(self) -> Color<Hsv> {
        self.as_hsv()
    }
}

impl Color<Hsv> {
    pub const fn new_hsv(
        hue: CircleDegrees,
        saturation: PercentageF32,
        value: PercentageF32,
    ) -> Self {
        Self {
            color: Hsv {
                hue,
                saturation,
                value,
            },
        }
    }
}
