use crate::bxdf::{same_hemisphere, world_to_bxdf, BxDF, BxDFSample, Type};
use crate::debug_utils::is_normalized;
use crate::sampler::Sample;
use crate::Spectrum;
use serde::{Deserialize, Serialize};
use ultraviolet::Vec3;

#[derive(Serialize, Deserialize)]
pub struct BSDF {
    bxdfs: Vec<Box<dyn BxDF>>,
}

impl BSDF {
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn new(bxdfs: Vec<Box<dyn BxDF>>) -> Self {
        Self { bxdfs }
    }

    pub fn size(&self) -> usize {
        self.bxdfs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn num_types(&self, t: Type) -> usize {
        self.bxdfs.iter().filter(|bxdf| bxdf.is_type(t)).count()
    }

    fn random_matching_bxdf(&self, t: Type, rand: f32) -> Option<&dyn BxDF> {
        let count = self.num_types(t);
        if count == 0 {
            return None;
        }

        let index = (rand * count as f32) as usize;
        self.bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(t) {
                    Some(bxdf.as_ref())
                } else {
                    None
                }
            })
            .nth(index)
    }

    pub fn evaluate(
        &self,
        normal: &Vec3,
        incident_world: &Vec3,
        outgoing_world: &Vec3,
        mut types: Type,
    ) -> Spectrum {
        let rotation = world_to_bxdf(normal);
        let incident = rotation * *incident_world;
        let outgoing = rotation * *outgoing_world;

        // transmission or reflection
        if same_hemisphere(&incident, &outgoing) {
            types &= !Type::TRANSMISSION;
        } else {
            types &= !Type::REFLECTION;
        }

        self.bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(types) {
                    Some(bxdf.evaluate(&incident, &outgoing))
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn evaluate_light_wave(
        &self,
        normal: &Vec3,
        incident_world: &Vec3,
        outgoing_world: &Vec3,
        mut types: Type,
        light_wave_index: usize,
    ) -> f32 {
        let rotation = world_to_bxdf(normal);
        let incident = rotation * *incident_world;
        let outgoing = rotation * *outgoing_world;

        // transmission or reflection
        if same_hemisphere(&incident, &outgoing) {
            types &= !Type::TRANSMISSION;
        } else {
            types &= !Type::REFLECTION;
        }

        self.bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(types) {
                    Some(bxdf.evaluate_light_wave(&incident, &outgoing, light_wave_index))
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn sample(
        &self,
        normal: &Vec3,
        outgoing_world: &Vec3,
        types: Type,
        sample: &Sample,
    ) -> Option<BxDFSample<Spectrum>> {
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * *outgoing_world;

        let bxdf = self.random_matching_bxdf(types, sample.one_d)?;

        if let Some(mut sample) = bxdf.sample(&outgoing, &sample.two_d) {
            sample.incident = rotation.reversed() * sample.incident;

            Some(sample)
        } else {
            None
        }
    }

    pub fn sample_light_wave(
        &self,
        normal: &Vec3,
        outgoing_world: &Vec3,
        types: Type,
        sample: &Sample,
        light_wave_index: usize,
    ) -> Option<BxDFSample<f32>> {
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * *outgoing_world;

        let bxdf = self.random_matching_bxdf(types, sample.one_d)?;

        if let Some(mut sample) = bxdf.sample_light_wave(&outgoing, &sample.two_d, light_wave_index)
        {
            sample.incident = rotation.reversed() * sample.incident;

            Some(sample)
        } else {
            None
        }
    }

    pub fn pdf(
        &self,
        normal: &Vec3,
        incident_world: &Vec3,
        outgoing_world: &Vec3,
        types: Type,
    ) -> f32 {
        let rotation = world_to_bxdf(normal);
        let incident = rotation * *incident_world;
        let outgoing = rotation * *outgoing_world;

        let (pdf, num) = self
            .bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(types) {
                    Some(bxdf.pdf(&incident, &outgoing))
                } else {
                    None
                }
            })
            .fold((0.0, 0usize), |(a, num), b| (a + b, num + 1));

        if num > 0 {
            pdf / num as f32
        } else {
            0.0
        }
    }
}
