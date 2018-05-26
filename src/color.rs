

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}


impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color{ r, g, b, a }
    }
}
