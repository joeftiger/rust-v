mod box_filter;
mod gaussian_filter;
mod lanczos_sinc_filter;
mod mitchell_filter;
mod triangle_filter;

pub use box_filter::*;
pub use gaussian_filter::*;
pub use lanczos_sinc_filter::*;
pub use mitchell_filter::*;
pub use triangle_filter::*;

use ultraviolet::Vec2;

#[typetag::serde]
pub trait Filter {
    fn radius(&self) -> Vec2;
    fn inv_radius(&self) -> Vec2;

    /// Evaluates a relative sample point to filter center position.
    ///
    /// # Constraints
    /// * `point` - All values should be finite (neither infinite nor `NaN`).
    ///             Should not be outside the filter's extent.
    ///
    /// # Arguments
    /// * `point` - The relative point to evaluate on the filter function
    ///
    /// # Returns
    /// * The evaluated value
    fn evaluate(&self, point: &Vec2) -> f32;
}
