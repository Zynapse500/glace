

#[derive(Copy, Clone)]
#[derive(Debug)]
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


impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            self.r as f32,
            self.g as f32,
            self.b as f32,
            self.a as f32
        ]
    }
}

impl Into<[f64; 4]> for Color {
    fn into(self) -> [f64; 4] {
        [
            self.r,
            self.g,
            self.b,
            self.a
        ]
    }
}
