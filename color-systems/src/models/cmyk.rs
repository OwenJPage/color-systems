use crate::{
    circle_degrees::CircleDegrees,
    color::Color,
    models::{
        color_model::ColorModel,
        shared::{
            cmyk_float_to_rgb_float,
            rgb_float_to_hsvl,
        },
    },
    percentage_f32::PercentageF32,
};

pub struct Cmyk {
    cyan:      u8,
    magenta:   u8,
    yellow:    u8,
    key_black: u8,
}

impl ColorModel for Cmyk {
    #[inline]
    fn select_cmyk<const C: bool, const M: bool, const Y: bool, const K: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
        (
            C.then_some(self.cyan),
            M.then_some(self.magenta),
            Y.then_some(self.yellow),
            K.then_some(self.key_black),
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
            C.then(|| PercentageF32::from_percent_of_u8(self.cyan)),
            M.then(|| PercentageF32::from_percent_of_u8(self.magenta)),
            Y.then(|| PercentageF32::from_percent_of_u8(self.yellow)),
            K.then(|| PercentageF32::from_percent_of_u8(self.key_black)),
        )
    }

    #[inline]
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
        let (c, m, y, k) = self.select_cmyk_float::<R, G, B, true>();

        cmyk_float_to_rgb_float::<R, G, B>(
            c.expect("Cyan was not returned"),
            m.expect("Magenta was not returned"),
            y.expect("Yellow was not returned"),
            k.expect("Key/black was not returned"),
        )
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
            r.expect("Red value was not returned"),
            g.expect("Green value was not returned"),
            b.expect("Blue value was not returned"),
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
            r.expect("Red value was not returned"),
            g.expect("Green value was not returned"),
            b.expect("Blue value was not returned"),
        )
    }
}

impl<C: ColorModel> Color<C> {
    #[inline]
    pub fn to_cmyk(&self) -> Color<Cmyk> {
        let (c, m, y, k) = self.color.select_cmyk::<true, true, true, true>();

        Color::new_cmyk(
            c.expect("Cyan not returned"),
            m.expect("Magenta not returned"),
            y.expect("Yellow not returned"),
            k.expect("Key/black not returned"),
        )
    }

    #[inline]
    pub fn into_cmyk(self) -> Color<Cmyk> {
        self.to_cmyk()
    }
}

impl Color<Cmyk> {
    #[inline]
    pub const fn new_cmyk(cyan: u8, magenta: u8, yellow: u8, key_black: u8) -> Self {
        Self {
            color: Cmyk {
                cyan,
                magenta,
                yellow,
                key_black,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::super::color_model::test_utils::TestColour,
        crate::{
            circle_degrees,
            models::{
                cmyk::Cmyk,
                color_model::test_utils::{
                    colour_model_tests,
                    GenerateColour,
                },
            },
            p32,
        },
    };

    impl GenerateColour<Cmyk> for TestColour {
        fn generate(&self) -> Cmyk {
            Cmyk {
                cyan:      self.cmyk.0,
                magenta:   self.cmyk.1,
                yellow:    self.cmyk.2,
                key_black: self.cmyk.3,
            }
        }
    }

    colour_model_tests!(TestColour {
        cmyk:       (0, 0, 0, 0),
        cmyk_float: (p32!(0.), p32!(0.), p32!(0.), p32!(0.)),
        hsl:        (circle_degrees!(0), p32!(0.), p32!(0.)),
        hsv:        (circle_degrees!(0), p32!(0.), p32!(0.)),
        rgb:        (0, 0, 0),
        rgb_float:  (p32!(0.), p32!(0.), p32!(0.)),
    });

    #[test]
    pub fn test() {}
}
