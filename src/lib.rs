#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Rgba32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba32 {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn new_grey(x: u8) -> Self {
        Self {
            r: x,
            g: x,
            b: x,
            a: 255,
        }
    }

    pub fn to_f32_array_01(self) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            self.a as f32 / 255.,
        ]
    }

    pub fn with_r(self, r: u8) -> Self {
        Self { r, ..self }
    }

    pub fn with_g(self, g: u8) -> Self {
        Self { g, ..self }
    }

    pub fn with_b(self, b: u8) -> Self {
        Self { b, ..self }
    }

    pub fn with_a(self, a: u8) -> Self {
        Self { a, ..self }
    }

    pub const fn linear_interpolate(self, to: Rgba32, by: u8) -> Self {
        const fn interpolate_channel(from: u8, to: u8, by: u8) -> u8 {
            let total_delta = to as i32 - from as i32;
            let current_delta = (total_delta * by as i32) / 255;
            (from as i32 + current_delta) as u8
        }
        Self {
            r: interpolate_channel(self.r, to.r, by),
            g: interpolate_channel(self.g, to.g, by),
            b: interpolate_channel(self.b, to.b, by),
            a: interpolate_channel(self.a, to.a, by),
        }
    }

    pub fn alpha_composite(self, below: Rgba32) -> Rgba32 {
        fn mul_u8(a: u8, b: u8) -> u8 {
            ((a as u16 * b as u16) / 255) as u8
        }
        fn div_u8(a: u8, b: u8) -> u8 {
            ((255 * a as u16) / b as u16) as u8
        }
        let alpha_out_rhs = mul_u8(below.a, 255 - self.a);
        let alpha_out = self.a + alpha_out_rhs;
        let single_channel =
            |c_a: u8, c_b: u8| div_u8(mul_u8(c_a, self.a) + mul_u8(c_b, alpha_out_rhs), alpha_out);
        Self {
            r: single_channel(self.r, below.r),
            g: single_channel(self.g, below.g),
            b: single_channel(self.b, below.b),
            a: alpha_out,
        }
    }
}

pub const fn rgba32(r: u8, g: u8, b: u8, a: u8) -> Rgba32 {
    Rgba32::new(r, g, b, a)
}

pub const fn rgba32_rgb(r: u8, g: u8, b: u8) -> Rgba32 {
    Rgba32::new_rgb(r, g, b)
}

pub const fn rgba32_grey(x: u8) -> Rgba32 {
    Rgba32::new_grey(x)
}
