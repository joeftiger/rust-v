use crate::floats::FloatExt;
use crate::*;
use std::f64::consts::PI;

/// Solves a quadratic equation, handling generics.
///
/// `a`x^2 + `b`x + `c`
///
/// # Constraints
/// * `a` - Should be finite (neither infinite nor `NaN`).
/// * `b` - Should be finite.
/// * `c` - Should be finite.
///
/// # Arguments
/// * `a` - The parameter for `x^2`
/// * `b` - The parameter for `x`
/// * `c` - The constant parameter
///
/// # Returns
/// * `Option<(f32, f32)>` - The solutions in ascending order (if any)
#[inline]
#[must_use]
pub fn solve_quadratic(a: Float, b: Float, c: Float) -> Option<(Float, Float)> {
    debug_assert!(a.is_finite());
    debug_assert!(b.is_finite());
    debug_assert!(c.is_finite());

    if a < Float::epsilon() {
        if b < Float::epsilon() {
            return None;
        }

        let sol = -c / b;

        return Some((sol, sol));
    }

    let discriminant = b.mul_add(b, -4.0 * a * c);
    if discriminant < 0.0 {
        return None;
    }

    let a_x1 = -0.5 * (b + Float::copysign(discriminant.sqrt(), b));

    let x0 = a_x1 / a;
    let x1 = c / a_x1;

    if x0 < x1 {
        Some((x0, x1))
    } else {
        Some((x1, x0))
    }
}

/// Computes the `sinc()` function.
///
/// # Constraints
/// * `x` - Should be finite (neither infinite nor `NaN`).
///
/// # Arguments
/// * `x` - The value to apply sinc() to
///
/// # Returns
/// * The sinc of `x`
#[inline(always)]
pub fn sinc(mut x: Float) -> Float {
    debug_assert!(x.is_finite());

    x = x.abs();

    if x < Float::epsilon() {
        1.0
    } else {
        let pix = PI as Float * x;
        Float::sin(pix) / pix
    }
}
