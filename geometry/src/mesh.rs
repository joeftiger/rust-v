use std::sync::Arc;

use tobj::Mesh as TobjMesh;
use ultraviolet::{Rotor3, Vec3};

use crate::bvh::BottomUpBVH;
use crate::ray::Ray;
use crate::{Boundable, Cube, Intersectable, Intersection};
use std::ops::Deref;
use utility::floats::EPSILON;

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    a: Arc<Vec3>,
    b: Arc<Vec3>,
    c: Arc<Vec3>,
}

impl Triangle {
    pub fn new(a: Arc<Vec3>, b: Arc<Vec3>, c: Arc<Vec3>) -> Self {
        Self { a, b, c }
    }
}

impl Boundable for Triangle {
    fn bounds(&self) -> Cube {
        let min = self.a.min_by_component(self.b.min_by_component(*self.c));
        let max = self.a.max_by_component(self.b.max_by_component(*self.c));

        Cube::new(min, max)
    }
}

impl Boundable for Arc<Triangle> {
    fn bounds(&self) -> Cube {
        self.deref().bounds()
    }
}

impl Intersectable for Triangle {
    #[allow(clippy::many_single_char_names)]
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let ab = *self.b - *self.a;
        let ac = *self.c - *self.a;
        let h = ray.direction.cross(ac);

        let det = ab.dot(h);
        if det < EPSILON {
            return None;
        }

        let t = ray.origin - *self.a;
        let u = t.dot(h) / det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = t.cross(ab);
        let v = ray.direction.dot(q) / det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = ac.dot(q) / det;
        if t < ray.t_start || ray.t_end < t {
            return None;
        }

        let point = ray.at(t);

        let ab = *self.b - *self.a;
        let ac = *self.c - *self.a;
        let normal = ac.cross(ab).normalized();

        Some(Intersection::new(point, normal, t, *ray))
    }

    #[allow(clippy::many_single_char_names)]
    fn intersects(&self, ray: &Ray) -> bool {
        let ab = *self.b - *self.a;
        let ac = *self.c - *self.a;
        let h = ray.direction.cross(ac);

        let det = ab.dot(h);
        if det < EPSILON {
            return false;
        }

        let t = ray.origin - *self.a;
        let u = t.dot(h) / det;
        if !(0.0..=1.0).contains(&u) {
            return false;
        }

        let q = t.cross(ab);
        let v = ray.direction.dot(q) / det;
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = ac.dot(q) / det;

        ray.contains(t)
    }
}

impl Intersectable for Arc<Triangle> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.deref().intersect(ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.deref().intersects(ray)
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Arc<Vec3>>,
    bvh: Arc<BottomUpBVH<Arc<Triangle>>>,
}

impl Mesh {
    #[allow(dead_code)]
    pub fn new(vertices: Vec<Arc<Vec3>>, triangles: Vec<Arc<Triangle>>) -> Self {
        let bvh = BottomUpBVH::create_from_vec(triangles);

        Self { vertices, bvh }
    }

    #[allow(dead_code)]
    pub fn load_scale_floor_rot(
        (tobj_mesh, scale, center_floor, rotation): (&TobjMesh, Vec3, Vec3, Rotor3),
    ) -> Self {
        let mut vertices = Vec::with_capacity(tobj_mesh.positions.len());
        let mut v_center = Vec3::zero();
        let mut minimum_y = f32::INFINITY;

        let mut i = 0;
        while i < tobj_mesh.positions.len() {
            let vertex = Vec3::new(
                tobj_mesh.positions[i] * scale.x,
                tobj_mesh.positions[i + 1] * scale.y,
                tobj_mesh.positions[i + 2] * scale.z,
            );
            vertices.push(vertex);
            v_center += vertex;
            minimum_y = minimum_y.min(vertex.y);
            i += 3;
        }
        let vertices: Vec<Arc<Vec3>> = vertices
            .iter()
            .map(|v| {
                let mut v = *v + center_floor;
                v.y -= minimum_y;

                let mut v_tmp = v - center_floor;
                v_tmp = rotation * v_tmp;
                v = v_tmp + center_floor;

                Arc::new(v)
            })
            .collect();

        let mut triangles = Vec::with_capacity(tobj_mesh.indices.len() / 3);
        let mut i = 0;
        while i < tobj_mesh.indices.len() {
            let a = vertices[tobj_mesh.indices[i] as usize].clone();
            let b = vertices[tobj_mesh.indices[i + 1] as usize].clone();
            let c = vertices[tobj_mesh.indices[i + 2] as usize].clone();

            let triangle = Triangle::new(a, b, c);
            triangles.push(Arc::new(triangle));
            i += 3;
        }
        triangles.shrink_to_fit();

        Mesh::new(vertices, triangles)
    }
}

impl Boundable for Mesh {
    fn bounds(&self) -> Cube {
        self.bvh.bounds()
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.bvh.intersect(ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.bvh.intersects(ray)
    }
}