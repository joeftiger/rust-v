use crate::bxdf::{same_hemisphere, world_to_bxdf, BxDF, BxDFSample, Type};
use crate::debug_utils::is_normalized;
use crate::samplers::Sample;
use crate::*;
use serde::{Deserialize, Serialize};

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

    pub fn random_matching_bxdf(&self, t: Type, rand: Float) -> Option<&dyn BxDF> {
        let count = self.num_types(t);
        if count == 0 {
            return None;
        }

        let index = (rand * count as Float) as usize;
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
        normal: Vector3,
        incident_world: Vector3,
        outgoing_world: Vector3,
        mut types: Type,
    ) -> Spectrum {
        let rotation = world_to_bxdf(normal);

        let incident = rotation * incident_world;
        let outgoing = rotation * outgoing_world;

        // transmission or reflection
        if same_hemisphere(incident, outgoing) {
            types &= !Type::TRANSMISSION;
        } else {
            types &= !Type::REFLECTION;
        }

        self.bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(types) {
                    Some(bxdf.evaluate(incident, outgoing))
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn evaluate_light_wave(
        &self,
        normal: Vector3,
        incident_world: Vector3,
        outgoing_world: Vector3,
        mut types: Type,
        light_wave_index: usize,
    ) -> Float {
        let rotation = world_to_bxdf(normal);
        let incident = rotation * incident_world;
        let outgoing = rotation * outgoing_world;

        // transmission or reflection
        if same_hemisphere(incident, outgoing) {
            types &= !Type::TRANSMISSION;
        } else {
            types &= !Type::REFLECTION;
        }

        self.bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(types) {
                    Some(bxdf.evaluate_light_wave(incident, outgoing, light_wave_index))
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn evaluate_bxdf_light_wave(
        bxdf: &dyn BxDF,
        normal: Vector3,
        incident_world: Vector3,
        outgoing_world: Vector3,
        light_wave_index: usize,
    ) -> Float {
        let rotation = world_to_bxdf(normal);
        let incident = rotation * incident_world;
        let outgoing = rotation * outgoing_world;

        bxdf.evaluate_light_wave(incident, outgoing, light_wave_index)
    }

    pub fn sample(
        &self,
        normal: Vector3,
        outgoing_world: Vector3,
        types: Type,
        sample: Sample,
    ) -> Option<BxDFSample<Spectrum>> {
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * outgoing_world;

        let bxdf = self.random_matching_bxdf(types, sample.one_d)?;

        bxdf.sample(outgoing, sample.two_d).map(|mut s| {
            s.incident = rotation.reversed() * s.incident;
            s
        })
    }

    pub fn sample_bxdf(
        bxdf: &dyn BxDF,
        normal: Vector3,
        outgoing_world: Vector3,
        sample: Sample,
    ) -> Option<BxDFSample<Spectrum>> {
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * outgoing_world;

        bxdf.sample(outgoing, sample.two_d).map(|mut s| {
            s.incident = rotation.reversed() * s.incident;
            s
        })
    }

    pub fn sample_light_wave(
        &self,
        normal: Vector3,
        outgoing_world: Vector3,
        types: Type,
        sample: Sample,
        light_wave_index: usize,
    ) -> Option<BxDFSample<Float>> {
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * outgoing_world;

        let bxdf = self.random_matching_bxdf(types, sample.one_d)?;

        bxdf.sample_light_wave(outgoing, sample.two_d, light_wave_index)
            .map(|mut s| {
                s.incident = rotation.reversed() * s.incident;
                s
            })
    }

    pub fn sample_bxdf_light_wave(
        bxdf: &dyn BxDF,
        normal: Vector3,
        outgoing_world: Vector3,
        sample: Sample,
        light_wave_index: usize,
    ) -> Option<BxDFSample<Float>> {
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * outgoing_world;

        bxdf.sample_light_wave(outgoing, sample.two_d, light_wave_index)
            .map(|mut s| {
                s.incident = rotation.reversed() * s.incident;
                s
            })
    }

    pub fn sample_bxdf_incident(
        bxdf: &dyn BxDF,
        normal: Vector3,
        outgoing_world: Vector3,
        sample: Vector2,
    ) -> Option<Vector3> {
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * outgoing_world;

        bxdf.sample_incident(outgoing, sample)
            .map(|incident| rotation.reversed() * incident)
    }

    pub fn sample_bxdf_incident_light_wave(
        bxdf: &dyn BxDF,
        normal: Vector3,
        outgoing_world: Vector3,
        sample: Vector2,
        light_wave_index: usize,
    ) -> Option<Vector3> {
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * outgoing_world;

        bxdf.sample_incident_light_wave(outgoing, sample, light_wave_index)
            .map(|incident| rotation.reversed() * incident)
    }

    pub fn sample_bxdf_incident_pdf(
        bxdf: &dyn BxDF,
        normal: Vector3,
        outgoing_world: Vector3,
        sample: Vector2,
    ) -> Option<(Vector3, Float)> {
        debug_assert!(!bxdf.is_type(Type::SPECULAR));
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * outgoing_world;

        bxdf.sample_incident_pdf(outgoing, sample)
            .map(|(incident, pdf)| (rotation.reversed() * incident, pdf))
    }

    pub fn sample_bxdf_incident_pdf_light_wave(
        bxdf: &dyn BxDF,
        normal: Vector3,
        outgoing_world: Vector3,
        sample: Vector2,
        light_wave_index: usize,
    ) -> Option<(Vector3, Float)> {
        debug_assert!(!bxdf.is_type(Type::SPECULAR));
        debug_assert!(is_normalized(normal));
        debug_assert!(is_normalized(outgoing_world));

        let rotation = world_to_bxdf(normal);
        let outgoing = rotation * outgoing_world;

        bxdf.sample_incident_pdf_light_wave(outgoing, sample, light_wave_index)
            .map(|(incident, pdf)| (rotation.reversed() * incident, pdf))
    }

    pub fn pdf(
        &self,
        normal: Vector3,
        incident_world: Vector3,
        outgoing_world: Vector3,
        types: Type,
    ) -> Float {
        let rotation = world_to_bxdf(normal);
        let incident = rotation * incident_world;
        let outgoing = rotation * outgoing_world;

        let (pdf, num) = self
            .bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.is_type(types) {
                    Some(bxdf.pdf(incident, outgoing))
                } else {
                    None
                }
            })
            .fold((0.0, 0usize), |(a, num), b| (a + b, num + 1));

        if num > 0 {
            pdf / num as Float
        } else {
            0.0
        }
    }
}
