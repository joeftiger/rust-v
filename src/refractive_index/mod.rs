#![allow(clippy::excessive_precision)]

use crate::Float;
use color::color_data::{LAMBDA_END, LAMBDA_START};
use serde::{Deserialize, Serialize};
use utility::floats::FloatExt;

///! In optics, the **refractive index** of a material is a dimensionless number that describes
///! how fast light travels through the material.
///!
///! This trait helps describe the different spectra of refractive indices, as different wavelengths
///! refract differently.
///!
///! To complement the refractive index, this trait also specifies to return an **optional extinction
///! coefficient**. The extinction coefficient describes how strongly a material absorbs light at given
///! wavelength.
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
    Linear(Float, Float),
}

impl RefractiveType {
    /// Returns the refractive index (inaccurate for different wavelengths).
    ///
    /// # Returns
    /// * The refractive index
    #[inline]
    pub fn n_uniform(&self) -> Float {
        match self {
            RefractiveType::Air => 1.00029,
            RefractiveType::Vacuum => 1.0,
            RefractiveType::Water => 1.3325,
            RefractiveType::Glass => 1.5168,
            RefractiveType::Sapphire => 1.7490,
            &RefractiveType::Linear(min, max) => 0.5 * (min + max),
        }
    }

    /// Returns the extinction coefficient (if it exists).
    ///
    /// # Returns
    /// * `Some` extinction coefficient, or
    /// * `None`
    #[inline]
    pub fn k_uniform(&self) -> Option<Float> {
        match self {
            RefractiveType::Water => Some(7.2792e-9),
            RefractiveType::Glass => Some(9.7525e-9),
            RefractiveType::Sapphire => Some(0.020900),
            _ => None,
        }
    }

    /// Returns the refractive index at a given wavelength.
    ///
    /// # Arguments
    /// * `lambda` - The wavelength in **µm**
    ///
    /// # Returns
    /// * The corresponding refractive index
    #[inline]
    pub fn n(&self, lambda: Float) -> Float {
        match self {
            RefractiveType::Air => air::sellmeier_n(lambda),
            RefractiveType::Vacuum => 1.0,
            RefractiveType::Water => search_and_lerp(&water::INDEX, &water::N, lambda),
            RefractiveType::Glass => glass::sellmeier_n(lambda),
            RefractiveType::Sapphire => sapphire::sellmeier_n(lambda),
            RefractiveType::Linear(min, max) => {
                Float::lerp_map(LAMBDA_START, LAMBDA_END, *max, *min, lambda)
            }
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
    pub fn k(&self, lambda: Float) -> Option<Float> {
        match self {
            RefractiveType::Water => Some(search_and_lerp(&water::INDEX, &water::K, lambda)),
            RefractiveType::Glass => Some(search_and_lerp(&glass::INDEX_K, &glass::K, lambda)),
            RefractiveType::Sapphire => {
                Some(search_and_lerp(&sapphire::INDEX_K, &sapphire::K, lambda))
            }
            _ => None,
        }
    }
}

/// Searches for the index of a given value inside a given slice.
/// If no such value is found, it will return the the indexes below/above the value, allowing to
/// lerp further usages.
#[inline]
pub fn search_index(slice: &[Float], value: Float) -> Result<usize, (usize, usize)> {
    match slice.binary_search_by(|a| a.fast_cmp(value)) {
        Ok(index) => Ok(index),
        Err(index) => Err((index - 1, index)),
    }
}

#[inline]
pub fn search_and_lerp(
    index_slice: &[Float],
    value_slice: &[Float],
    wavelength_nm: Float,
) -> Float {
    match search_index(index_slice, wavelength_nm) {
        Ok(i) => value_slice[i],
        Err((min, max)) => {
            if max >= value_slice.len() {
                return value_slice[min];
            }

            Float::lerp_map(
                min as Float,
                max as Float,
                value_slice[min],
                value_slice[max],
                wavelength_nm,
            )
        }
    }
}
