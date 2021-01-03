use crate::floats::BIG_EPSILON;

/// # Summary
/// Solves a quadratic equation, handling generics.
///
/// # Arguments
/// `a`x^2 + `b`x + `c`
///
/// # Returns
/// * `Option<(f32, f32)>` - The solutions in ascending order (if any)
#[inline(always)]
#[must_use]
pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    debug_assert!(!a.is_nan());
    debug_assert!(!b.is_nan());
    debug_assert!(!c.is_nan());

    if a < BIG_EPSILON {
        if b < BIG_EPSILON {
            return None;
        }

        let sol = -c / b;

        return Some((sol, sol));
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let a_x1 = -0.5 * (b + f32::copysign(discriminant.sqrt(), b));

    let x0 = a_x1 / a;
    let x1 = c / a_x1;

    if x0 < x1 {
        Some((x0, x1))
    } else {
        Some((x1, x0))
    }
}