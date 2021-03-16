use crate::scene::{Scene, SceneIntersection};
use geometry::Ray;
use crate::sampler::Sampler;
use crate::Spectrum;

pub trait Integrator {
    fn render(scene: &Scene);
}

pub trait SamplerIntegrator: Integrator {
    fn illumination(&self, ray: &Ray, scene: &Scene, sampler: &dyn Sampler, depth: usize) -> Spectrum;
    fn specular_reflect(&self, ray: &Ray, intersection: &SceneIntersection, scene: &Scene, sampler: &dyn Sampler, depth: usize) -> Spectrum;
    fn specular_transmit(&self, ray: &Ray, intersection: &SceneIntersection, scene: &Scene, sampler: &dyn Sampler, depth: usize) -> Spectrum;

}