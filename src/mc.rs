use crate::debug_utils::within_01;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};
use ultraviolet::{Vec2, Vec3};

/// # Summary
/// Samples a concentric mapped point from the given random sample.
///
/// # Constraints
/// * `sample` - All values should be within `[0, 1]`.
///
/// # Arguments
/// * `sample` - A random sample in `[0, 1]`
///
/// # Results
/// * `Vec2` - A concentric sample
pub fn concentric_sample_disk(sample: &Vec2) -> Vec2 {
    debug_assert!(within_01(sample));

    // Map uniform random numbers to [-1,1]^2
    let offset = 2.0 * *sample - Vec2::one();

    // Handle degeneracy at the origin
    if offset.x == 0.0 || offset.y == 0.0 {
        return Vec2::zero();
    }

    // Apply concentric mapping to point
    let r;
    let theta;
    if offset.x.abs() > offset.y.abs() {
        r = offset.x;
        theta = FRAC_PI_4 * offset.y / offset.x;
    } else {
        r = offset.y;
        theta = FRAC_PI_2 - FRAC_PI_4 * offset.x / offset.y;
    }

    r * Vec2::new(theta.cos(), theta.sin())
}

/// # Summary
/// Samples a hemisphere with a cosine distribution described by the sample.
///
/// # Constraints
/// * `sample` - All values should be within `[0, 1]`.
///
/// # Arguments
/// * `sample` - A random sample in `[0, 1]`
///
/// # Results
/// * `Vec3` - A point on the hemisphere around `(0, 0, 1)`
pub fn cos_sample_hemisphere(sample: &Vec2) -> Vec3 {
    debug_assert!(within_01(sample));

    let d = concentric_sample_disk(sample);
    let z = f32::max(0.0, 1.0 - d.x * d.x - d.y * d.y).sqrt();

    Vec3::new(d.x, d.y, z)
}

/// # Summary
/// Samples a sphere with a uniform distribution described by the sample.
///
/// # Constraints
/// * `sample` - All values should be within `[0, 1]`.
///
/// # Arguments
/// * `sample` - A random sample in `[0, 1]`
///
/// # Results
/// * `Vec3` - A point on the sphere around `(0, 0, 0)`
pub fn uniform_sample_sphere(sample: &Vec2) -> Vec3 {
    debug_assert!(within_01(sample));

    let z = 1.0 - 2.0 * sample.x;
    let r = f32::max(0.0, 1.0 - z * z).sqrt();
    let phi = PI * 2.0 * sample.y;

    Vec3::new(phi.cos() * r, phi.sin() * r, z)
}

// /// # Summary
// /// Samples a cone around the `(0, 1, 0)` axis with a uniform distribution described by the sample.
// ///
// /// # Constraints
// /// * `sample` - All values should be within `[0, 1]`.
// ///
// /// # Arguments
// /// * `sample` - A random sample in `[0, 1]`
// /// * `cos_theta_max` - The max angle
// ///
// /// # Results
// /// * `Vec3` - A direction in the cone around `(0, 1, 0)`
// pub fn uniform_sample_cone(sample: &Vec2, cos_theta_max: f32) -> Vec3 {
//     debug_assert!(within_01(sample));
//
//     let cos = cos_theta_max.lerp(1.0, sample.x);
//     let sin = f32::sqrt(1.0 - cos * cos);
//     let phi = sample.y * TAU;
//
//     // TODO: This is weird: The commented out version should be correct but results in non-correct images
//     // Maybe there is a bug somewhere else?
//
//     Vec3::new(phi.cos() * sin, cos, phi.sin() * sin)
//     // Vec3::new(phi.cos() * sin, phi.sin() * sin, cos)
// }
//
// /// # Summary
// /// Samples a cone around the `frame.e2` axis with a uniform distribution described by the sample.
// ///
// /// # Constraints
// /// * `sample` - All values should be within `[0, 1]`.
// ///
// /// # Arguments
// /// * `sample` - A random sample in `[0, 1]`
// /// * `cos_theta_max` - The max angle
// /// * `frame` - The coordinate system frame. Y-axis is "up"-axis.
// ///
// /// # Results
// /// * `Vec3` - A direction in the cone around `frame.e2`
// pub fn uniform_sample_cone_frame(
//     sample: &Vec2,
//     cos_theta_max: f32,
//     frame: &CoordinateSystem,
// ) -> Vec3 {
//     debug_assert!(within_01(sample));
//
//     let cos = cos_theta_max.lerp(1.0, sample.x);
//     let sin = f32::sqrt(1.0 - cos * cos);
//     let phi = sample.y * 2.0 * PI;
//
//     (phi.cos() * sin * frame.x) - (cos * frame.y) + (phi.sin() * sin * frame.z)
// }

/// # Summary
/// Computes the pdf for uniformly sampling a code.
///
/// # Arguments
/// * `cos_theta` - The cone angle
///
/// # Results
/// * `f32` - The pdf
pub fn uniform_cone_pdf(cos_theta: f32) -> f32 {
    1.0 / (TAU * (1.0 - cos_theta))
}
