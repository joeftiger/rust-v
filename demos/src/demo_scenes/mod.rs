mod cornell;
mod debug;
mod prism;
mod spheres;

use ultraviolet::UVec2;

pub use cornell::CornellScene;
pub use debug::DebugScene;
pub use prism::PrismScene;

use rust_v::config::Config;
use rust_v::integrator::{Integrator, Whitted};
use rust_v::samplers::Sampler;
use rust_v::scene::Scene;

use rust_v::serialization::Serialization;
pub use spheres::SphereScene;

#[cfg(not(feature = "f64"))]
type Float = f32;
#[cfg(not(feature = "f64"))]
type Vector3 = ultraviolet::Vec3;
#[cfg(feature = "f64")]
type Float = f64;
#[cfg(feature = "f64")]
type Vector3 = ultraviolet::DVec3;

pub const SIGMA: Float = 20.0;
pub const FOVY: Float = 70.0;

pub trait Demo {
    // TODO: WIP
    fn create() -> Serialization;

    fn empty() -> (UVec2, Config, Box<dyn Integrator>, Sampler, Scene) {
        let resolution = UVec2::new(512, 512);
        let config = Config {
            filename: None,
            bounds: None,
            block_size: UVec2::broadcast(8),
            passes: 100,
            threads: None,
        };

        let integrator = Box::new(Whitted::new(8));
        let sampler = Sampler::Random;
        let scene = Scene::default();

        (resolution, config, integrator, sampler, scene)
    }
}
