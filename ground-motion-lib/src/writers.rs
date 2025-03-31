//! # Data Writers for GMPE Output Files
//!
//! This module provides utilities for writing ground motion prediction results
//! (as [`GmpePoint`] instances) to delimited text files.
//!
//! ## Features
//!
//! - Serialize computed GMPE values for site points into CSV or other delimited formats.
//! - Configurable delimiter support (e.g., tab, comma).
//! - Optionally writes header rows.
//!
//! ## Primary Functions
//!
//! - [`write_gmpe_points`]: Writes a vector of [`GmpePoint`] instances to a delimited file.
//!
//! ## Example Output Format (tab-delimited)
//!
//! ```text
//! lon	lat	value	kind
//! 142.600	50.100	0.789	Pga
//! 142.700	50.200	0.923	Pga
//! ```
//!
//! ## See Also
//!
//! - [`crate::gmm::GmpePoint`]
//! - [`csv`](https://docs.rs/csv/)

use crate::gmm::GmpePoint;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;
use std::path::Path;

/// Writes a list of [`GmpePoint`] instances to a delimited text file.
///
/// This function serializes a list of ground motion prediction results into a file
/// with a configurable delimiter. Each [`GmpePoint`] is written as a CSV row,
/// including a header row describing the columns.
///
/// # Type Parameters
///
/// * `P` — A type convertible to a [`Path`] reference (e.g., `&str`, `PathBuf`).
///
/// # Arguments
///
/// * `path` — The output file path.
/// * `delim` — Delimiter character for the file (e.g., `b','` for comma, `b'\t'` for tab).
/// * `points` — A slice of [`GmpePoint`] instances to write.
///
/// # Returns
///
/// * `Ok(())` if writing was successful.
/// * An error boxed as `Box<dyn Error>` if file I/O or serialization fails.
///
/// # Example
///
/// ```rust
/// use ground_motion_lib::writers::write_gmpe_points;
/// use ground_motion_lib::gmm::{GmpePoint, GmpePointKind};
///
/// let points = vec![
///     GmpePoint { lon: 10.0, lat: 20.0, value: 0.5, kind: GmpePointKind::Pga },
///     GmpePoint { lon: 15.0, lat: 25.0, value: 0.8, kind: GmpePointKind::Pga },
/// ];
///
/// write_gmpe_points("output.csv", b'\t', &points).unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be created or opened.
/// - Any [`GmpePoint`] instance fails to serialize.
pub fn write_gmpe_points<P: AsRef<Path>>(
    path: P,
    delim: u8,
    points: &[GmpePoint],
) -> Result<(), Box<dyn Error>> {
    // Open the file in write mode, create if doesn't exist
    let file = File::create(path)?;

    // Build a CSV writer with the specified delimiter and no headers
    let mut wtr = WriterBuilder::new()
        .delimiter(delim)
        .has_headers(true)
        .from_writer(file);

    // Serialize each GmpePoint struct as a CSV record
    for point in points {
        wtr.serialize(point)?;
    }

    // Ensure all data is flushed to the file
    wtr.flush()?;
    Ok(())
}
