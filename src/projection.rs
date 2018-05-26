
use trap::Vector3;

#[derive(Copy, Clone)]
pub enum Projection {
    Perspective {
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64
    },

    Orthographic {
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
        near: f64,
        far: f64
    }
}


#[derive(Copy, Clone)]
pub enum View {
    LookAt {
        eye: Vector3,
        target: Vector3,
        up: Vector3
    },

    None
}
