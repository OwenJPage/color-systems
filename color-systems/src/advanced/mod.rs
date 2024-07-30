use std::mem::MaybeUninit;

pub type MatrixRow = [f64; 3];
pub type Matrix = [MatrixRow; 3];

fn apply_transformation(to: &MatrixRow, using: &Matrix) -> MatrixRow {
    [
        using[0][0] * to[0] + using[0][1] * to[1] + using[0][2] * to[2],
        using[1][0] * to[0] + using[1][1] * to[1] + using[1][2] * to[2],
        using[2][0] * to[0] + using[2][1] * to[1] + using[2][2] * to[2],
    ]
}

fn invert_matrix(input: &Matrix) -> Matrix {
    let atom_1 = input[1][1] * input[2][2] - input[1][2] * input[2][1];
    let atom_2 = input[1][0] * input[2][2];
    let atom_3 = input[1][2] * input[2][0];
    let atom_4 = input[1][0] * input[2][1] - input[1][1] * input[2][0];

    let determinant = input[0][0] * atom_1 - input[0][1] * (atom_2 - atom_3) + input[0][2] * atom_4;

    [
        [
            atom_1 / determinant,
            (input[0][2] * input[2][1] - input[0][1] * input[2][2]) / determinant,
            (input[0][1] * input[1][2] - input[0][2] * input[1][1]) / determinant,
        ],
        [
            (atom_3 - atom_2) / determinant,
            (input[0][0] * input[2][2] - input[0][2] * input[2][0]) / determinant,
            (input[0][2] * input[1][0] - input[0][0] * input[1][2]) / determinant,
        ],
        [
            atom_4 / determinant,
            (input[0][1] * input[2][0] - input[0][0] * input[2][1]) / determinant,
            (input[0][0] * input[1][1] - input[0][1] * input[1][0]) / determinant,
        ],
    ]
}

pub const D65_STANDARD_ILLUMINANT: MatrixRow = [96.047, 100., 108.833];

const SRGB_TO_XYZ_MATRIX: Matrix = [
    [0.4124564, 0.3575761, 0.1804375],
    [0.2126729, 0.7151522, 0.0721750],
    [0.0193339, 0.1191920, 0.9503041],
];

const XYZ_TO_SRGB_MATRIX: Matrix = [
    [3.2404542, -1.5371385, -0.4985314],
    [-0.9692660, 1.8760108, 0.0415560],
    [0.0556434, -0.2040259, 1.0572252],
];

pub struct RgbColourSpace {
    x_r:         f64,
    y_r:         f64,
    x_g:         f64,
    y_g:         f64,
    x_b:         f64,
    y_b:         f64,
    white_point: MatrixRow,
}

trait StaticZippable<E, Z, const N: usize> {
    type ZipType;

    fn zip(self, other: Z) -> Self::ZipType;
}

impl<E, F, const N: usize> StaticZippable<E, [F; N], N> for [E; N] {
    type ZipType = [(E, F); N];

    fn zip(self, other: [F; N]) -> Self::ZipType {
        let mut data: [MaybeUninit<(E, F)>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut self_iter = self.into_iter();
        let mut other_iter = other.into_iter();

        for elem in &mut data {
            elem.write((self_iter.next().unwrap(), other_iter.next().unwrap()));
        }

        data.map(|elem| unsafe { elem.assume_init() })
    }
}

impl<E: Copy, F: Copy, const N: usize> StaticZippable<E, &[F; N], N> for [E; N] {
    type ZipType = [(E, F); N];

    fn zip(self, other: &[F; N]) -> Self::ZipType {
        let mut data: [MaybeUninit<(E, F)>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            data[i].write((self[i], other[i]));
        }

        data.map(|elem| unsafe { elem.assume_init() })
    }
}

impl RgbColourSpace {
    fn get_xyz_transform_matrix(&self) -> Matrix {
        let rgb_matrix = [
            [
                self.x_r / self.y_r,
                self.x_g / self.y_g,
                self.x_b / self.y_b,
            ],
            [1., 1., 1.],
            [
                (1. - self.x_r - self.y_r) / self.y_r,
                (1. - self.x_g - self.y_g) / self.y_g,
                (1. - self.x_b - self.y_b) / self.y_b,
            ],
        ];

        let s_vector = apply_transformation(&self.white_point, &invert_matrix(&rgb_matrix));

        rgb_matrix.map(|row| row.zip(s_vector).map(|(r, s)| r * s))
    }
}

pub struct RgbXyzConverter {
    rgb_to_xyz: Matrix,
    xyz_to_rgb: Matrix,
}

impl RgbXyzConverter {
    pub const SRGB_CONVERTER: Self = Self::define_matrices(SRGB_TO_XYZ_MATRIX, XYZ_TO_SRGB_MATRIX);

    pub fn calculate_matrices(colour_space: ()) -> Self {
        todo!()
    }

    pub const fn define_matrices(rgb_to_xyz: Matrix, xyz_to_rgb: Matrix) -> Self {
        Self {
            rgb_to_xyz,
            xyz_to_rgb,
        }
    }
}
