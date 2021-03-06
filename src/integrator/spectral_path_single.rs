use crate::bxdf::Type;
use crate::integrator::{direct_illumination_wavelength, DirectLightStrategy, Integrator};
use crate::objects::SceneObject;
use crate::samplers::spectral_samplers::SpectralSampler;
use crate::samplers::Sampler;
use crate::scene::{Scene, SceneIntersection};
use crate::sensor::pixel::Pixel;
use crate::Float;
use geometry::{offset_ray_towards, Ray};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectralPathSingle {
    max_depth: u32,
    light_wave_samples: u32,
    direct_light_strategy: DirectLightStrategy,
    spectral_sampler: SpectralSampler,
}

impl SpectralPathSingle {
    fn trace_single(
        &self,
        scene: &Scene,
        mut hit: SceneIntersection,
        sampler: Sampler,
        index: usize,
    ) -> Float {
        let mut illumination = 0.0;
        let mut throughput = 1.0;

        for _ in 0..self.max_depth {
            let outgoing = -hit.ray.direction;
            let normal = hit.normal;
            let bsdf = hit.object.bsdf();

            if let SceneObject::Emitter(e) = &hit.object {
                illumination += throughput * e.emission[index];
            } else {
                illumination += throughput
                    * direct_illumination_wavelength(
                        scene,
                        sampler,
                        self.direct_light_strategy,
                        &hit,
                        bsdf,
                        index,
                    );
            }

            if let Some(bxdf_sample) =
                bsdf.sample_light_wave(normal, outgoing, Type::ALL, sampler.get_sample(), index)
            {
                if bxdf_sample.pdf == 0.0 || bxdf_sample.spectrum == 0.0 {
                    break;
                }

                let cos_abs = if bxdf_sample.typ.is_specular() {
                    // division of cosine omitted in specular bxdfs
                    1.0
                } else {
                    bxdf_sample.incident.dot(normal).abs()
                };

                throughput *= bxdf_sample.spectrum * cos_abs / bxdf_sample.pdf;

                let ray = offset_ray_towards(hit.point, hit.normal, bxdf_sample.incident);
                match scene.intersect(&ray) {
                    Some(i) => hit = i,
                    None => break,
                }
            } else {
                break;
            }
        }

        illumination
    }
}

#[typetag::serde]
impl Integrator for SpectralPathSingle {
    fn integrate(&self, pixel: &mut Pixel, scene: &Scene, primary_ray: &Ray, sampler: Sampler) {
        if let Some(hit) = scene.intersect(primary_ray) {
            let mut indices = vec![0; self.light_wave_samples as usize];

            self.spectral_sampler.fill_samples(&mut indices);

            for index in indices {
                let lambda = self.trace_single(scene, hit.clone(), sampler, index);
                pixel.add_light_wave(lambda, index);
            }
        } else {
            pixel.add_black();
        }
    }
}
