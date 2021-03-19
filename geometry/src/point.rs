use crate::{Aabb, Boundable, Geometry, Intersectable, Intersection, Ray};
use serde::{Deserialize, Serialize};
use ultraviolet::Vec3;

/// Represents a point in space.
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point(pub Vec3);

impl Boundable for Point {
    fn bounds(&self) -> Aabb {
        Aabb::new(self.0, self.0)
    }
}

impl Intersectable for Point {
    /// A point never intersects.
    ///
    /// # Arguments
    /// Ignored
    ///
    /// # Returns
    /// * `None`
    fn intersect(&self, _: &Ray) -> Option<Intersection> {
        None
    }

    /// A point never intersects.
    ///
    /// # Arguments
    /// Ignored
    ///
    /// # Returns
    /// * `false`
    fn intersects(&self, _: &Ray) -> bool {
        false
    }
}

#[typetag::serde]
impl Geometry for Point {}
