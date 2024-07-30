use crate::{
    circle_degrees::CircleDegrees,
    percentage_f32::PercentageF32,
};

pub trait ColorModel {
    fn select_cmyk<const C: bool, const M: bool, const Y: bool, const K: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>);

    fn select_cmyk_float<const C: bool, const M: bool, const Y: bool, const K: bool>(
        &self,
    ) -> (
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    );

    fn select_rgb<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>);

    fn select_rgb_float<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (
        Option<PercentageF32>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    );

    fn select_hsv<const H: bool, const S: bool, const V: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    );

    fn select_hsl<const H: bool, const S: bool, const L: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    );
}

#[cfg(test)]
#[macro_use]
pub mod test_utils {
    use crate::{
        circle_degrees::CircleDegrees,
        models::color_model::ColorModel,
        percentage_f32::PercentageF32,
    };

    pub trait GenerateColour<C: ColorModel> {
        fn generate(&self) -> C;
    }

    pub struct TestColour {
        pub cmyk:       (u8, u8, u8, u8),
        pub cmyk_float: (PercentageF32, PercentageF32, PercentageF32, PercentageF32),
        pub hsl:        (CircleDegrees, PercentageF32, PercentageF32),
        pub hsv:        (CircleDegrees, PercentageF32, PercentageF32),
        pub rgb:        (u8, u8, u8),
        pub rgb_float:  (PercentageF32, PercentageF32, PercentageF32),
    }

    macro_rules! colour_model_tests {
        ($test_colour:expr) => {};
    }

    macro_rules! assert_rgb_color {
        ($selector:expr, $value:expr, $expected:expr) => {
            if $selector {
                assert_eq!(
                    $value.unwrap_or_else(|| {
                        panic!("Returned {} value was None", stringify!($selector));
                    }),
                    $expected,
                    "Expected {} value of {}",
                    stringify!($selector),
                    $expected
                )
            } else {
                assert_eq!(
                    $value,
                    None,
                    "Expected {} value of None",
                    stringify!($selector)
                )
            }
        };
    }

    macro_rules! rgb_tests {
        (
            $none_color:expr,
            $r_only_color:expr,
            $g_only_color:expr,
            $b_only_color:expr,
            $rg_only_color:expr,
            $rb_only_color:expr,
            $gb_only_color:expr,
            $all_color:expr
        ) => {
            use $crate::color::color_model::test_utils::assert_rgb_color;

            #[inline(always)]
            fn rgb_asserts<const R: bool, const G: bool, const B: bool>(color: Rgb) {
                let (r, g, b) = color.select_rgb::<R, G, B>();

                assert_rgb_color!(R, r, color.red);
                assert_rgb_color!(G, g, color.green);
                assert_rgb_color!(B, b, color.blue);
            }

            #[test]
            fn test_select_rgb_none() {
                let color = $none_color;

                rgb_asserts::<false, false, false>(color);
            }

            #[test]
            fn test_select_rgb_r_only() {
                let color = $r_only_color;

                rgb_asserts::<true, false, false>(color);
            }

            #[test]
            fn test_select_rgb_g_only() {
                let color = $g_only_color;

                rgb_asserts::<false, true, false>(color);
            }
            #[test]
            fn test_select_rgb_b_only() {
                let color = $b_only_color;

                rgb_asserts::<false, false, true>(color);
            }

            #[test]
            fn test_select_rgb_rg_only() {
                let color = $rg_only_color;

                rgb_asserts::<true, true, false>(color);
            }

            #[test]
            fn test_select_rgb_rb_only() {
                let color = $rb_only_color;

                rgb_asserts::<true, false, true>(color);
            }

            #[test]
            fn test_select_rgb_gb_only() {
                let color = $gb_only_color;

                rgb_asserts::<false, true, true>(color);
            }

            #[test]
            fn test_select_rgb_all() {
                let color = $all_color;

                rgb_asserts::<true, true, true>(color);
            }
        };
    }

    pub(crate) use {
        // assert_rgb_color,
        // rgb_tests,
        colour_model_tests,
    };
}
