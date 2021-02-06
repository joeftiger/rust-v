use crate::bvh::candidate::Candidates;
use crate::bvh::item::Item;
use crate::bvh::plane::Plane;
use crate::bvh::side::Side;
use crate::{Aabb, Container, Intersectable, Ray};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::Arc;

const K_T: f32 = 15.;
const K_I: f32 = 20.;

#[derive(Clone)]
pub struct InternalNode<T>
where
    T: Clone,
{
    left_space: Aabb,
    left_node: Node<T>,
    right_space: Aabb,
    right_node: Node<T>,
}

impl<T> InternalNode<T>
where
    T: Clone,
{
    fn new(left_space: Aabb, left_node: Node<T>, right_space: Aabb, right_node: Node<T>) -> Self {
        Self {
            left_space,
            left_node,
            right_space,
            right_node,
        }
    }
}

#[derive(Clone)]
pub enum Node<T>
where
    T: Clone,
{
    Leaf { items: HashSet<Arc<Item<T>>> },
    Node { node: Box<InternalNode<T>> },
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(
        space: Aabb,
        mut candidates: Candidates<T>,
        n: usize,
        sides: &mut Vec<Side>,
    ) -> Self {
        let (cost, best_index, n_l, n_r) = Self::partition(n, &space, &candidates);

        // Check that the cost of the splitting is not higher than the cost of the leaf.
        if cost > K_I * n as f32 {
            // Create the set of primitives
            let mut items = HashSet::with_capacity(n);
            candidates
                .drain(..)
                .filter(|c| c.is_left && c.dimension() == 0)
                .for_each(|c| {
                    items.insert(c.item);
                });
            // let items = HashSet::from_iter(candidates.iter().filter_map(|c| if c.is_left && c.dimension() == 0 {
            //     Some(c.item.clone())
            // } else {
            //     None
            // }));

            return Self::Leaf { items };
        }

        let (left_space, right_space) = Self::split_space(&space, &candidates[best_index].plane);
        let (left_candidates, right_candidates) = Self::classify(candidates, best_index, sides);

        let inner_node = InternalNode::new(
            left_space,
            Self::new(left_space, left_candidates, n_l, sides),
            right_space,
            Self::new(right_space, right_candidates, n_r, sides),
        );

        Self::Node {
            node: Box::new(inner_node),
        }
    }

    fn partition(n: usize, space: &Aabb, candidates: &Candidates<T>) -> (f32, usize, usize, usize) {
        let mut best_cost = f32::INFINITY;
        let mut best_candidate_index = 0;

        // Variables to keep count the number of items in both subspace for each dimension
        let mut n_l = [0; 3];
        let mut n_r = [n; 3];

        // Keep n_l and n_r for the best splitting candidate
        let mut best_n_l = 0;
        let mut best_n_r = n;

        // Find best candidate
        for (i, candidate) in candidates.iter().enumerate() {
            let dim = candidate.dimension();

            // If the right candidate removes it from the right subspace
            if !candidate.is_left {
                n_r[dim] -= 1;
            }

            // Compute the cost of the split and update the best split
            let cost = Self::cost(&candidate.plane, space, n_l[dim], n_r[dim]);
            if cost < best_cost {
                best_cost = cost;
                best_candidate_index = i;
                best_n_l = n_l[dim];
                best_n_r = n_r[dim];
            }

            // If the left candidate add it from the left subspace
            if candidate.is_left {
                n_l[dim] += 1;
            }
        }
        (best_cost, best_candidate_index, best_n_l, best_n_r)
    }

    fn cost(p: &Plane, v: &Aabb, n_l: usize, n_r: usize) -> f32 {
        let (left, right) = Self::split_space(v, p);

        let volume_left = left.volume();
        if volume_left == 0.0 {
            return f32::INFINITY;
        }

        let volume_right = right.volume();
        if volume_right == 0.0 {
            return f32::INFINITY;
        }

        let total_volume = volume_left + volume_right;

        // Decrease cost if it cuts empty space
        let factor = if n_l == 0 || n_r == 0 { 0.8 } else { 1.0 };

        factor
            * (K_T
                + K_I
                    * (n_l as f32 * volume_left / total_volume
                        + n_r as f32 * volume_right / total_volume))
    }

    fn split_space(space: &Aabb, plane: &Plane) -> (Aabb, Aabb) {
        let mut left = *space;
        let mut right = *space;
        match plane {
            Plane::X(x) => {
                left.max.x = x.max(space.min.x).min(space.max.x);
                right.min.x = x.max(space.min.x).min(space.max.x);
            }
            Plane::Y(y) => {
                left.max.y = y.max(space.min.y).min(space.max.y);
                right.min.y = y.max(space.min.y).min(space.max.y);
            }
            Plane::Z(z) => {
                left.max.z = z.max(space.min.z).min(space.max.z);
                right.min.z = z.max(space.min.z).min(space.max.z);
            }
        }
        (left, right)
    }

    fn classify(
        candidates: Candidates<T>,
        best_index: usize,
        sides: &mut Vec<Side>,
    ) -> (Candidates<T>, Candidates<T>) {
        // Step 1: Udate sides to classify items
        Self::classify_items(&candidates, best_index, sides);

        // Step 2: Splicing candidates left and right subspace
        Self::splicing_candidates(candidates, &sides)
    }

    /// Step 1 of classify.
    /// Given a candidate list and a splitting candidate identify wich items are part of the
    /// left, right and both subspaces.
    fn classify_items(candidates: &Candidates<T>, best_index: usize, sides: &mut Vec<Side>) {
        let best_dimension = candidates[best_index].dimension();
        for i in 0..(best_index + 1) {
            if candidates[i].dimension() == best_dimension {
                if !candidates[i].is_left {
                    sides[candidates[i].item.id as usize] = Side::Left;
                } else {
                    sides[candidates[i].item.id as usize] = Side::Both;
                }
            }
        }
        for i in best_index..candidates.len() {
            if candidates[i].dimension() == best_dimension && candidates[i].is_left {
                sides[candidates[i].item.id as usize] = Side::Right;
            }
        }
    }

    // Step 2: Splicing candidates left and right subspace given items sides
    fn splicing_candidates(
        mut candidates: Candidates<T>,
        sides: &Vec<Side>,
    ) -> (Candidates<T>, Candidates<T>) {
        let mut left_candidates = Candidates::with_capacity(candidates.len() / 2);
        let mut right_candidates = Candidates::with_capacity(candidates.len() / 2);

        for e in candidates.drain(..) {
            match sides[e.item.id as usize] {
                Side::Left => left_candidates.push(e),
                Side::Right => right_candidates.push(e),
                Side::Both => {
                    right_candidates.push(e.clone());
                    left_candidates.push(e);
                }
            }
        }
        (left_candidates, right_candidates)
    }

    pub fn intersect(&self, ray: &Ray, intersect_items: &mut HashSet<Arc<Item<T>>>) {
        match self {
            Node::Leaf { items } => intersect_items.extend(items.clone()),
            Node::Node { node } => {
                if node.left_space.contains(&ray.origin) || node.left_space.intersects(ray) {
                    node.left_node.intersect(ray, intersect_items);
                }
                if node.right_space.contains(&ray.origin) || node.right_space.intersects(ray) {
                    node.right_node.intersect(ray, intersect_items);
                }
            }
        }
    }
}