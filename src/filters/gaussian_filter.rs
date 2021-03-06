use crate::filters::Filter;
use crate::*;

use serde::{Deserialize, Serialize};

/// Sample weights considered with a Gaussian bump.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct GaussianFilter {
    pub radius: Vector2,
    alpha: Float,
    exp: Vector2,
}

impl GaussianFilter {
    pub fn new(radius: Vector2, alpha: Float) -> Self {
        let exp = -alpha * radius * radius;

        Self { radius, alpha, exp }
    }

    #[inline]
    fn gaussian(&self, point: Float, exp: Float) -> Float {
        Float::max(0.0, Float::exp(-self.alpha * point * point) - exp)
    }
}

#[typetag::serde]
impl Filter for GaussianFilter {
    fn evaluate(&self, point: Vector2) -> Float {
        self.gaussian(point.x, self.exp.x) * self.gaussian(point.y, self.exp.y)
    }
}
