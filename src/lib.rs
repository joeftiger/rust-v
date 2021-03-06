#[macro_use]
extern crate bitflags;

#[cfg(feature = "show-image")]
pub use window::RenderWindow;

pub mod bxdf;

pub mod camera;
mod debug_utils;
pub mod integrator;
pub mod mc;
pub mod objects;
pub mod samplers;
pub mod scene;

pub mod filters;
pub mod refractive_index;

pub mod config;
pub mod renderer;
pub mod sensor;
pub mod serialization;
#[cfg(feature = "show-image")]
mod window;

pub type Spectrum = color::Spectrum;

#[cfg(not(feature = "f64"))]
type Float = f32;
#[cfg(not(feature = "f64"))]
type Vector2 = ultraviolet::Vec2;
#[cfg(not(feature = "f64"))]
type Vector3 = ultraviolet::Vec3;
#[cfg(not(feature = "f64"))]
type Rotation3 = ultraviolet::Rotor3;
#[cfg(feature = "f64")]
type Float = f64;
#[cfg(feature = "f64")]
type Vector2 = ultraviolet::DVec2;
#[cfg(feature = "f64")]
type Vector3 = ultraviolet::DVec3;
#[cfg(feature = "f64")]
type Rotation3 = ultraviolet::DRotor3;
