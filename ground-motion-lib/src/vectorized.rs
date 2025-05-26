//! # Vectorized Ground Motion Calculations
//!
//! This module provides parallelized routines for ground motion prediction calculations
//! and post-processing over collections of site points. It leverages [`Rayon`] for efficient
//! multi-threaded computation on large datasets.
//!
//! ## Features
//!
//! - **Parallel GMPE prediction:** Compute ground motion values for large sets of site points in parallel.
//! - **Summary statistics computation:** Derive key statistical descriptors (mean, standard deviation, min, max, median)
//!   from ground motion prediction results, with parallelism applied to key operations.
//!
//! ## Primary Types and Functions
//!
//! - [`calc_gmpe_vec`]: Perform parallel ground motion prediction for a vector of [`Vs30Point`] instances.
//! - [`compute_stats`]: Calculate summary statistics over a collection of predicted [`GmpePoint`] values.
//! - [`Stats`]: Struct representing the computed statistical summary.
//!
//! ## Parallelism
//!
//! This module uses [`Rayon`](https://docs.rs/rayon/latest/rayon/) for thread-safe, data-parallel operations:
//!
//! - `par_iter()` for distributing GMPE calculations and statistical reductions across threads.
//! - Number of threads is controlled by the `RAYON_NUM_THREADS` environment variable or defaults
//!   to the number of logical CPU cores.
//!
//! ## Usage Example
//!
//! ```rust
//! use ground_motion_lib::gmm::{Vs30Point, Earthquake, Magnitude};
//! use ground_motion_lib::configs::get_mf2013_lib_configs;
//! use ground_motion_lib::vectorized::{calc_gmpe_vec, compute_stats};
//!
//! let points = vec![
//!     Vs30Point::new(142.5, 50.0, 400., Some(200.), Some(0)),
//!     Vs30Point::new(142.6, 50.1, 350., Some(150.), Some(1)),
//! ];
//!
//! let eq = Earthquake {
//!     lon: 142.4,
//!     lat: 50.0,
//!     depth: 10.0,
//!     magnitude: 6.5,
//!     magnitude_kind: Magnitude::Mw,
//! };
//!
//! let gmpe_ref = get_mf2013_lib_configs().get("config_mf2013_crustal_pga").unwrap();
//! let results = calc_gmpe_vec(&points, gmpe_ref, &eq);
//! let stats = compute_stats(&results);
//!
//! println!("Stats: {stats:?}");
//! ```
//!
//! ## See Also
//!
//! - [`crate::gmm::GmpePoint`]
//! - [`crate::gmm::Vs30Point`]
//! - [`crate::gmm::GroundMotionModeling`]
//! - [`crate::mf2013::MF2013`]
//!
//! ## Thread Safety
//!
//! All operations in this module are thread-safe and make use of [`Rayon`] for concurrency.

use crate::gmm::{Earthquake, GmpePoint, GroundMotionModeling, Vs30Point};
use rayon::prelude::*;

/// Calculate ground motion predictions for a set of site points in parallel.
///
/// This function takes a slice of `Vs30Point` site points, a reference to a ground motion prediction
/// equation (GMPE) implementation, and an earthquake definition, and computes ground motion values
/// (`GmpePoint`) for each site point using the provided GMPE model.
///
/// The calculation is performed in parallel using Rayon to improve performance for large datasets.
///
/// # Type Parameters
///
/// * `T` - A type implementing the `GroundMotionModeling` trait.
///   Must also implement `Sync` to allow safe parallel access across threads.
///
/// # Arguments
///
/// * `points` - A slice of `Vs30Point` instances representing the site points for which
///   ground motion predictions will be calculated.
/// * `gmpe` - A reference to a type implementing the `GroundMotionModeling` trait, representing
///   the GMPE model to be used for the calculations.
/// * `eq` - A reference to the `Earthquake` instance describing the earthquake event.
///
/// # Returns
///
/// A `Vec<GmpePoint>` containing the calculated ground motion values for each input site point.
///
/// # Examples
///
/// ```rust
/// use ground_motion_lib::gmm::{Vs30Point, Earthquake, Magnitude, GroundMotionModeling};
/// use ground_motion_lib::mf2013::MF2013;
/// use ground_motion_lib::configs::get_mf2013_lib_configs;
/// use ground_motion_lib::vectorized::calc_gmpe_vec;
///
/// let points = vec![
///     Vs30Point::new(142.5, 50.0, 400., Some(200.), Some(0)),
///     Vs30Point::new(142.6, 50.1, 350., Some(150.), Some(1)),
/// ];
///
/// let eq = Earthquake {
///     lon: 142.4,
///     lat: 50.0,
///     depth: 10.0,
///     magnitude: 6.5,
///     magnitude_kind: Magnitude::Mw,
/// };
///
/// let gmpe_ref = get_mf2013_lib_configs().get("config_mf2013_crustal_pga").unwrap();
///
/// let results = calc_gmpe_vec(&points, gmpe_ref, &eq);
/// println!("{results:?}");
/// ```
///
/// # Parallelism
///
/// This function uses Rayon’s `par_iter()` to distribute the workload of computing ground motion
/// values across multiple threads. The number of threads is typically controlled by the
/// `RAYON_NUM_THREADS` environment variable, or by default equals the number of logical CPU cores.
/// - [`Rayon`](https://docs.rs/rayon/latest/rayon/)
///
/// # See Also
///
/// - [`GmpePoint`](crate::gmm::GmpePoint)
/// - [`Vs30Point`](crate::gmm::Vs30Point)
/// - [`GroundMotionModeling`](crate::gmm::GroundMotionModeling)
///
pub fn calc_gmpe_vec<T: GroundMotionModeling + Sync>(
    points: &[Vs30Point],
    gmpe: &T,
    eq: &Earthquake,
) -> Vec<GmpePoint> {
    points
        .par_iter()
        .map(|point| point.get_gm(gmpe, eq))
        .collect()
}

/// Struct for computed summary statistics
#[derive(Debug, PartialEq)]
pub struct Stats {
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
}

/// Compute summary statistics (mean, standard deviation, minimum, maximum, and median)
/// for a list of `GmpePoint` values.
///
/// This function processes the `value` field from a slice of `GmpePoint` instances,
/// computing key statistical measures.
/// Most operations are parallelized using Rayon where possible (e.g., sum, variance, min, max)
/// to improve performance on larger datasets.
///
/// # Arguments
///
/// * `points` - A slice of `GmpePoint` instances to analyze.
///
/// # Returns
///
/// A `Stats` struct containing:
/// - `mean` — the arithmetic mean
/// - `std_dev` — the sample standard deviation
/// - `min` — the minimum value
/// - `max` — the maximum value
/// - `median` — the median value (sorted centrally)
///
/// # Example
///
/// ```rust
/// use ground_motion_lib::gmm::GmpePoint;
/// use ground_motion_lib::vectorized::{compute_stats, Stats};
///
/// let points = vec![
///     GmpePoint::new_pga(147.1, 50.1, 1.1),
///     GmpePoint::new_pga(147.2, 50.2, 1.2),
///     GmpePoint::new_pga(147.3, 50.3, 1.3),
/// ];
///
/// let stats = compute_stats(&points);
///
/// println!("Mean: {}", stats.mean);
/// println!("Min: {}", stats.min);
/// println!("Max: {}", stats.max);
/// println!("Median: {}", stats.median);
/// println!("Std Dev: {}", stats.std_dev);
/// ```
///
/// # Parallelism
///
/// - Sum, variance, min, and max calculations use `Rayon`’s parallel iterators.
/// - Median is computed single-threaded via an in-place sort since sorting isn’t parallelized here.
///
/// # Panics
///
/// This function will panic if called with an empty slice.
///
/// # See Also
///
/// - [`Rayon`](https://docs.rs/rayon/latest/rayon/)
/// - [`GmpePoint`](crate::gmm::GmpePoint)
/// - [`Stats`](crate::vectorized::Stats)
///
pub fn compute_stats(points: &[GmpePoint]) -> Stats {
    let n = points.len() as f64;

    // Extract values into a Vec<f64> to operate on
    let mut values: Vec<f64> = points.iter().map(|p| p.value).collect();

    // Compute sum in parallel, then mean
    let sum: f64 = values.par_iter().sum();
    let mean = sum / n;

    // Compute variance (sample, denominator is n-1)
    let variance: f64 = values
        .par_iter()
        .map(|v| {
            let diff = v - mean;
            diff * diff
        })
        .sum::<f64>()
        / (n - 1.0);
    let std_dev = variance.sqrt();

    // Compute min and max via parallel reduction
    let min = values
        .par_iter()
        .cloned()
        .reduce(|| f64::INFINITY, f64::min);

    let max = values
        .par_iter()
        .cloned()
        .reduce(|| f64::NEG_INFINITY, f64::max);

    // Compute median by sorting values locally (single-threaded)
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if values.len() % 2 == 0 {
        let mid = values.len() / 2;
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[values.len() / 2]
    };

    Stats {
        mean,
        std_dev,
        min,
        max,
        median,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gmm::GmpePointKind;

    #[test]
    fn test_compute_stats() {
        let points = vec![
            GmpePoint {
                lon: 0.0,
                lat: 0.0,
                value: 1.0,
                kind: GmpePointKind::Pga,
            },
            GmpePoint {
                lon: 0.0,
                lat: 0.0,
                value: 2.0,
                kind: GmpePointKind::Pga,
            },
            GmpePoint {
                lon: 0.0,
                lat: 0.0,
                value: 3.0,
                kind: GmpePointKind::Pga,
            },
            GmpePoint {
                lon: 0.0,
                lat: 0.0,
                value: 4.0,
                kind: GmpePointKind::Pga,
            },
            GmpePoint {
                lon: 0.0,
                lat: 0.0,
                value: 5.0,
                kind: GmpePointKind::Pga,
            },
        ];

        let stats = compute_stats(&points);

        // Expected values calculated by hand
        let expected = Stats {
            mean: 3.0,
            std_dev: 1.5811388300841898, // sqrt(2.5)
            min: 1.0,
            max: 5.0,
            median: 3.0,
        };

        assert!((stats.mean - expected.mean).abs() < 1e-10);
        assert!((stats.std_dev - expected.std_dev).abs() < 1e-10);
        assert_eq!(stats.min, expected.min);
        assert_eq!(stats.max, expected.max);
        assert_eq!(stats.median, expected.median);
    }
}
