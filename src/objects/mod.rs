use geometry::{Aabb, Boundable, Geometry, Intersectable, Intersection, Ray};

mod emitter;
mod point;
mod receiver;
mod sphere;

use crate::bxdf::BSDF;
pub use emitter::{Emitter, EmitterExt};
pub use receiver::{Receiver, ReceiverExt};

#[derive(Clone, Copy)]
pub enum SceneObject<'a> {
    Emitter(&'a dyn EmitterExt),
    Receiver(&'a dyn ReceiverExt),
}

impl ReceiverExt for SceneObject<'_> {
    fn geometry(&self) -> &dyn Geometry {
        match *self {
            SceneObject::Emitter(e) => e.geometry(),
            SceneObject::Receiver(r) => r.geometry(),
        }
    }

    fn bsdf(&self) -> &BSDF<'_> {
        match *self {
            SceneObject::Emitter(e) => e.bsdf(),
            SceneObject::Receiver(r) => r.bsdf(),
        }
    }
}

impl Boundable for SceneObject<'_> {
    fn bounds(&self) -> Aabb {
        match *self {
            SceneObject::Emitter(e) => e.bounds(),
            SceneObject::Receiver(r) => r.bounds(),
        }
    }
}

impl Intersectable for SceneObject<'_> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match *self {
            SceneObject::Emitter(e) => e.intersect(ray),
            SceneObject::Receiver(r) => r.intersect(ray),
        }
    }

    fn intersects(&self, ray: &Ray) -> bool {
        match *self {
            SceneObject::Emitter(e) => e.intersects(ray),
            SceneObject::Receiver(r) => r.intersects(ray),
        }
    }
}