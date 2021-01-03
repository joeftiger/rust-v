use crate::objects::emitter::{Sampleable, SurfaceSample};
use geometry::{Point, Ray};
use ultraviolet::{Vec2, Vec3};

impl Sampleable for Point {
    /// # Summary
    /// A point has no surface area
    ///
    /// # Returns
    /// * `0.0`
    fn surface_area(&self) -> f32 {
        0.0
    }

    /// # Summary
    /// A point cannot be sampled plausibly.
    ///
    /// # Constraints
    /// * `point` - ALl values should be finite (neither infinite nor `NaN`).
    ///
    /// # Arguments
    /// * `point` - The point from which we sample the emitter
    /// * `_` - Ignored
    ///
    /// # Returns
    /// * An emitter sample with normal towards the `point`
    fn sample_surface(&self, point: &Vec3, _: &Vec2) -> SurfaceSample {
        let normal = self.0 - *point;

        SurfaceSample::new(self.0, normal.normalized())
    }

    /// # Summary
    /// A point cannot be sampled plausibly.
    ///
    /// # Arguments
    /// Ignored
    ///
    /// # Returns
    /// * `1.0`
    fn pdf(&self, _: &Ray) -> f32 {
        1.0
    }
}