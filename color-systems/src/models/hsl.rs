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

pub struct Hsl {
    hue:        CircleDegrees,
    saturation: PercentageF32,
    luminosity: PercentageF32,
}

impl ColorModel for Hsl {
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
        let l_raw = self.luminosity.value();

        let a = s_raw * f32::min(l_raw, 1. - l_raw);

        let f = |n| {
            let k = (n + (h_raw as f32 / 30.)) % 12.;

            PercentageF32::new_or_panic(
                // l_raw - a * f32::max(-1., f32::min(k - 3., f32::min(9. - k, 1.))),
                f32::mul_add(
                    -a,
                    f32::max(-1., f32::min(k - 3., f32::min(9. - k, 1.))),
                    l_raw,
                ),
            )
        };

        (R.then(|| f(0.)), G.then(|| f(8.)), B.then(|| f(4.)))
    }

    fn select_hsv<const H: bool, const S: bool, const V: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let s_raw = self.saturation.value();
        let l_raw = self.luminosity.value();

        let v = const { V || S }.then(|| l_raw + s_raw * f32::min(l_raw, 1. - l_raw));
        let s = S.then(|| {
            let v = v.expect("Value was not calculated");

            if v == 0. {
                0.
            } else {
                2. * (1. - l_raw / v)
            }
        });

        (
            H.then_some(self.hue),
            s.map(PercentageF32::new_or_panic),
            v.map(PercentageF32::new_or_panic),
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
        (
            H.then_some(self.hue),
            S.then_some(self.saturation),
            L.then_some(self.luminosity),
        )
    }
}

impl<C: ColorModel> Color<C> {
    #[inline]
    pub fn to_hsl(&self) -> Color<Hsl> {
        let (h, s, l) = self.color.select_hsl::<true, true, true>();

        Color::new_hsl(
            h.expect("Hue was not returned"),
            s.expect("Saturation was not returned"),
            l.expect("Luminosity was not returned"),
        )
    }

    #[inline]
    pub fn into_hsl(self) -> Color<Hsl> {
        self.to_hsl()
    }
}

impl Color<Hsl> {
    #[inline]
    pub const fn new_hsl(
        hue: CircleDegrees,
        saturation: PercentageF32,
        luminosity: PercentageF32,
    ) -> Self {
        Self {
            color: Hsl {
                hue,
                saturation,
                luminosity,
            },
        }
    }
}
