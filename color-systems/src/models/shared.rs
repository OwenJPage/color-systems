use crate::{
    circle_degrees::CircleDegrees,
    circle_degrees_wrapped,
    models::color_model::ColorModel,
    percentage_f32::PercentageF32,
};

#[inline]
pub fn get_rgb_using_float<const R: bool, const G: bool, const B: bool, C: ColorModel>(
    colour: &C,
) -> (Option<u8>, Option<u8>, Option<u8>) {
    let (r_float, g_float, b_float) = colour.select_rgb_float::<R, G, B>();

    (
        R.then(|| {
            r_float
                .map(PercentageF32::to_percent_of_u8)
                .expect("Red value was not calculated")
        }),
        G.then(|| {
            g_float
                .map(PercentageF32::to_percent_of_u8)
                .expect("Green value was not calculated")
        }),
        B.then(|| {
            b_float
                .map(PercentageF32::to_percent_of_u8)
                .expect("Blue value was not calculated")
        }),
    )
}

pub fn rgb_float_to_hsvl<const H: bool, const S: bool, const VL: bool, const VL_IS_L: bool>(
    red: PercentageF32,
    green: PercentageF32,
    blue: PercentageF32,
) -> (
    Option<CircleDegrees>,
    Option<PercentageF32>,
    Option<PercentageF32>,
) {
    let r_float = red.value();
    let g_float = green.value();
    let b_float = blue.value();

    let max = f32::max(r_float, f32::max(g_float, b_float));
    let min = f32::min(r_float, f32::min(g_float, b_float));

    let h = H.then(|| {
        if max == min {
            circle_degrees_wrapped!(0)
        } else {
            let range = max - min;

            let component = match max {
                m if m == r_float => ((g_float - b_float) / range) % 6.,
                m if m == g_float => (b_float - r_float) / range + 2.,
                m if m == b_float => (r_float - g_float) / range + 4.,
                _ => panic!("Max does not match any RGB component"),
            };

            circle_degrees_wrapped!(f32::round(component * 60.) as i16)
        }
    });

    let vl = VL.then(|| {
        if VL_IS_L {
            (max + min) / 2.
        } else {
            max
        }
    });

    let s = S.then(|| {
        let diff = max - min;

        if VL_IS_L {
            let l = vl.expect("Luminosity was not calculated");

            if l == 0. || l == 1. {
                0.
            } else {
                diff / (1. - f32::abs(2. * max - diff - 1.))
            }
        } else if max == 0. {
            0.
        } else {
            diff / max
        }
    });

    (
        h,
        s.map(PercentageF32::new_or_panic),
        vl.map(PercentageF32::new_or_panic),
    )
}

pub fn rgb_float_to_cmyk_float<const C: bool, const M: bool, const Y: bool, const K: bool>(
    red: PercentageF32,
    green: PercentageF32,
    blue: PercentageF32,
) -> (
    Option<PercentageF32>,
    Option<PercentageF32>,
    Option<PercentageF32>,
    Option<PercentageF32>,
) {
    if const { !(C || M || Y || K) } {
        return (None, None, None, None);
    }

    let k_inv = PercentageF32::max(red, PercentageF32::max(green, blue));
    let k = PercentageF32::MAX - k_inv;

    (
        C.then(|| (PercentageF32::MAX - red - k) / k_inv),
        M.then(|| (PercentageF32::MAX - green - k) / k_inv),
        Y.then(|| (PercentageF32::MAX - blue - k) / k_inv),
        K.then_some(k),
    )
}

pub fn cmyk_float_to_rgb_float<const R: bool, const G: bool, const B: bool>(
    cyan: PercentageF32,
    magenta: PercentageF32,
    yellow: PercentageF32,
    key_black: PercentageF32,
) -> (
    Option<PercentageF32>,
    Option<PercentageF32>,
    Option<PercentageF32>,
) {
    let black_coefficient = PercentageF32::MAX - key_black;

    let f = |v| (PercentageF32::MAX - v) * black_coefficient;

    (
        R.then(|| f(cyan)),
        G.then(|| f(magenta)),
        B.then(|| f(yellow)),
    )
}
