
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
pub extern crate trap;
pub extern crate glutin;


pub use std::f64::consts::PI;

mod display;
pub use display::*;



type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

type Resources = gfx_device_gl::Resources;
type Factory = gfx_device_gl::Factory;
type Device = gfx_device_gl::Device;

