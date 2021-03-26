mod perspective;
pub mod perspective_simone;

pub use perspective::PerspectiveCamera;

use geometry::Ray;
use serde::{Deserialize, Serialize};
use ultraviolet::UVec2;

// #[derive(Serialize, Deserialize)]
// pub enum CameraType {
//     Dummy,
//     Perspective(PerspectiveCamera),
// }
//
// impl Camera for CameraType {
//     fn primary_ray(&mut self, pixel: UVec2) -> Ray {
//         match self {
//             CameraType::Dummy => unimplemented!(),
//             CameraType::Perspective(c) => c.primary_ray(pixel),
//         }
//     }
// }

#[typetag::serde]
pub trait Camera: Send + Sync {
    fn resolution(&self) -> UVec2;

    /// Creates a new primary ray of the given pixel.
    ///
    /// # Constraints
    /// * `pixel` - Should be within the camera's resolution.
    ///
    /// # Arguments
    /// * `pixel` - The pixel to generate the ray from
    ///
    /// # Returns
    /// * A ray
    fn primary_ray(&mut self, pixel: UVec2) -> Ray;
}

#[derive(Serialize, Deserialize)]
pub struct NoOpCamera;

#[typetag::serde]
impl Camera for NoOpCamera {
    fn resolution(&self) -> UVec2 {
        unimplemented!()
    }

    fn primary_ray(&mut self, _: UVec2) -> Ray {
        unimplemented!()
    }
}
