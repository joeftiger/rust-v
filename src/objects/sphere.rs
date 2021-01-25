use crate::bxdf::bxdf_to_world;
use crate::debug_utils::{is_finite, within_01};
use crate::mc::{
    sample_cone, sample_unit_disk, sample_unit_disk_concentric, sample_unit_sphere,
    sample_vector_from_angle, uniform_cone_pdf, uniform_sample_cone_frame,
};
use crate::objects::emitter::SurfaceSample;
use crate::objects::Sampleable;
use geometry::{spherical_to_cartesian_frame_trig, CoordinateSystem, Intersectable, Ray, Sphere};
use std::f32::consts::{PI, TAU};
use ultraviolet::{Lerp, Vec2, Vec3};

fn sample_surface_inside(sphere: &Sphere, sample: &Vec2) -> SurfaceSample {
    let normal = sample_unit_sphere(sample);
    let point = sphere.center + sphere.radius * normal;
    let pdf = 1.0 / sphere.surface_area() / 10000000.0;

    SurfaceSample::new(point, normal, pdf)
}

impl Sampleable for Sphere {
    fn surface_area(&self) -> f32 {
        2.0 * TAU * self.radius * self.radius
    }

    // Copyright: https://github.com/mmp/pbrt-v3/blob/master/src/shapes/sphere.cpp
    fn sample_surface(&self, origin: &Vec3, sample: &Vec2) -> SurfaceSample {
        debug_assert!(is_finite(origin));
        debug_assert!(within_01(sample));

        let point_to_center = self.center - *origin;
        let dist_sq = point_to_center.mag_sq();
        let r2 = self.radius * self.radius;

        // return sample_surface_inside(&self, sample);

        if dist_sq <= r2 {
            // inside the sphere (may happen)
            sample_surface_inside(&self, sample)
        } else {
            let distance = dist_sq.sqrt();
            let axis = point_to_center / distance;
            let frame = CoordinateSystem::from_y(axis);

            // correct
            let cos_theta_max = f32::sqrt(1.0 - r2 / dist_sq) / 2.0;
            // debug_assert!(cos_theta_max > 0.0);

            // FIXME: multiplying with it makes everything go dark, since the radius get small.
            let disk_radius = self.radius * cos_theta_max;

            let disk = sample_unit_disk_concentric(sample) * disk_radius;
            let target = bxdf_to_world(axis) * Vec3::new(disk.x, 0.0, disk.y);
            let direction = (target - *origin).normalized();

            /* TRY 1 */
            // let direction = bxdf_to_world(axis) * sample_cone(sample, cos_theta_max);

            /* TRY 2 */
            let direction = uniform_sample_cone_frame(sample, cos_theta_max, &frame);

            /* PBR code */
            // let sin_theta_max = f32::sqrt(r2 / dist_sq);
            // let sin_theta_max2 = sin_theta_max * sin_theta_max;
            // let inv_sin_theta_max = 1.0 / sin_theta_max;
            // let cos_theta_max = f32::max(0.0, 1.0 - sin_theta_max2).sqrt();
            //
            // let cos_theta = cos_theta_max.lerp(1.0, sample.x);
            // let sin_theta2 = 1.0 - cos_theta * cos_theta;
            //
            // let cos_alpha = sin_theta2 * inv_sin_theta_max
            //     + cos_theta * f32::max(0.0, 1.0 - sin_theta2 * inv_sin_theta_max * inv_sin_theta_max).sqrt();
            // let sin_alpha = f32::max(0.0, 1.0 - cos_alpha * cos_alpha).sqrt();
            // let (sin_phi, cos_phi) = f32::sin_cos(sample.y * TAU);
            //
            // let direction = spherical_to_cartesian_frame_trig(sin_alpha, cos_alpha, sin_phi, cos_phi, &frame);
            /* end PBR code */

            /* TRY 3 */
            // let sin_theta_max = f32::sqrt(r2 / dist_sq);
            // let direction = sample_vector_from_angle(axis, sin_theta_max, sample);
            let theta = direction.dot(axis).acos();

            // sine rule
            let phi = PI - theta * (1.0 + distance / self.radius);
            let t = phi / theta * self.radius;

            let point = direction * t;
            let normal = (point - self.center).normalized();
            /* END TRY 3 */

            let pdf = uniform_cone_pdf(cos_theta_max);

            SurfaceSample::new(point, normal, pdf)

            // let ray = Ray::new_fast(*origin, direction);
            // match self.intersect(&ray) {
            //     None => {
            //         let t = dist_sq - r2;
            //         // let t = direction.dot(-*origin);
            //         debug_assert!(t >= 0.0);
            //
            //         let point = ray.at(t);
            //         debug_assert!(point != self.center);
            //
            //         let normal = (point - self.center).normalized();
            //
            //         // SurfaceSample::new(self.center, -point_to_center.normalized(), pdf)
            //         SurfaceSample::new(point, normal, pdf)
            //     }
            //     Some(i) => SurfaceSample::new(i.point, i.normal, pdf),
            // }
        }
    }
}
