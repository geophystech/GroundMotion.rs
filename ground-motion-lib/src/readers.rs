//! # Data Readers for Site Input Files
//!
//! This module provides utilities for reading site-specific input data into the ground motion
//! prediction library. It focuses on deserializing tabular files into [`Vs30Point`] instances
//! for use in GMPE calculations.
//!
//! ## Features
//!
//! - Load site location and site condition data (longitude, latitude, Vs30, basin depth, and xvf flag).
//! - Support for configurable CSV delimiter characters (e.g., tab, comma).
//! - Assumes no header row in input files.
//!
//! ## Primary Functions
//!
//! - [`read_vs30_points`]: Reads a delimited text file into a vector of [`Vs30Point`] instances.
//!
//! ## Example File Format (tab-delimited)
//!
//! ```text
//! 142.523 52.913  300 250 1
//! 142.600 50.100  350 150 0
//! ```
//!
//! Columns are interpreted as:
//!
//! 1. longitude (f64)
//! 2. latitude (f64)
//! 3. Vs30 (f64)
//! 4. basin depth (optional, f64)
//! 5. xvf flag (optional, u8)
//!
//! ## See Also
//!
//! - [`crate::gmm::Vs30Point`]
//! - [`csv`](https://docs.rs/csv/)
//!
//! ## Errors
//!
//! This module returns boxed errors for I/O issues or data deserialization failures.

use crate::gmm::Vs30Point;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::path::Path;

/// Reads a list of [`Vs30Point`] instances from a delimited text file.
///
/// This function loads site-specific input points for ground motion prediction models from a
/// file. Each line in the file is parsed and deserialized into a [`Vs30Point`] instance, which
/// are collected into a `Vec`.
///
/// The file is assumed to have **no header row**, and the delimiter can be specified to support
/// flexible file formats (e.g., tab, comma, space).
///
/// # Type Parameters
///
/// * `P` — A type convertible to a [`Path`] reference (e.g., `&str`, `PathBuf`).
///
/// # Arguments
///
/// * `path` — Path to the input file.
/// * `delim` — Delimiter character (e.g., `b'\t'` for tab, `b','` for comma).
///
/// # Returns
///
/// A `Result` containing a vector of [`Vs30Point`] instances if successful, or a boxed error
/// if file I/O or parsing fails.
///
/// # Example
///
/// ```rust
/// use ground_motion_lib::readers::read_vs30_points;
///
/// let points = read_vs30_points("tests/data/testvs30.txt", b'\t').unwrap();
/// println!("First point: {:?}", points[0]);
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be opened.
/// - Any row in the file fails to deserialize into a [`Vs30Point`].
pub fn read_vs30_points<P: AsRef<Path>>(
    path: P,
    delim: u8,
) -> Result<Vec<Vs30Point>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(delim)
        .has_headers(false)
        .from_reader(file);

    let mut points = Vec::new();

    for result in rdr.deserialize() {
        let record: Vs30Point = result?;
        points.push(record);
    }

    Ok(points)
}
