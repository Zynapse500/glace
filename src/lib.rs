
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
pub extern crate trap;
pub extern crate glutin;


pub use std::f64::consts::PI;

mod display;
pub use display::*;

mod color;
pub use color::*;

mod draw;
pub use draw::*;

mod projection;
pub use projection::*;

mod vertex;
pub use vertex::*;

mod vertex_array;

mod pipeline;


mod frame;
pub use frame::*;


type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::Depth;
type DepthStencilFormat = gfx::format::DepthStencil;

type Resources = gfx_device_gl::Resources;
type Factory = gfx_device_gl::Factory;
type Device = gfx_device_gl::Device;

type RenderTargetView = gfx::handle::RenderTargetView<Resources, ColorFormat>;
type DepthStencilView = gfx::handle::DepthStencilView<Resources, DepthFormat>;

type CommandBuffer = gfx_device_gl::CommandBuffer;

type Encoder = gfx::Encoder<Resources, CommandBuffer>;
type PipelineState = gfx::PipelineState<Resources, pipeline::pipe::Meta>;