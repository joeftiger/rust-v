#![allow(clippy::upper_case_acronyms)]

mod bsdf;
mod diffuse;
mod fresnel;
mod lambertian;
mod microfacet;
mod oren_nayar;
mod specular;

pub use bsdf::BSDF;

pub use diffuse::*;
pub use fresnel::*;
pub use lambertian::*;
pub use microfacet::*;
pub use oren_nayar::*;
pub use specular::*;

use crate::debug_utils::{is_finite, is_normalized, within_01};
use crate::mc::sample_unit_hemisphere;
use crate::Spectrum;
use crate::*;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "f64"))]
use std::f32::consts::{FRAC_1_PI, PI};
#[cfg(feature = "f64")]
use std::f64::consts::{FRAC_1_PI, PI};
use std::ops::Mul;
use utility::floats::FloatExt;

/// A rotation is either
/// - not happening
/// - flipping direction
/// - some rotation
#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    None,
    Flip,
    Some(Rotation3),
}

impl Rotation {
    /// Reversed the rotation (if any).
    #[inline]
    pub fn reversed(&self) -> Self {
        match self {
            Rotation::Some(r) => Self::Some(r.reversed()),
            _ => *self,
        }
    }
}

impl Mul<Vector3> for Rotation {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        match self {
            Rotation::None => rhs,
            Rotation::Flip => flip(rhs),
            Rotation::Some(r) => r * rhs,
        }
    }
}

/// Allows indicating whether an intersection was found along a path starting from a camera or one
/// starting from a light source.
///
/// This has implications on the calculations of `BSDF`.
#[derive(PartialEq)]
pub enum TransportMode {
    Radiance,
    Importance,
}

/// The BxDF normal is defined in the y-axis of the world space.
///
/// # Returns
/// * The global BxDF normal
#[inline(always)]
pub fn bxdf_normal() -> Vector3 {
    Vector3::unit_y()
}

#[inline(always)]
pub fn bxdf_incident_to(v: Vector3) -> Vector3 {
    debug_assert!(is_finite(v));

    Vector3::new(-v.x, v.y, -v.z)
}

#[inline(always)]
pub fn is_neg(v: Vector3) -> bool {
    debug_assert!(is_finite(v));

    v.y < 0.0
}

#[inline(always)]
pub fn flip_if_neg(mut v: Vector3) -> Vector3 {
    debug_assert!(is_finite(v));

    if is_neg(v) {
        v.y = -v.y;
    }
    v
}

#[inline(always)]
pub fn flip(mut v: Vector3) -> Vector3 {
    debug_assert!(is_finite(v));

    v.y = -v.y;
    v
}

#[inline(always)]
pub fn bxdf_is_parallel(v: Vector3) -> bool {
    debug_assert!(is_finite(v));

    v.y == 0.0
}

#[inline(always)]
pub fn cos_theta(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    v.y
}

#[inline(always)]
pub fn cos2_theta(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    cos_theta(v) * cos_theta(v)
}

#[inline(always)]
pub fn sin2_theta(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    0.0.fast_max(1.0 - cos2_theta(v))
}

#[inline(always)]
pub fn sin_theta(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    sin2_theta(v).sqrt()
}

#[inline(always)]
pub fn tan_theta(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    sin_theta(v) / cos_theta(v)
}

#[inline(always)]
pub fn tan2_theta(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    sin2_theta(v) / cos2_theta(v)
}

#[inline(always)]
pub fn cos_phi(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    let sin_theta = sin_theta(v);
    if sin_theta == 0.0 {
        0.0
    } else {
        Float::fast_clamp(v.x / sin_theta, -1.0, 1.0)
    }
}

#[inline(always)]
pub fn sin_phi(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    let sin_theta = sin_theta(v);
    if sin_theta == 0.0 {
        0.0
    } else {
        Float::fast_clamp(v.z / sin_theta, -1.0, 1.0)
    }
}

#[inline(always)]
pub fn cos2_phi(v: Vector3) -> Float {
    debug_assert!(is_finite(v));

    let cos_phi = cos_phi(v);
    cos_phi * cos_phi
}

#[inline(always)]
pub fn sin2_phi(v: Vector3) -> Float {
    let sin_phi = sin_phi(v);
    sin_phi * sin_phi
}

#[inline]
pub fn cos_d_phi(a: Vector3, b: Vector3) -> Float {
    debug_assert!(is_finite(a));
    debug_assert!(is_finite(b));

    let abxz = a.x * b.x + a.z * b.z;
    let axz = a.x * a.x + a.z * a.z;
    let bxz = b.x * b.x + b.z * b.z;

    Float::fast_clamp(abxz / Float::sqrt(axz * bxz), -1.0, 1.0)
}

#[inline]
pub fn refract(v: Vector3, n: Vector3, eta: Float) -> Option<Vector3> {
    let cos_i = n.dot(v);
    let sin_t2 = eta * eta * cos_i.mul_add(-cos_i, 1.0).fast_max(0.0);

    if sin_t2 > 1.0 {
        None
    } else {
        let cos_t = Float::sqrt(1.0 - sin_t2);
        let right = eta.mul_add(cos_i, -cos_t);
        let r = eta * -v + right * n;

        Some(r)
    }
}

#[inline(always)]
pub fn face_forward(v: Vector3, n: Vector3) -> Vector3 {
    if n.dot(v) > 0.0 {
        v
    } else {
        -v
    }
}

#[inline(always)]
pub fn same_hemisphere(a: Vector3, b: Vector3) -> bool {
    debug_assert!(is_finite(a));
    debug_assert!(is_finite(b));

    a.y * b.y > 0.0
}

#[inline(always)]
pub fn world_to_bxdf(v: Vector3) -> Rotation {
    debug_assert!(is_finite(v));

    if v == Vector3::unit_y() {
        Rotation::None
    } else if v == -Vector3::unit_y() {
        Rotation::Flip
    } else {
        Rotation::Some(Rotation3::from_rotation_between(v, bxdf_normal()))
    }
}

#[inline]
pub fn bxdf_to_world(v: Vector3) -> Rotation3 {
    debug_assert!(is_finite(v));

    if v == Vector3::unit_y() {
        Rotation3::default()
    } else if v == -Vector3::unit_y() {
        Rotation3::from_rotation_xy(-PI)
    } else {
        Rotation3::from_rotation_between(bxdf_normal(), v)
    }
}

bitflags! {
    /// All BxDF have types, for example
    /// * `BRDF` - Bidirectional Reflection Function
    /// * `BTDF` - Bidirectional Transmission Function
    /// All these have properties like being `reflective`, `transmissive`, `diffuse`, `glossy`,
    /// `specular` or be any mixture in between.
    ///
    /// This type allows bit-flagging these properties at will.
    pub struct Type: u8 {
        const NONE = 1 << 0;
        const REFLECTION = 1 << 1;
        const TRANSMISSION = 1 << 2;
        const DIFFUSE = 1 << 3;
        const GLOSSY = 1 << 4;
        const SPECULAR = 1 << 5;
        const ALL = Self::REFLECTION.bits | Self::TRANSMISSION.bits | Self::DIFFUSE.bits | Self::GLOSSY.bits | Self::SPECULAR.bits;
    }
}

impl Type {
    /// Returns whether this type is `reflective`.
    ///
    /// # Returns
    /// * Is `reflective`
    pub fn is_reflection(&self) -> bool {
        *self & Self::REFLECTION == Self::REFLECTION
    }

    /// Returns whether this type is `transmissive`.
    ///
    /// # Returns
    /// * Is `transmissive`
    pub fn is_transmission(&self) -> bool {
        *self & Self::TRANSMISSION == Self::TRANSMISSION
    }

    /// Returns whether this type is `diffuse`.
    ///
    /// # Returns
    /// * Is `diffuse`
    pub fn is_diffuse(&self) -> bool {
        *self & Self::DIFFUSE == Self::DIFFUSE
    }

    /// Returns whether this type is `glossy`.
    ///
    /// # Returns
    /// * Is `glossy`
    pub fn is_glossy(&self) -> bool {
        *self & Self::GLOSSY == Self::GLOSSY
    }

    /// Returns whether this type is `specular`.
    ///
    /// # Returns
    /// * Is `specular`
    pub fn is_specular(&self) -> bool {
        *self & Self::SPECULAR == Self::SPECULAR
    }
}

/// The result of sampling a BxDF with multiple wavelengths.
///
/// If the sampling was wavelength-independent, it is of type `Bundle`.
/// Otherwise it's of type `Buffer` and needs to be handled accordingly.
pub enum BxDFSampleResult {
    Bundle(BxDFSample<Vec<Float>>),
    ScatteredBundle(Vec<BxDFSampleIndex>),
}

/// Contains of
/// * `intensity` - An evaluated scaling intensity.
/// * `incident` - An evaluated incident direction.
/// * `pdf` - An evaluated pdf.
/// * `typ` - The sampled `Type`.
/// * `index` - The wavelength index.
pub struct BxDFSampleIndex {
    pub intensity: Float,
    pub incident: Vector3,
    pub pdf: Float,
    pub typ: Type,
    pub index: usize,
}

impl BxDFSampleIndex {
    pub fn new(intensity: Float, incident: Vector3, pdf: Float, typ: Type, index: usize) -> Self {
        Self {
            intensity,
            incident,
            pdf,
            typ,
            index,
        }
    }
}

/// Contains of
/// * `spectrum` - An evaluated scaling spectrum
/// * `incident` - An evaluated incident direction
/// * `pdf` - An evaluated pdf
/// * `typ` - The sampled `Type`
pub struct BxDFSample<T> {
    pub spectrum: T,
    pub incident: Vector3,
    pub pdf: Float,
    pub typ: Type,
}

impl<T> BxDFSample<T> {
    /// Creates a new sample.
    ///
    /// # Constraints
    /// * `incident` - Should be normalized.
    ///
    /// # Arguments
    /// * `spectrum` - The spectrum type
    /// * `incident` The incident vector
    /// * `pdf` - The probability density function value
    /// * `typ` - The bxdf type
    ///
    /// # Returns
    /// * Self
    pub fn new(spectrum: T, incident: Vector3, pdf: Float, typ: Type) -> Self {
        debug_assert!(is_normalized(incident));

        Self {
            spectrum,
            incident,
            pdf,
            typ,
        }
    }
}

/// The common base shared between BRDFs and BTDFs.
/// Provides methods for evaluating and sampling the distribution function for pairs of directions
/// at an intersection.
#[typetag::serde]
pub trait BxDF: Send + Sync {
    /// Returns the type of this bxdf.
    ///
    /// # Results
    /// * The type of this BxDF
    fn get_type(&self) -> Type;

    /// Allows matching the user-supplied type to this BxDF.
    ///
    /// # Results
    /// * Whether the type matches.
    fn is_type(&self, t: Type) -> bool {
        let st = self.get_type();
        (st & t) == st
    }

    /// Evaluates the BxDF for the pair of incident and outgoing light directions and the
    /// intersection normal.
    ///
    /// # Constraints
    /// * `incident` - All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `outgoing` - All values should be finite.
    ///                Should be normalized.
    ///
    /// # Arguments
    /// * `incident` - The incident direction onto the intersection we evaluate
    /// * `outgoing` - The outgoing light direction
    ///
    /// # Results
    /// * A scaling spectrum
    fn evaluate(&self, incident: Vector3, outgoing: Vector3) -> Spectrum;

    fn evaluate_buf(&self, incident: Vector3, outgoing: Vector3, indices: &[usize]) -> Vec<Float> {
        indices
            .iter()
            .map(|&i| self.evaluate_wavelength(incident, outgoing, i))
            .collect()
    }

    fn evaluate_wavelength(&self, incident: Vector3, outgoing: Vector3, index: usize) -> Float;

    /// Samples an incident light direction for an outgoing light direction from the given sample
    /// space.
    ///
    /// # Constraints
    /// * `outgoing` - All values should be finite.
    ///                Should be normalized.
    /// * `sample` - All values should be within `[0, 1]`.
    ///
    /// # Arguments
    /// * `outgoing` - The outgoing light direction
    /// * `sample` - The sample space for randomization
    ///
    /// # Results
    /// * The sampled spectrum, incident and pdf
    fn sample(&self, outgoing: Vector3, sample: Vector2) -> Option<BxDFSample<Spectrum>> {
        debug_assert!(is_normalized(outgoing));
        debug_assert!(within_01(sample));

        let incident = flip_if_neg(sample_unit_hemisphere(sample));
        let spectrum = self.evaluate(incident, outgoing);
        let pdf = self.pdf(incident, outgoing);

        Some(BxDFSample::new(spectrum, incident, pdf, self.get_type()))
    }

    fn sample_buf(
        &self,
        outgoing: Vector3,
        sample: Vector2,
        indices: &[usize],
    ) -> Option<BxDFSampleResult> {
        debug_assert!(is_normalized(outgoing));
        debug_assert!(within_01(sample));

        let incident = flip_if_neg(sample_unit_hemisphere(sample));
        let spectrum = self.evaluate_buf(incident, outgoing, indices);
        let pdf = self.pdf(incident, outgoing);

        Some(BxDFSampleResult::Bundle(BxDFSample::new(
            spectrum,
            incident,
            pdf,
            self.get_type(),
        )))
    }

    fn sample_wavelength(
        &self,
        outgoing: Vector3,
        sample: Vector2,
        light_wave_index: usize,
    ) -> Option<BxDFSample<Float>> {
        debug_assert!(is_normalized(outgoing));
        debug_assert!(within_01(sample));
        debug_assert!(light_wave_index < Spectrum::size());

        let incident = flip_if_neg(sample_unit_hemisphere(sample));
        let lambda = self.evaluate_wavelength(incident, outgoing, light_wave_index);
        let pdf = self.pdf(incident, outgoing);

        Some(BxDFSample::new(lambda, incident, pdf, self.get_type()))
    }

    /// Computes the probability density function (`pdf`) for the pair of directions.
    ///
    /// # Constraints
    /// * `incident` - All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `outgoing` - All values should be finite.
    ///                Should be normalized.
    ///
    /// # Arguments
    /// * `incident` - The incident direction onto the intersection we evaluate
    /// * `outgoing` - The outgoing light direction
    ///
    /// # Results
    /// * The evaluated pdf
    #[inline]
    fn pdf(&self, incident: Vector3, outgoing: Vector3) -> Float {
        if same_hemisphere(incident, outgoing) {
            cos_theta(incident).abs() * FRAC_1_PI
        } else {
            0.0
        }
    }
}

/// This special BxDF scales all spectrum outputs of another one, effectively wrapping around
/// some `BxDF` with a `scale`.
#[derive(Serialize, Deserialize)]
pub struct ScaledBxDF {
    bxdf: Box<dyn BxDF>,
    scale: Spectrum,
}

impl ScaledBxDF {
    /// Creates a new scaled `BxDF`.
    ///
    /// # Arguments
    /// * `bxdf` - The `BxDF`
    /// * `scale` - The scale
    ///
    /// # Returns
    /// * Self
    pub fn new(bxdf: Box<dyn BxDF>, scale: Spectrum) -> Self {
        Self { bxdf, scale }
    }
}

#[typetag::serde]
impl BxDF for ScaledBxDF {
    fn get_type(&self) -> Type {
        self.bxdf.get_type()
    }

    fn evaluate(&self, view: Vector3, from: Vector3) -> Spectrum {
        self.scale * self.bxdf.evaluate(view, from)
    }

    fn evaluate_wavelength(
        &self,
        incident: Vector3,
        outgoing: Vector3,
        light_wave_index: usize,
    ) -> Float {
        self.scale[light_wave_index]
            * self
                .bxdf
                .evaluate_wavelength(incident, outgoing, light_wave_index)
    }

    fn sample(&self, outgoing: Vector3, sample: Vector2) -> Option<BxDFSample<Spectrum>> {
        self.bxdf.sample(outgoing, sample).map(|mut s| {
            s.spectrum *= self.scale;

            s
        })
    }

    fn pdf(&self, incident: Vector3, outgoing: Vector3) -> Float {
        self.bxdf.pdf(incident, outgoing)
    }
}
