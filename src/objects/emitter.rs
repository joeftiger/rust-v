use crate::bxdf::BSDF;
#[cfg(debug_assertions)]
use crate::debug_utils::{in_range_incl, is_finite, is_normalized, within_01};
use crate::objects::receiver::ReceiverExt;
use crate::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use geometry::{Aabb, Boundable, Geometry, Intersectable, Intersection, Ray};
use ultraviolet::{Vec2, Vec3};
use utility::floats;

/// # Summary
/// An emitter is a receiver that also emits light.
pub trait EmitterExt: ReceiverExt {
    /// # Summary
    /// Reinterprets this emitter as a receiver.
    ///
    /// # Safety
    /// This should theoretically be safe, as the emitter is of type receiver anyway!
    ///
    /// # Returns
    /// * self as a receiver
    unsafe fn as_receiver(&self) -> &dyn ReceiverExt;

    /// # Summary
    /// Returns the emission of this emitter.
    ///
    /// # Returns
    /// * The emission
    fn emission(&self) -> Spectrum;

    /// # Summary
    /// Returns the radiance of this emitter, comparing the incident and normal vector.
    ///
    /// # Constraints
    /// * `incident` - All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `normal` - All values should be finite.
    ///              Should be normalized.
    ///
    /// # Arguments
    /// * `incident` - The incident on the surface of an object
    /// * `normal` - The nromal on the surface of an object
    ///
    /// # Returns
    /// * The radiated spectrum
    #[inline]
    fn radiance(&self, incident: &Vec3, normal: &Vec3) -> Spectrum {
        debug_assert!(is_finite(incident));
        debug_assert!(is_normalized(incident));
        debug_assert!(is_finite(normal));
        debug_assert!(is_normalized(normal));

        let dot = incident.dot(*normal);

        if dot > 0.0 {
            self.emission()
        } else {
            Spectrum::new_const(0.0)
        }
    }

    /// # Summary
    /// Samples the emitter from a given point in space.
    ///
    /// # Constraints
    /// * `point` - ALl values should be finite (neither infinite nor `NaN`).
    /// * `sample` - All values should be within `[0, 1)`.
    ///
    /// # Arguments
    /// * `point` - The point from which we sample the emitter
    /// * `sample` - A random sample
    ///
    /// # Returns
    /// * An emitter sample
    fn sample(&self, point: &Vec3, sample: &Vec2) -> EmitterSample;
}

/// # Summary
/// An emitter is similar to a receiver, consisting of a geometry and a BSDF. Additionally, the
/// emitter also has an emission.
pub struct Emitter<'a, T> {
    geometry: T,
    bsdf: BSDF<'a>,
    emission: Spectrum,
}

impl<'a, T> Emitter<'a, T> {
    /// # Summary
    /// Creates a new emitter.
    ///
    /// # Arguments
    /// * `geometry` - The geometry of the emitter
    /// * `bsdf` - The BSDF of the emitter
    /// * `emission` - The emission of the emitter
    ///
    /// # Returns
    /// * Self
    pub fn new(geometry: T, bsdf: BSDF<'a>, emission: Spectrum) -> Self {
        Self {
            geometry,
            bsdf,
            emission,
        }
    }
}

impl<T> EmitterExt for Emitter<'_, T>
where
    T: Sampleable,
{
    unsafe fn as_receiver(&self) -> &dyn ReceiverExt {
        self
    }

    fn emission(&self) -> Spectrum {
        self.emission
    }

    fn sample(&self, point: &Vec3, sample: &Vec2) -> EmitterSample {
        debug_assert!(is_finite(point));
        debug_assert!(within_01(sample));

        let surface = self.geometry.sample_surface(&point, sample);

        let incident = (surface.point - *point).normalized();

        let from = point; // + intersection.info.normal * floats::EPSILON;
        let occlusion_tester = OcclusionTester::between(*from, surface.point);

        let pdf = self.geometry.pdf(&occlusion_tester.ray);
        let radiance = self.radiance(&incident, &surface.normal);

        EmitterSample::new(radiance, incident, pdf, occlusion_tester)
    }
}

impl<T> ReceiverExt for Emitter<'_, T>
where
    T: Geometry,
{
    fn geometry(&self) -> &dyn Geometry {
        &self.geometry
    }

    fn bsdf(&self) -> &BSDF<'_> {
        &self.bsdf
    }
}

impl<T> Boundable for Emitter<'_, T>
where
    T: Boundable,
{
    fn bounds(&self) -> Aabb {
        self.geometry.bounds()
    }
}

impl<T> Intersectable for Emitter<'_, T>
where
    T: Intersectable,
{
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.geometry.intersect(ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.geometry.intersects(ray)
    }
}

/// # Summary
/// An emitter sample consists of
/// * A `radiance` of the emitter
/// * An `incident` vector (normalized) towards the emitter
/// * A `pdf` (inside `[0, 1]`) that the emitter is hit
/// * An `occlusion tester` to test against a scene
pub struct EmitterSample {
    pub radiance: Spectrum,
    pub incident: Vec3,
    pub pdf: f32,
    pub occlusion_tester: OcclusionTester,
}

impl EmitterSample {
    /// # Summary
    /// Creates a new emitter sample.
    ///
    /// # Constraints
    /// * `incident` - All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `pdf` - Should be inside `[0, 1]`
    ///
    /// # Arguments
    /// * `radiance` - The radiance of the emitter
    /// * `incident` - The incident on the emitter
    /// * `pdf` - The pdf that the emitter is hit
    /// * `occlusion_tester` - A scene occlusion tester
    ///
    /// # Returns
    /// * Self
    pub fn new(
        radiance: Spectrum,
        incident: Vec3,
        pdf: f32,
        occlusion_tester: OcclusionTester,
    ) -> Self {
        debug_assert!(is_finite(&incident));
        debug_assert!(is_normalized(&incident));
        debug_assert!(in_range_incl(pdf, 0.0, 1.0));

        Self {
            radiance,
            incident,
            pdf,
            occlusion_tester,
        }
    }
}

/// # Summary
/// A simple occlusion tester to test a ray against a scene.
pub struct OcclusionTester {
    ray: Ray,
}

impl OcclusionTester {
    /// # Summary
    /// Creates a new occlusion tester between the two given points.
    /// The created ray partition will be clamped to `[e, distance - e]`, with `e` denoting an epsilon
    /// and `distance` the distance between the points.
    /// This is to work around floating point imprecision that might occur in the intersection code.
    ///
    /// # Constraints
    /// * `origin` - All values should be finite (neither infinite nor `NaNN`).
    /// * `target` - All values should be finite.
    ///
    /// # Arguments
    /// * `origin` - The origin of the occlusion tester
    /// * `target` - The target to test towards
    ///
    /// # Returns
    /// * Self
    pub fn between(origin: Vec3, target: Vec3) -> Self {
        debug_assert!(is_finite(&origin));
        debug_assert!(is_finite(&target));

        let dir = target - origin;
        let ray = Ray::new(
            origin,
            dir.normalized(),
            floats::BIG_EPSILON,
            dir.mag() - floats::BIG_EPSILON,
        );

        Self { ray }
    }

    /// # Summary
    /// Tests the contained ray against the scene.
    ///
    /// # Arguments
    /// * `scene` - The scene to test against
    ///
    /// # Returns
    /// * `true` - If the scene intersects
    /// * `false` - Otherwise
    pub fn test(&self, scene: &Scene) -> bool {
        scene.intersects(&self.ray)
    }

    /// # Summary
    /// Tests the contained ray against the scene.
    ///
    /// # Arguments
    /// * `scene` - The scene to test against
    ///
    /// # Returns
    /// * The scene intersection (if any)
    pub fn test_get<'a>(&self, scene: &'a Scene) -> Option<SceneIntersection<'a>> {
        scene.intersect(&self.ray)
    }
}

/// # Summary
/// Describes a `point` and `normal` of a sampled surface.
pub struct SurfaceSample {
    pub point: Vec3,
    pub normal: Vec3,
}

impl SurfaceSample {
    /// # Summary
    /// Creates a new surface sample.
    ///
    /// # Constraints
    /// * `point` - All values should be finite (neither infinite nor `NaN`).
    /// * `normal` - All values should be finite.
    ///              Should be normalized.
    ///
    /// # Arguments
    /// * `point` - The surface point
    /// * `normal` - The surface normal
    ///
    /// # Returns
    /// * Self
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        debug_assert!(is_finite(&point));
        debug_assert!(is_normalized(&normal));

        Self { point, normal }
    }
}

/// # Summary
/// Allows geometries to be sampled for a surface point.
pub trait Sampleable: Geometry {
    /// # Summary
    /// Returns the surface area of this object.
    ///
    /// # Returns
    /// * The surface area
    fn surface_area(&self) -> f32;

    /// # Summary
    /// Samples the surface from the given point in the "solid angle" form.
    ///
    /// # Constraints
    /// * `point` - All values should be finite (neither infinite nor `NaN`).
    /// * `sample` - ALl values should be inside `[0, 1)`.
    ///
    /// # Arguments
    /// * `point` - The point at which we look at the object
    /// * `sample` - A random sample
    ///
    /// # Returns
    /// * A surface sample
    fn sample_surface(&self, point: &Vec3, sample: &Vec2) -> SurfaceSample;

    /// # Summary
    /// Computes the pdf that the ray intersects this object.
    ///
    /// # Arguments
    /// * `ray` - The ray to intersect this object
    ///
    /// # Returns
    /// * The pdf
    fn pdf(&self, ray: &Ray) -> f32;
}