#![allow(dead_code)]
#![allow(unused_variables)]

use crate::bxdf::{
    FresnelType, LambertianReflection, SpecularReflection, SpecularTransmission, BSDF,
};
use crate::camera::{CameraType, PerspectiveCamera};
use crate::demo_scenes::{DemoScene, FOVY};
use crate::objects::{Emitter, Receiver, SceneObject};
use crate::refractive_index::RefractiveType;
use crate::sampler::pixel_samplers::PixelSamplerType;
use crate::scene::Scene;
use crate::Spectrum;
use color::{Color, Colors};
use geometry::{Aabb, Point, Sphere};
use std::sync::{Arc, Mutex};
use ultraviolet::{UVec2, Vec3};

const FLOOR: f32 = 0.0;
const SKY_RADIUS: f32 = 500.0;
const RADIUS: f32 = 0.5;

const DISTRIBUTION: f32 = 10.0;
const NUM_SPHERES_IN_DIMENSION: u32 = 5;

pub struct SphereScene;

impl DemoScene for SphereScene {
    fn create(resolution: UVec2) -> Scene {
        fastrand::seed(0);
        let mut scene = create_scene();
        scene.camera = Mutex::new(create_camera(resolution));

        scene
    }
}

fn ground() -> SceneObject {
    let min = Vec3::new(-1000000.0, FLOOR - 5.0, -1000000.0);
    let max = Vec3::new(1000000.0, FLOOR, 1000000.0);
    let cube = Aabb::new(min, max);

    let lambertian = LambertianReflection::new(Spectrum::white());
    let bxdf = Box::new(lambertian);

    let bsdf = BSDF::new(vec![bxdf]);
    let geometry = Box::new(cube);

    let receiver = Arc::new(Receiver::new(geometry, bsdf));

    SceneObject::Receiver(receiver)
}

fn sky() -> SceneObject {
    let center = Vec3::zero();
    let sphere = Sphere::new(center, SKY_RADIUS);

    let lambertian = LambertianReflection::new(Spectrum::blue() + Spectrum::white() * 0.2);
    let bxdf = Box::new(lambertian);

    let bsdf = BSDF::new(vec![bxdf]);
    let geometry = Box::new(sphere);

    let receiver = Arc::new(Receiver::new(geometry, bsdf));
    SceneObject::Receiver(receiver)
}

fn random_pos() -> Vec3 {
    let x = DISTRIBUTION * (fastrand::f32() - 0.5);
    let z = DISTRIBUTION * (fastrand::f32() - 0.5);

    Vec3::new(x, RADIUS, z)
}

fn random_color() -> Spectrum {
    let rand = fastrand::f32() * 1.5;

    if rand < 0.25 {
        Spectrum::red()
    } else if rand < 0.5 {
        Spectrum::green()
    } else if rand < 0.75 {
        Spectrum::blue()
    } else {
        Spectrum::white()
    }
}

fn random_bsdf(color: Spectrum) -> (bool, BSDF) {
    let rand = fastrand::f32();

    let mut out = false;
    let bsdf = if color == Spectrum::white() {
        if rand < 0.6 {
            out = true;
            BSDF::empty()
        } else if rand < 0.8 {
            let specular = SpecularReflection::new(Spectrum::new_const(1.0), FresnelType::NoOp);
            let bxdf = Box::new(specular);

            BSDF::new(vec![bxdf])
        } else {
            let specular = SpecularTransmission::new(
                Spectrum::new_const(1.0),
                RefractiveType::Air,
                RefractiveType::Glass,
            );
            let bxdf = Box::new(specular);

            BSDF::new(vec![bxdf])
        }
    } else {
        let lambertian = LambertianReflection::new(color);
        let bxdf = Box::new(lambertian);

        BSDF::new(vec![bxdf])
    };

    (out, bsdf)
}

fn create_emitter() -> SceneObject {
    let position = Vec3::new(0.0, SKY_RADIUS / 2.0, 0.0);
    let point = Point(position);

    let bsdf = BSDF::empty();
    let mut emission = Spectrum::white() + Spectrum::green() + Spectrum::red();
    emission /= 2.0;
    let geometry = Box::new(point);

    let emitter = Arc::new(Emitter::new(geometry, bsdf, emission));
    SceneObject::Emitter(emitter)
}

fn create_scene() -> Scene {
    let mut scene = Scene::default();

    for _ in 0..NUM_SPHERES_IN_DIMENSION {
        for _ in 0..NUM_SPHERES_IN_DIMENSION {
            let center = random_pos();
            let sphere = Sphere::new(center, RADIUS);
            let geometry = Box::new(sphere);

            let color = random_color();
            let (emitting, bsdf) = random_bsdf(color);

            let obj = if emitting {
                let emitter = Arc::new(Emitter::new(geometry, bsdf, color * 2.0));
                SceneObject::Emitter(emitter)
            } else {
                let receiver = Arc::new(Receiver::new(geometry, bsdf));
                SceneObject::Receiver(receiver)
            };

            scene.add(obj);
        }
    }

    scene.add(ground());
    scene.add(sky());
    scene.add(create_emitter());

    scene
}

//noinspection DuplicatedCode
fn create_camera(resolution: UVec2) -> CameraType {
    let position = Vec3::new(0.0, 5.0, 10.0);
    let target = Vec3::new(0.0, 1.0, 0.0);

    let camera = PerspectiveCamera::new(
        PixelSamplerType::Random,
        position,
        target,
        Vec3::unit_y(),
        FOVY,
        resolution,
    );
    // let camera = crate::camera::perspective_simone::PerspectiveCamera::new(position, target, Vec3::unit_y(), FOVY, resolution);

    CameraType::Perspective(camera)
}
