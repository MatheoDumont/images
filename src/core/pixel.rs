
pub enum PixelFormat {
    RGB,
    GRAYSCALE
}

pub trait PixelType<T> {
    fn value(v: u8) -> T;
    fn zero() -> T;
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}


pub struct Grayscale {
    pub intensity: u8,
}

impl PixelType for RGB {

}