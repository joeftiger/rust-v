use std::cmp::Ordering;

pub const EPSILON: f32 = 10.0 * f32::EPSILON;
pub const BIG_EPSILON: f32 = 1e-4;

// WolframAlpha
#[allow(clippy::excessive_precision)]
pub const SQRT_PI: f32 = 1.7724538509055160272981674833411451827975494561223871282138077898;
#[allow(clippy::excessive_precision)]
pub const FRAC_1_SQRT_PI: f32 = 0.5641895835477562869480794515607725858440506293289988568440857217;
#[allow(clippy::excessive_precision)]
pub const PI_2: f32 = 6.2831853071795864769252867665590057683943387987502116419498891846;

/// Returns whether the given value is in range of `(min, max)`.
#[inline(always)]
pub fn in_range(f: f32, min: f32, max: f32) -> bool {
    min < f && f < max
}

/// Returns whether the given value is in range of `[min, max]`.
#[inline(always)]
#[must_use]
pub fn in_range_incl(f: f32, min: f32, max: f32) -> bool {
    min <= f && f <= max
}

/// Returns whether the given value is in range of `[min, max)`.
#[inline(always)]
#[must_use]
pub fn in_range_incl_left(f: f32, min: f32, max: f32) -> bool {
    min <= f && f < max
}

/// Returns whether the given value is in range of `(min, max]`.
#[inline(always)]
#[must_use]
pub fn in_range_incl_right(f: f32, min: f32, max: f32) -> bool {
    min < f && f <= max
}

#[must_use]
pub fn is_finite(ar: &[f32]) -> bool {
    ar.iter().all(|f| f.is_finite())
}

#[must_use]
pub fn is_not_nan(ar: &[f32]) -> bool {
    ar.iter().all(|f| !f.is_nan())
}

#[inline(always)]
#[must_use]
pub fn fast_min(a: f32, b: f32) -> f32 {
    if b < a {
        b
    } else {
        a
    }
}

#[inline(always)]
#[must_use]
pub fn fast_max(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

#[inline(always)]
#[must_use]
pub fn approx_zero_tolerance(value: f32, zero_tolerance: f32) -> bool {
    value.abs() <= zero_tolerance
}

#[inline(always)]
#[must_use]
pub fn approx_zero(value: f32) -> bool {
    approx_zero_tolerance(value, EPSILON)
}

#[must_use]
pub fn approx_zero_ar(ar: &[f32]) -> bool {
    ar.iter().all(|a| approx_zero(*a))
}

#[inline(always)]
#[must_use]
pub fn approx_equal_tolerance(a: f32, b: f32, zero_tolerance: f32) -> bool {
    let distance = (b - a).abs();
    if distance <= zero_tolerance {
        true
    } else {
        let largest = fast_max(1.0, fast_max(a.abs(), b.abs()));
        distance <= largest * f32::EPSILON
    }
}

#[inline(always)]
#[must_use]
pub fn approx_equal(a: f32, b: f32) -> bool {
    approx_equal_tolerance(a, b, EPSILON)
}

#[must_use]
pub fn approx_equal_ar(ar: &[f32], approx: f32) -> bool {
    ar.iter().all(|a| approx_equal(*a, approx))
}

#[inline(always)]
#[must_use]
pub fn approx_equal_big(a: f32, b: f32) -> bool {
    approx_equal_tolerance(a, b, BIG_EPSILON)
}

#[inline(always)]
#[must_use]
pub fn lt_epsilon_tolerance(a: f32, zero_tolerance: f32) -> bool {
    a < zero_tolerance
}

#[inline(always)]
#[must_use]
pub fn lt_epsilon(a: f32) -> bool {
    lt_epsilon_tolerance(a, EPSILON)
}

#[inline(always)]
#[must_use]
pub fn fast_clamp(f: f32, min: f32, max: f32) -> f32 {
    fast_min(max, fast_max(min, f))
}

pub fn fast_clamp_ar(ar: &mut [f32], min: f32, max: f32) {
    ar.iter_mut()
        .for_each(|value| *value = fast_clamp(*value, min, max));
}

#[inline(always)]
#[must_use]
pub fn fast_cmp(a: f32, b: f32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}