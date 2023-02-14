use super::pixel::PixelType;

pub struct NDMatrix<PIXELTYPE: PixelType, const N: usize, const D: usize> {
    data: [[PIXELTYPE; D]; N],
}


impl<PIXELTYPE: PixelType, const N: usize, const D: usize> NDMatrix<PIXELTYPE, N, D> {
    pub fn value(v: u8) -> NDMatrix<PIXELTYPE, N, D> {
        NDMatrix {
            data: [[PIXELTYPE::value(v); D]; N]
        }
    }
}