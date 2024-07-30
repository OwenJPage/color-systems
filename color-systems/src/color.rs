use crate::{
    circle_degrees::CircleDegrees,
    models::color_model::ColorModel,
    percentage_f32::PercentageF32,
};

pub struct Color<C: ColorModel> {
    pub(super) color: C,
}

impl<C: ColorModel> Color<C> {
    #[inline]
    pub fn red(&self) -> u8 {
        let (r, ..) = self.color.select_rgb::<true, false, false>();

        r.expect("Red value was not returned")
    }

    #[inline]
    pub fn green(&self) -> u8 {
        let (_, g, _) = self.color.select_rgb::<false, true, false>();

        g.expect("Green value was not returned")
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        let (.., b) = self.color.select_rgb::<false, false, true>();

        b.expect("Blue value was not returned")
    }

    #[inline]
    pub fn red_float(&self) -> PercentageF32 {
        let (r, ..) = self.color.select_rgb_float::<true, false, false>();

        r.expect("Red value was not returned")
    }

    #[inline]
    pub fn green_float(&self) -> PercentageF32 {
        let (_, g, _) = self.color.select_rgb_float::<false, true, false>();

        g.expect("Green value was not returned")
    }

    #[inline]
    pub fn blue_float(&self) -> PercentageF32 {
        let (.., b) = self.color.select_rgb_float::<false, false, true>();

        b.expect("Red value was not returned")
    }

    #[inline]
    pub fn hue(&self) -> CircleDegrees {
        let (h, ..) = self.color.select_hsv::<true, false, false>();

        h.expect("Hue value was not returned")
    }

    #[inline]
    pub fn saturation_hsv(&self) -> PercentageF32 {
        let (_, s, _) = self.color.select_hsv::<false, true, false>();

        s.expect("Saturation value was not returned")
    }

    #[inline]
    pub fn saturation_hsl(&self) -> PercentageF32 {
        let (_, s, _) = self.color.select_hsl::<false, true, false>();

        s.expect("Saturation value was not returned")
    }

    #[inline]
    pub fn hsv_value(&self) -> PercentageF32 {
        let (.., v) = self.color.select_hsv::<false, false, true>();

        v.expect("Value value was not returned")
    }

    #[inline]
    pub fn luminosity(&self) -> PercentageF32 {
        let (.., l) = self.color.select_hsl::<false, false, true>();

        l.expect("Luminosity was not returned")
    }

    #[inline]
    pub fn cyan(&self) -> u8 {
        let (c, ..) = self.color.select_cmyk::<true, false, false, false>();

        c.expect("Cyan value was not returned")
    }

    #[inline]
    pub fn magenta(&self) -> u8 {
        let (_, m, ..) = self.color.select_cmyk::<false, true, false, false>();

        m.expect("Magenta value was not returned")
    }

    #[inline]
    pub fn yellow(&self) -> u8 {
        let (_, _, y, _) = self.color.select_cmyk::<false, false, true, false>();

        y.expect("Yellow value was not returned")
    }

    #[inline]
    pub fn key_black(&self) -> u8 {
        let (.., k) = self.color.select_cmyk::<false, false, false, true>();

        k.expect("Key value was not returned")
    }
}
