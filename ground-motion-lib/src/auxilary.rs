//! Auxiliary constants and helper functions.
//!
//! This module provides numerical constants and utility functions for
//! common operations used in ground motion prediction calculations.

/// Standard acceleration due to gravity on Earth's surface, in m/s².
pub const G_GLOBAL: f64 = 9.81;

/// Default depth (in meters) to the subsurface layer where the shear-wave
/// velocity (Vs) reaches 1400 m/s at a site.
///
/// This value is used when no site-specific `dl` value is provided,
/// but is required by a GMPE implementation.
pub const DL: u64 = 250;

/// Check if two floating-point numbers are approximately equal within a given tolerance (epsilon).
///
/// # Arguments
///
/// * `a` - First floating-point value.
/// * `b` - Second floating-point value.
/// * `epsilon` - Maximum allowed difference between `a` and `b` for them to be considered equal.
///
/// # Returns
///
/// `true` if the absolute difference between `a` and `b` is less than `epsilon`, `false` otherwise.
///
/// # Examples
///
/// ```
/// use ground_motion_lib::auxilary::approx_equal;
/// assert!(approx_equal(1.00001, 1.00002, 0.0001));
/// ```
pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Round a floating-point number to a specified number of decimal places.
///
/// # Arguments
///
/// * `val` - The floating-point value to round.
/// * `places` - Number of decimal places to round to.
///
/// # Returns
///
/// A new `f64` rounded to the specified precision.
///
/// # Examples
///
/// ```
/// use ground_motion_lib::auxilary::round_to_places;
/// assert_eq!(round_to_places(3.14159, 2), 3.14);
/// ```
pub fn round_to_places(val: f64, places: u32) -> f64 {
    let factor = 10_f64.powi(places as i32);
    (val * factor).round() / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_equal_true() {
        assert!(approx_equal(1.000001, 1.000002, 0.00001));
    }

    #[test]
    fn test_approx_equal_false() {
        assert!(!approx_equal(1.0, 1.1, 0.00001));
    }

    #[test]
    fn test_approx_equal_exact() {
        assert!(approx_equal(3.14, 3.14, 0.00001));
    }

    #[test]
    fn test_approx_equal_with_small_epsilon() {
        assert!(!approx_equal(3.14, 3.1400001, 1e-12));
        assert!(approx_equal(3.14, 3.14, 1e-12));
    }

    #[test]
    fn test_round_to_places() {
        assert_eq!(round_to_places(3.14159, 2), 3.14);
        assert_eq!(round_to_places(3.145, 2), 3.15);
        assert_eq!(round_to_places(-3.145, 2), -3.15);
        assert_eq!(round_to_places(3.0, 2), 3.0);
    }

    #[test]
    fn test_round_zero_places() {
        assert_eq!(round_to_places(3.7, 0), 4.0);
        assert_eq!(round_to_places(3.3, 0), 3.0);
    }

    #[test]
    fn test_round_to_more_places() {
        assert_eq!(round_to_places(3.14159, 4), 3.1416);
    }
}
