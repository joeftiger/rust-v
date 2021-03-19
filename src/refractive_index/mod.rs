#![allow(clippy::excessive_precision)]

use serde::{Deserialize, Serialize};
///! In optics, the **refractive index** of a material is a dimensionless number that describes
///! how fast light travels through the material.
///!
///! This trait helps describe the different spectra of refractive indices, as different wavelengths
///! refract differently.
///!
///! To complement the refractive index, this trait also specifies to return an **optional extinction
///! coefficient**. The extinction coefficient describes how strongly a material absorbs light at given
///! wavelength.
use utility::floats::fast_cmp;
use utility::math::lerp_map;

pub mod air;
pub mod glass;
pub mod sapphire;
pub mod water;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum RefractiveType {
    Air,
    Vacuum,
    Water,
    Glass,
    Sapphire,
}

impl RefractiveType {
    /// Returns the refractive index (inaccurate for different wavelengths).
    ///
    /// # Returns
    /// * The refractive index
    #[inline(always)]
    pub fn n_uniform(&self) -> f32 {
        match self {
            RefractiveType::Air => 1.00029,
            RefractiveType::Vacuum => 1.0,
            RefractiveType::Water => 1.3325,
            RefractiveType::Glass => 1.5168,
            RefractiveType::Sapphire => 1.7490,
        }
    }

    /// Returns the extinction coefficient (if it exists).
    ///
    /// # Returns
    /// * `Some` extinction coefficient, or
    /// * `None`
    #[inline(always)]
    pub fn k_uniform(&self) -> Option<f32> {
        match self {
            RefractiveType::Air => None,
            RefractiveType::Vacuum => None,
            RefractiveType::Water => Some(7.2792e-9),
            RefractiveType::Glass => Some(9.7525e-9),
            RefractiveType::Sapphire => Some(0.020900),
        }
    }

    /// Returns the refractive index at a given wavelength.
    ///
    /// # Arguments
    /// * `lambda` - The wavelength in **µm**
    ///
    /// # Returns
    /// * The corresponding refractive index
    pub fn n(&self, lambda: f32) -> f32 {
        match self {
            // RefractiveType::AIR => search_and_get(&air::INDEX, &air::N, lambda),
            RefractiveType::Air => air::sellmeier_n(lambda),
            RefractiveType::Vacuum => 1.0,
            RefractiveType::Water => search_and_lerp(&water::INDEX, &water::N, lambda),
            RefractiveType::Glass => glass::sellmeier_n(lambda),
            RefractiveType::Sapphire => sapphire::sellmeier_n(lambda),
        }
    }

    /// Returns the extinction coefficient at a given wavelength (if it exists).
    ///
    /// # Arguments
    /// * `lambda` - The wavelength in **µm**
    ///
    /// # Returns
    /// * `Some` corresponding extinction coefficient, or
    /// * `None`
    pub fn k(&self, lambda: f32) -> Option<f32> {
        match self {
            RefractiveType::Air => None,
            RefractiveType::Vacuum => None,
            RefractiveType::Water => Some(search_and_lerp(&water::INDEX, &water::K, lambda)),
            RefractiveType::Glass => Some(search_and_lerp(&glass::INDEX_K, &glass::K, lambda)),
            RefractiveType::Sapphire => {
                Some(search_and_lerp(&sapphire::INDEX_K, &sapphire::K, lambda))
            }
        }
    }
}

/// Searches for the index of a given value inside a given slice.
/// If no such value is found, it will return the the indexes below/above the value, allowing to
/// lerp further usages.
#[inline]
pub fn search_index(slice: &[f32], value: f32) -> Result<usize, (usize, usize)> {
    match slice.binary_search_by(|a| fast_cmp(*a, value)) {
        Ok(index) => Ok(index),
        Err(index) => Err((index - 1, index)),
    }
}

#[inline]
pub fn search_and_lerp(index_slice: &[f32], value_slice: &[f32], wavelength_nm: f32) -> f32 {
    match search_index(index_slice, wavelength_nm) {
        Ok(i) => value_slice[i],
        Err((min, max)) => {
            if max >= value_slice.len() {
                return value_slice[min];
            }

            let to_lerp = (value_slice[min], value_slice[max]);

            lerp_map((min as f32, max as f32), to_lerp, wavelength_nm)
        }
    }
}
