use {
    super::color_model::ColorModel,
    crate::{
        circle_degrees::CircleDegrees,
        color::Color,
        models::shared::{
            cmyk_float_to_rgb_float,
            rgb_float_to_hsvl,
        },
        percentage_f32::PercentageF32,
    },
};

pub struct CmykFloat {
    cyan:      PercentageF32,
    magenta:   PercentageF32,
    yellow:    PercentageF32,
    key_black: PercentageF32,
}

impl ColorModel for CmykFloat {
    #[inline]
    fn select_cmyk<const C: bool, const M: bool, const Y: bool, const K: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
        (
            C.then(|| self.cyan.as_percent_of_u8()),
            M.then(|| self.magenta.as_percent_of_u8()),
            Y.then(|| self.yellow.as_percent_of_u8()),
            K.then(|| self.key_black.as_percent_of_u8()),
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
        (
            C.then_some(self.cyan),
            M.then_some(self.magenta),
            Y.then_some(self.yellow),
            K.then_some(self.key_black),
        )
    }

    fn select_rgb<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>) {
        let (r, g, b) = self.select_rgb_float::<R, G, B>();

        (
            r.map(PercentageF32::to_percent_of_u8),
            g.map(PercentageF32::to_percent_of_u8),
            b.map(PercentageF32::to_percent_of_u8),
        )
    }

    fn select_rgb_float<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        cmyk_float_to_rgb_float::<R, G, B>(self.cyan, self.magenta, self.yellow, self.key_black)
    }

    fn select_hsv<const H: bool, const S: bool, const V: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let (r, g, b) = self.select_rgb_float::<true, true, true>();

        rgb_float_to_hsvl::<H, S, V, false>(
            r.expect("Red was not returned"),
            g.expect("Green was not returned"),
            b.expect("Blue was not returned"),
        )
    }

    fn select_hsl<const H: bool, const S: bool, const L: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let (r, g, b) = self.select_rgb_float::<true, true, true>();

        rgb_float_to_hsvl::<H, S, L, true>(
            r.expect("Red was not returned"),
            g.expect("Green was not returned"),
            b.expect("Blue was not returned"),
        )
    }
}

impl<C: ColorModel> Color<C> {
    #[inline]
    pub fn to_cmyk_float(&self) -> Color<CmykFloat> {
        let (c, m, y, k) = self.color.select_cmyk_float::<true, true, true, true>();

        Color::new_cmyk_float(
            c.expect("Cyan not returned"),
            m.expect("Magenta not returned"),
            y.expect("Yellow not returned"),
            k.expect("Key/black not returned"),
        )
    }

    #[inline]
    pub fn into_cmyk_float(self) -> Color<CmykFloat> {
        self.to_cmyk_float()
    }
}

impl Color<CmykFloat> {
    #[inline]
    pub const fn new_cmyk_float(
        cyan: PercentageF32,
        magenta: PercentageF32,
        yellow: PercentageF32,
        key_black: PercentageF32,
    ) -> Self {
        Self {
            color: CmykFloat {
                cyan,
                magenta,
                yellow,
                key_black,
            },
        }
    }
}
